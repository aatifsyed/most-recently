/// Extension trait for [`String`], allowing it to be use as a `&'static str`
pub trait IntoStaticStr {
    fn into_static_str(self) -> &'static str;
}

impl IntoStaticStr for String {
    fn into_static_str(self) -> &'static str {
        Box::leak(self.into_boxed_str())
    }
}

/// Prepend 'no_' to a constant argument
#[macro_export]
macro_rules! no {
    ($ex:expr) => {{
        use const_format::concatcp;
        concatcp!("no_", $ex)
    }};
}
