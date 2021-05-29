// Reference: https://codeforces.com/contest/1526/submission/117622674
fn next_permutation<T: Ord>(v: &mut [T]) -> bool {
    v.windows(2).rposition(|v| v[0] < v[1]).map_or(false, |x| {
        let y = v.iter().rposition(|b| v[x] < *b).unwrap();
        v.swap(x, y);
        v[(x + 1)..].reverse();
        true
    })
}