
#[macro_export]
macro_rules! pub_use_mod {
    ( $mod_name:ident ) => {
        pub mod $mod_name;
        pub use $mod_name::*;
    };

    ( $( $mod_name:ident ),* $(,)* ) => {
        $( pub_use_mod!($mod_name); )*
    };
}

macro_rules! tag {
    ($val:expr) => {
        
    };
}

macro_rules! meta {
    ($prop:ident, $cont:ident) => {
        tag!();
    };
}