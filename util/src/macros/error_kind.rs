#![macro_use]

#[macro_export]
macro_rules! error_from_unhandled {
    ($error:ident, $error_kind:ident, $from:path) => {
        impl From<$from> for $error {
            fn from(err: $from) -> $error {
                use failure::Fail;
                $error {
                    inner: err.context($error_kind::Internal),
                }
            }
        }
    };
}

#[macro_export]
macro_rules! error_kind {
    ( $error: ident, $error_kind: ident $(,$from: path)* ) => {

        #[derive(Debug)]
        pub struct $error {
            inner: ::failure::Context<$error_kind>,
        }



        impl ::failure::Fail for $error {
            fn cause(&self) -> Option<&::failure::Fail> {
                self.inner.cause()
            }

            fn backtrace(&self) -> Option<&::failure::Backtrace> {
                self.inner.backtrace()
            }
        }

        impl ::std::fmt::Display for $error {
            fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                ::std::fmt::Display::fmt(&self.inner, f)
            }
        }


        impl $error {
            pub fn map_to<T: Into<::failure::Error>>(error_kind: $error_kind) -> impl Fn(T) -> $error {
                move |err| $error { inner: err.into().context(error_kind) }
            }
        }



        impl ::serde::Serialize for $error {
            fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
                where
                    S: ::serde::Serializer,
            {
                use ::serde::ser::SerializeStruct;

                let mut state = serializer.serialize_struct(stringify!($error), 2)?;
                state.serialize_field("error", stringify!($error))?;
                state.serialize_field("kind", &self.kind())?;
                state.end()
            }
        }


        impl From<$error_kind> for $error {
            fn from(kind: $error_kind) -> $error {
                $error { inner: ::failure::Context::new(kind) }
            }
        }

        impl From<::failure::Context<$error_kind>> for $error {
            fn from(inner: ::failure::Context<$error_kind>) -> $error {
                $error { inner: inner }
            }
        }


        impl ::util::error::Error for $error {

            type Kind = $error_kind;

            fn is_internal_err(&self) -> bool {
                self.kind() == $error_kind::Internal
            }

            fn to_internal_err<T>(err:T) -> Self where T: Into<::failure::Error> {
                Self { inner: err.into().context($error_kind::Internal) }
            }

            fn kind(&self) -> $error_kind {
                *self.inner.get_context()
            }
        }


        $(error_from_unhandled!($error,$error_kind,$from);)*

    }
}
