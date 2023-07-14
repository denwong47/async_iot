pub trait FromWithKey<T> {
    fn from_with_key(key: &str, value: T) -> Self;
}
