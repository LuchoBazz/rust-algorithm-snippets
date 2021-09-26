#[derive(Debug, Clone, Hash, PartialEq, Eq, PartialOrd, Ord)]
struct WR {
    a: i32,
    b: i32,
}
impl WR {
    fn new(a: i32, b: i32) -> WR {
        WR {a, b}
    }
}