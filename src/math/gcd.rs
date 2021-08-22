fn gcd<T>(a: T, b: T) -> T
where
    T: std::marker::Copy + std::ops::Rem<Output = T> + std::cmp::Eq + std::convert::From<i32>,
{
    if b == T::from(0) {
        a
    } else {
        gcd(b, a % b)
    }
}