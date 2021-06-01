// inplace sorting
pub fn quick_sort<T, F>(slice: &mut [T], compare: &mut F)
where
    T: PartialOrd,
    F: FnMut(&T, &T) -> bool,
{
    // find the pivot for partition the slice into two parts of items smaller than
    // pivot and those larger than pivot.
    // We would get [smaller] pivot [larger] after one pass
    // When applying the f for comparison we would get the order we need.
    let size = slice.len();
    if size <= 1 {
        return;
    }
    // Because the pivot is extremely important to the partition phase,
    // we need to use a smarter strategy to find a suitable one that
    // help reduce the number of comparison. Here we just use the middle one for simplicity.
    let pivot = partition(slice, size / 2, compare);
    quick_sort(&mut slice[..pivot], compare);
    quick_sort(&mut slice[pivot + 1..], compare);
}

fn partition<T, F>(slice: &mut [T], pivot: usize, is_less: &mut F) -> usize
where
    T: PartialOrd,
    F: FnMut(&T, &T) -> bool,
{
    let mut left = 0;
    let end = slice.len() - 1;
    let mut right = end - 1;
    slice.swap(end, pivot);
    // find the first pair of elements out of order from left to right and from right to left
    // get_unchecked is unsafe
    unsafe {
        loop {
            while left < end && is_less(slice.get_unchecked(left), &slice[end]) {
                left += 1;
            }
            while left < right && !is_less(slice.get_unchecked(right), &slice[end]) {
                right -= 1;
            }
            if left >= right {
                break;
            }
            slice.swap(left, right);
        }
    }
    // place back pivot between two partitions
    slice.swap(end, left);
    return left;
}

#[test]
fn test_quick_sort() {
    use crate::util::temp_node::TempNode;
    let mut s = [6, 5, 4, 3, 2, 1];
    quick_sort(&mut s, &mut |a, b| a < b);
    assert_eq!(s, [1, 2, 3, 4, 5, 6]);
    let mut s = [2, 1, 4, 3, 6, 5, 8, 7, 10, 9, 12, 11];
    quick_sort(&mut s, &mut |a, b| a < b);
    assert_eq!(s, [1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12]);
    let mut s = [0];
    quick_sort(&mut s, &mut |a, b| a < b);
    assert_eq!(s, [0]);
    let mut s = ['c', 'b', 'a'];
    quick_sort(&mut s, &mut |a, b| a < b);
    assert_eq!(s, ['a', 'b', 'c']);
    let mut s = [1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12];
    quick_sort(&mut s, &mut |a, b| a < b);
    assert_eq!(s, [1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12]);

    // unstable
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
    quick_sort(&mut s, &mut |a, b| a.val <= b.val);
    let keys: Vec<i32> = s.iter().map(|node| node.key).collect();
    let vals: Vec<i32> = s.iter().map(|node| node.val).collect();
    println!("vals = {:?}", vals);
    assert_ne!(keys, [2, 1, 3, 4, 6, 5, 7, 8, 9])
}
