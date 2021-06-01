/// Bubble sort divides slice into unsorted and sorted.
/// Each time it would find a minimal element from unsorted elements and put it at last of sorted
/// elements.
pub fn bubble_sort<T, F>(slice: &mut [T], compare: &mut F)
where
    T: PartialOrd,
    F: FnMut(&T, &T) -> bool,
{
    let mut sorted_len = 0;
    let size = slice.len();
    while sorted_len < size {
        let mut i = size - 1;
        while i > sorted_len {
            if !compare(&slice[i], &slice[i - 1]) {
                slice.swap(i, i - 1);
            }
            i -= 1;
        }
        sorted_len += 1;
    }
}

#[test]
fn test_bubble_sort() {
    use crate::util::temp_node::TempNode;
    let mut s = [6, 5, 4, 3, 2, 1];
    bubble_sort(&mut s, &mut |a, b| a < b);
    assert_eq!(s, [1, 2, 3, 4, 5, 6]);
    let mut s = [2, 1, 4, 3, 6, 5, 8, 7, 10, 9, 12, 11];
    bubble_sort(&mut s, &mut |a, b| a < b);
    assert_eq!(s, [1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12]);
    let mut s = [0];
    bubble_sort(&mut s, &mut |a, b| a < b);
    assert_eq!(s, [0]);
    let mut s = ['c', 'b', 'a'];
    bubble_sort(&mut s, &mut |a, b| a < b);
    assert_eq!(s, ['a', 'b', 'c']);
    let mut s = [1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12];
    bubble_sort(&mut s, &mut |a, b| a < b);
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
    bubble_sort(&mut s, &mut |a, b| a.val <= b.val);
    let keys: Vec<i32> = s.iter().map(|node| node.key).collect();
    let vals: Vec<i32> = s.iter().map(|node| node.val).collect();
    println!("vals = {:?}", vals);
    assert_eq!(keys, [2, 1, 3, 4, 6, 5, 7, 8, 9])
}
