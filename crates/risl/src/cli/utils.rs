/// Utility macro to create Vec<String> from &str slices
macro_rules! str_vec {
    () => (
        $crate::vec::Vec::<String>::new()
    );
    ($($x:expr),+ $(,)?) => (
        [$($x),+].map(String::from).to_vec()
    );
}
pub(crate) use str_vec;
