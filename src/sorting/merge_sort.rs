/// Merge sort is a classic sorting algorithm using divde-and-conqure strategy. By dividing the slice into smaller slice until there is only one element left. Then each subset is merged according to compare fn with a auxiliary vector, which implies that it is not a in-place sorting algorithm.
pub fn merge_sort<T, F>(slice: &mut [T], compare: &mut F)
where
    T: Copy + PartialOrd,
    F: FnMut(&T, &T) -> bool,
{
    let size = slice.len();
    if size <= 1 {
        return;
    }
    let end = size / 2;
    merge_sort(&mut slice[..end], compare);
    merge_sort(&mut slice[end..], compare);
    let mut temp = slice.to_vec();
    merge(&slice[..end], &slice[end..], &mut temp[..], compare);
    // merge
    slice.copy_from_slice(&temp);
}

fn merge<T, F>(a: &[T], b: &[T], temp: &mut [T], compare: &mut F)
where
    T: Copy + PartialOrd,
    F: FnMut(&T, &T) -> bool,
{
    let mut i = 0;
    let mut j = 0;
    let mut cur = 0;
    while i < a.len() && j < b.len() {
        if compare(&a[i], &b[j]) {
            temp[cur] = a[i];
            cur += 1;
            i += 1;
        } else {
            temp[cur] = b[j];
            cur += 1;
            j += 1;
        }
    }
    if i < a.len() {
        temp[cur..].copy_from_slice(&a[i..])
    }
    if j < b.len() {
        temp[cur..].copy_from_slice(&b[j..])
    }
}

#[test]
fn test_merge_sort() {
    use crate::util::temp_node::TempNode;
    let mut s = [6, 5, 4, 3, 2, 1];
    merge_sort(&mut s, &mut |a, b| a < b);
    assert_eq!(s, [1, 2, 3, 4, 5, 6]);
    let mut s = [2, 1, 4, 3, 6, 5, 8, 7, 10, 9, 12, 11];
    merge_sort(&mut s, &mut |a, b| a < b);
    assert_eq!(s, [1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12]);
    let mut s = [0];
    merge_sort(&mut s, &mut |a, b| a < b);
    assert_eq!(s, [0]);
    let mut s = ['c', 'b', 'a'];
    merge_sort(&mut s, &mut |a, b| a < b);
    assert_eq!(s, ['a', 'b', 'c']);
    let mut s = [1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12];
    merge_sort(&mut s, &mut |a, b| a < b);
    assert_eq!(s, [1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12]);
    let mut s = [
        TempNode { val: 2, key: 3 },
        TempNode { val: 3, key: 4 },
        TempNode { val: 1, key: 2 },
        TempNode { val: 5, key: 6 },
        TempNode { val: 1, key: 1 },
        TempNode { val: 9, key: 9 },
        TempNode { val: 5, key: 5 },
        TempNode { val: 6, key: 8 },
        TempNode { val: 5, key: 7 },
    ];
    merge_sort(&mut s, &mut |a, b| a.val <= b.val);
    let keys: Vec<i32> = s.iter().map(|node| node.key).collect();
    let vals: Vec<i32> = s.iter().map(|node| node.val).collect();
    println!("vals = {:?}", vals);
    assert_eq!(keys, [2, 1, 3, 4, 6, 5, 7, 8, 9])
}
