fn fastpow<T, U>(base: T, exp: U) -> T
where
    T: std::marker::Copy
        + std::ops::Mul<Output = T>
        + std::convert::From<i32>,
    U: std::marker::Copy
        + std::ops::BitAnd<Output = U>
        + std::ops::ShrAssign<U>
        + std::cmp::PartialOrd<U>
        + std::convert::From<i32>,
{
    assert!(U::from(0) <= exp);
    let mut base = base;
    let mut exp = exp;
    let mut ans: T = T::from(1);
    while (exp > U::from(0)) {
        if ((exp & U::from(1)) > U::from(0)) {
            ans = ans * base;
        }
        base = base * base;
        exp >>= U::from(1);
    }
    return ans;
}