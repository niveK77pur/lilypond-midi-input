/// Create mappings for enum variants and corresponding string representations
#[macro_export]
macro_rules! make_lily_str_map {
    ($(#[$outer:meta])* $name:ident;
     $err:ident::$err_variant:ident;
     $($(#[$inner:meta])* $key:ident, $main:literal $(, $string:literal)*);*;
    ) => {
        #[derive(Debug, Clone, PartialEq, Eq)]
        $(#[$outer])*
        pub enum $name {
            $($(#[$inner])* $key),*
        }

        impl std::str::FromStr for $name {
            type Err = $err;

            fn from_str(s: &str) -> Result<Self, Self::Err> {
                match s {
                    $($main | stringify!($key) $(|$string)* => Ok($name::$key),)*
                    _ => Err($err::$err_variant(s.into())),
                }
            }
        }

        impl std::convert::TryFrom<&str> for $name {
            type Error = $err;

            fn try_from(value: &str) -> Result<Self, Self::Error> {
                <Self as std::str::FromStr>::from_str(value)
            }
        }

        impl TryFrom<$name> for &str {
            type Error = String;

            fn try_from(value: $name) -> Result<Self, Self::Error> {
                match value {
                    $($name::$key => Ok($main)),*
                }
            }
        }

        impl clap::ValueEnum for $name {
            fn value_variants<'a>() -> &'a [Self] {
                &[$($name::$key),*]
            }

            fn to_possible_value(&self) -> Option<clap::builder::PossibleValue> {
                Some(match self {
                    $($name::$key => clap::builder::PossibleValue::new($main).help(stringify!($key))),*
                })
            }
        }
    };
}

#[macro_export]
macro_rules! output {
    ($($arg:tt)*) => { println!($($arg)*) };
}

#[macro_export]
macro_rules! echoinfo {
    ($($arg:tt)*) => { eprintln!(":: {}", format!($($arg)*)) };
}

#[macro_export]
macro_rules! echoerr {
    ($($arg:tt)*) => { eprintln!("!! {}", format!($($arg)*)) }
}
