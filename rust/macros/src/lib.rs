#[macro_export]
macro_rules! hashmap {
    ($($($key:expr => $value:expr),+$(,)?)*) => {
        ::std::collections::HashMap::from([$(($key, $value)),*])
    };
}
