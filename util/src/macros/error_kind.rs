#![macro_use]

#[macro_export]
macro_rules! error_from_unhandled {
    ( $error: ident, $error_kind: ident, $from: path ) => {
        impl From<$from> for $error{
            fn from(err: $from) -> $error {
                use failure::Fail;
                $error { inner: err.context($error_kind::Internal) }
            }
        }
    }
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
            pub fn kind(&self) -> $error_kind {
                *self.inner.get_context()
            }

            pub fn map_to<T: Into<::failure::Error>>(error_kind: $error_kind) -> impl Fn(T) -> $error {
                move |err| $error { inner: err.into().context(error_kind) }
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

        $(error_from_unhandled!($error,$error_kind,$from);)*

    }
}

