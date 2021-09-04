// math_gcd

fn lcm<T>(a: T, b: T) -> T
where
    T: std::marker::Copy
        + std::ops::Mul<Output = T>
        + std::ops::Div<Output = T>
        + std::ops::Rem<Output = T>
        + std::cmp::Eq
        + std::convert::From<i32>,
{
    return (a * b) / gcd(a, b);
}