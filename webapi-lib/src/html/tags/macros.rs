macro_rules! tag_setter {
    ( $fn_name:ident ( $( $arg:ident : $val:ty )* $(,)* )) => {
        pub fn $fn_name(mut self, $($arg: $val)*) -> Self {
            $( self.attrs.insert("$fn_name".into(), $arg.to_string()) )*;
            self
        }
    };
}

macro_rules! tag_setter_impl {
    ( $impl_name:ident {
        $( $fn_name:ident ( $( $arg:ident : $val:ty ),* $(,)* ) $(,)* )*
    } ) => {
        impl $impl_name {
            $( pub fn $fn_name(mut self, $($arg: $val),*) -> Self {
                $( self.attrs.insert("$fn_name".into(), $arg.to_string()); )*
                self
            })*
        }
    };
}