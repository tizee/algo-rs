/// In selection sort algorithm, it finds the minimal element each time from the unsorted elements and insert it into sorted part. Unlike bubble sort, insertion of such minimal element only requires one swapping operation.
pub fn selection_sort<T, F>(slice: &mut [T], compare: &mut F)
where
    T: PartialOrd,
    F: FnMut(&T, &T) -> bool,
{
    let mut sorted_len = 0;
    let size = slice.len();
    while sorted_len < size {
        let mut minimal = &slice[sorted_len];
        let mut i = sorted_len;
        let mut j = sorted_len; // index for insertion
        while i < size {
            if !compare(minimal, &slice[i]) {
                minimal = &slice[i];
                j = i;
            }
            i += 1;
        }
        slice.swap(sorted_len, j);
        sorted_len += 1;
    }
}

#[test]
fn test_selection_sort() {
    use crate::util::temp_node::TempNode;
    let mut s = [6, 5, 4, 3, 2, 1];
    selection_sort(&mut s, &mut |a, b| a < b);
    assert_eq!(s, [1, 2, 3, 4, 5, 6]);
    let mut s = [2, 1, 4, 3, 6, 5, 8, 7, 10, 9, 12, 11];
    selection_sort(&mut s, &mut |a, b| a < b);
    assert_eq!(s, [1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12]);
    let mut s = [0];
    selection_sort(&mut s, &mut |a, b| a < b);
    assert_eq!(s, [0]);
    let mut s = ['c', 'b', 'a'];
    selection_sort(&mut s, &mut |a, b| a < b);
    assert_eq!(s, ['a', 'b', 'c']);
    let mut s = [1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12];
    selection_sort(&mut s, &mut |a, b| a < b);
    assert_eq!(s, [1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12]);
    // stable
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
    selection_sort(&mut s, &mut |a, b| a.val <= b.val);
    let keys: Vec<i32> = s.iter().map(|node| node.key).collect();
    let vals: Vec<i32> = s.iter().map(|node| node.val).collect();
    println!("vals = {:?}", vals);
    assert_eq!(keys, [2, 1, 3, 4, 6, 5, 7, 8, 9])
}
