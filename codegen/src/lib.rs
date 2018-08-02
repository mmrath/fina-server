#![feature(rust_2018_preview)]
#![deny(rust_2018_compatibility)]
#![deny(rust_2018_idioms)]
#![recursion_limit="1024"]

use quote::{quote,quote_each_token, quote_spanned, pounded_var_names, multi_zip_expr, nested_tuples_pat};
use syn::*;
use syn::Meta::{List, NameValue, Word};
use syn::NestedMeta::{Literal, Meta};
use proc_macro2::{TokenStream, Ident, Span};
use synstructure::{Structure,decl_derive};

decl_derive!([CustomError, attributes(custom_error)] => custom_error_derive);

fn custom_error_derive(s: Structure<'_>) -> TokenStream {

    //let input = input.to_string();
    let ast: &DeriveInput = s.ast();

    let enum_name = ast.ident.to_string();
    let (error_name, _) = enum_name.split_at(enum_name.len()-4); //Remove Kind
    let error_type = Ident::new(error_name, Span::call_site());

    let main_impl = if let Data::Enum(ref data_enum) = ast.data {
        generate_error_type(&ast.ident, &error_type, data_enum)
    } else {
        panic!("#derive(CustomError) can only be applied to enums")
    };

    let mut impls = if let Data::Enum(ref data_enum) = ast.data {
        let mut impls = Vec::new();
        let res = data_enum.variants.iter().map(
            |v|process_variant(&ast.ident,&error_type,v)
        ).fold(impls, |mut sum,mut u|{sum.append(&mut u); sum});
        res

    } else {
        panic!("#derive(CustomError) can only be applied to enums")
    };



    let modname = Ident::new(&format!("error_impl_for_{}", enum_name), Span::call_site());


    (quote! {
        pub use self::#modname::#error_type;
        #[allow(non_snake_case)]
        mod #modname {

           #main_impl
           #( #impls )*
        }
    }).into()

}

fn process_variant(enum_name: &Ident,
                   error_type: &Ident,
                   variant: &syn::Variant)->Vec<TokenStream>{

    use std::iter::FromIterator;

    let mut impls: Vec<TokenStream> = Vec::new();
    let enum_variant = &variant.ident;
    for meta_items in variant.attrs.iter().filter_map(get_custom_error_meta_items) {
        for meta_item in meta_items {
            match meta_item {
                // Parse `#[serde(bound(serialize = "D: Serialize", deserialize = "D: Deserialize"))]`
                Meta(List(ref m)) if m.ident == "map_from" => {
                    for meta in &m.nested {
                        match *meta {
                            Meta(Word(ref word)) => {
                                let q =quote!{
                                    impl From<#word> for #error_type {
                                        fn from(err: #word) -> #error_type {
                                            use failure::Fail;
                                            #error_type::map_to(#enum_name::#enum_variant)(err)
                                        }
                                    }

                                };
                                impls.push(TokenStream::from(q));
                            },
                            _ =>
                                panic!("Only error type expected")
                        }
                    }
                },
                _ => {
                    panic!("Only map_from currently supported")
                }
            }
        }
    }
    impls


}

fn get_custom_error_meta_items(attr: &syn::Attribute) -> Option<Vec<syn::NestedMeta>> {

    if attr.path.segments.len() == 1 && attr.path.segments[0].ident == "custom_error" {
        match attr.interpret_meta() {
            Some(List(ref meta)) => Some(meta.nested.iter().cloned().collect()),
            _ => {
                // TODO: produce an error
                None
            }
        }
    } else {
        None
    }
}

fn generate_error_type(enum_name: &Ident, error_type: &Ident,
                       data_enum: &DataEnum, ) -> TokenStream {

    use std::iter::FromIterator;

    let modname = Ident::new(&format!("error_impl_for_{}", enum_name), Span::call_site());




    let main_impl = quote! {


            use super::*;
            use failure::{Fail,Backtrace,Context,Error};
            use std::fmt;
            use fina_util::error::Error as UtilError;

            #[derive(Debug)]
            pub struct #error_type {
                inner: Context<#enum_name>,
            }

            impl Fail for #error_type {
                fn cause(&self) -> Option<&Fail> {
                    self.inner.cause()
                }

                fn backtrace(&self) -> Option<&Backtrace> {
                    self.inner.backtrace()
                }
            }

            impl fmt::Display for #error_type {
                fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
                    fmt::Display::fmt(&self.inner, f)
                }
            }


            impl #error_type {
                pub fn map_to<T: Into<Error>>(error_kind: #enum_name) -> impl Fn(T) -> #error_type {
                    move |err| #error_type { inner: err.into().context(error_kind) }
                }
            }



            impl ::serde::Serialize for #error_type {
                fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
                    where
                        S: ::serde::Serializer,
                {
                    use ::serde::ser::SerializeStruct;

                    let mut state = serializer.serialize_struct(stringify!(#error_type), 2)?;
                    state.serialize_field("error", stringify!(#error_type))?;
                    state.serialize_field("kind", &self.kind())?;
                    state.end()
                }
            }


            impl From<#enum_name> for #error_type {
                fn from(kind: #enum_name) -> #error_type {
                    #error_type { inner: Context::new(kind) }
                }
            }

            impl From<Context<#enum_name>> for #error_type {
                fn from(inner: Context<#enum_name>) -> #error_type {
                    #error_type { inner: inner }
                }
            }


            impl UtilError for #error_type {

                type Kind = #enum_name;

                fn is_internal_err(&self) -> bool {
                    self.kind() == #enum_name::Internal
                }

                fn to_internal_err<T>(err:T) -> Self where T: Into<Error> {
                    Self { inner: err.into().context(#enum_name::Internal) }
                }

                fn kind(&self) -> #enum_name {
                    *self.inner.get_context()
                }
            }




    };
    main_impl

}