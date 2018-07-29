#![feature(rust_2018_preview)]
#![deny(rust_2018_compatibility)]
#![deny(rust_2018_idioms)]
#![recursion_limit="1024"]

use proc_macro::{TokenStream};
use quote::{quote,quote_each_token, quote_spanned};
use syn::*;
use proc_macro2::{Ident, Span};

#[proc_macro_derive(CustomError)]
pub fn custom_error_derive(input: TokenStream) -> TokenStream {

    //let input = input.to_string();
    let ast: DeriveInput = syn::parse(input).unwrap();

    let enum_name = ast.ident.to_string();
    let (error_name, _) = enum_name.split_at(enum_name.len()-4); //Remove Kind
    let error_type = Ident::new(error_name, Span::call_site());

    let quoted = if let Data::Enum(ref _variants) = ast.data {
        generate_error_type(&ast.ident, &error_type)
    } else {
        panic!("#derive(CustomError) can only be applied to enums")
    };

    TokenStream::from(quoted)

}


fn generate_error_type(
    enum_name: &Ident,
    error_type: &Ident
) -> proc_macro2::TokenStream {

    let modname = Ident::new(&format!("error_impl_for_{}", enum_name), Span::call_site());

    quote! {
        pub use self::#modname::#error_type;
        #[allow(non_snake_case)]
        mod #modname {

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
        }
    }

}