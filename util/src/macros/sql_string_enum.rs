#![macro_use]

#[macro_export]
macro_rules! sql_string_enum {
( $enumname: ident {
    $first_enumval: ident
    $(,$enumval: ident)*$(,)*
    } ) => {

        #[derive(Debug, Clone, Copy, Serialize, Deserialize)]
        pub enum $enumname {
            $first_enumval
            $(,$enumval)*
        }

        impl ::diesel::types::FromSqlRow<::diesel::sql_types::Text, ::diesel::pg::Pg> for $enumname {
            fn build_from_row<R: ::diesel::row::Row<::diesel::pg::Pg>>(row: &mut R)
            -> ::std::result::Result<Self, Box<::std::error::Error + Send + Sync>> {
                use std::str::FromStr;
                Ok($enumname::from_str(String::build_from_row(row)?.as_ref())?)
            }
        }

        impl<'a> ::diesel::expression::AsExpression<::diesel::sql_types::Nullable<::diesel::sql_types::Text>> for &'a $enumname {


            type Expression = ::diesel::expression::helper_types::AsExprOf<String, ::diesel::sql_types::Nullable<::diesel::sql_types::Text>>;

            fn as_expression(self) -> Self::Expression {
                ::diesel::expression::AsExpression::<::diesel::sql_types::Nullable<::diesel::sql_types::Text>>::as_expression(self.to_string())
            }
        }

        impl<'a> ::diesel::expression::AsExpression<::diesel::sql_types::Text> for &'a $enumname {


            type Expression = ::diesel::expression::helper_types::AsExprOf<String, ::diesel::sql_types::Text>;

            fn as_expression(self) -> Self::Expression {
                ::diesel::expression::AsExpression::<::diesel::sql_types::Text>::as_expression(self.to_string())
            }
        }

        impl ::std::fmt::Display for $enumname {
            fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                write!(f, "{:?}", self)
            }
        }


        impl ::diesel::Queryable<::diesel::sql_types::Text, ::diesel::pg::Pg> for $enumname {
            type Row = Self;

            fn build(row: Self::Row) -> Self {
                row
            }
        }

        impl ::std::str::FromStr for $enumname {
            type Err = Box<::std::error::Error + Send + Sync>;

            fn from_str(s: &str) -> ::std::result::Result<Self, Self::Err> {
                match s {
                    stringify!($first_enumval) => Ok($enumname::$first_enumval),
                    //FIXME: This is where I need to enum identifiers
                    $( stringify!($enumval) => Ok($enumname::$enumval),)*
                    _ => {
                        let msg = stringify!(Not a valid $enumname value:).to_owned()+ s;
                        Err(msg)
                    },
                }
            }
        }
    }
}
