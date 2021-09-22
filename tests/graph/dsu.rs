// graph/dsu.rs

#[test]
fn dsu_test() {
    let n = 5;
    let mut dsu = DSU::new(n);
    assert_eq!(dsu.comps, 5);
    dsu.unite(0, 1);
    assert_eq!(dsu.find(0), dsu.find(1));
    assert_eq!(dsu.comps, 4);
    dsu.unite(0, 2);
    assert_eq!(dsu.find(0), dsu.find(2));
    assert_eq!(dsu.comps, 3);
    dsu.unite(3, 4);
    assert_eq!(dsu.find(3), dsu.find(4));
    assert_eq!(dsu.comps, 2);
}
