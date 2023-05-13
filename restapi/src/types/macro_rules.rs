#[macro_export]
macro_rules! impl_enum_as_string {
    ($enum_name:ident { $($variant:ident),+ $(,)? }) => {
        impl $enum_name {
            #[allow(dead_code)]
            pub fn as_string(&self) -> &str {
                match self {
                    $( $enum_name::$variant => stringify!($variant) ),+
                }
            }
        }
    };
}
#[macro_export]
macro_rules! sort_by_field {
    ($field:ident, $a:expr, $b:expr) => {
        $a.$field.cmp(&$b.$field)
    };
}