pub trait IsFloat {}
impl IsFloat for f32 {}
impl IsFloat for f64 {}

fn cmp<T>(a: T, b: T, eps: T) -> std::cmp::Ordering
where
    T: IsFloat + std::ops::Add<Output = T> + std::cmp::PartialOrd + Copy,
{
    if a + eps < b {
        std::cmp::Ordering::Less
    } else {
        if b + eps < a {
            std::cmp::Ordering::Greater
        } else {
            std::cmp::Ordering::Equal
        }
    }
}
// let eps: f64 = 1e-9;
// cmp(a, b, eps);