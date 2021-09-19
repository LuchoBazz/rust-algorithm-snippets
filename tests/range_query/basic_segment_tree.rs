// range_query/basic_segment_tree.rs

#[test]
fn range_query_basic_segment_tree_add() {
    let mut a = vec![3, 2, 4, 5, 1, 1, 5, 3];
    let mut seg = SegTree::new(&a, 0, |a, b| *a + *b);
    assert_eq!(14, seg.query(0, 3));
    assert_eq!(2, seg.query(4, 5));
    seg.modify(2, 1);
    assert_eq!(11, seg.query(0, 3));
}

#[test]
fn range_query_basic_segment_tree_min() {
    let mut a = vec![3, 2, 4, 5, 1, 1, 5, 3];
    let mut seg = SegTree::new(&a, 1e18 as i64, |a, b| std::cmp::min(*a, *b));
    assert_eq!(2, seg.query(0, 3));
    assert_eq!(1, seg.query(4, 5));
    seg.modify(2, 1);
    assert_eq!(1, seg.query(0, 3));
}

#[test]
fn range_query_basic_segment_tree_max() {
    let mut a = vec![3, 2, 4, 5, 1, 1, 5, 3];
    let mut seg = SegTree::new(&a, 1e18 as i64, |a, b| std::cmp::max(*a, *b));
    assert_eq!(5, seg.query(0, 3));
    assert_eq!(1, seg.query(4, 5));
    seg.modify(2, 100);
    assert_eq!(100, seg.query(0, 3));
}