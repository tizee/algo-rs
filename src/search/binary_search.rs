use std::cmp::Ordering;
use std::cmp::Ordering::*;

// binary search algorithm extracted from slice
// https://github.com/rust-lang/rust/pull/74024/commits/3eb5bee242fae12c4cf547bfe0665653c20ca0c2
pub fn binary_search<'a, T, F>(slice: &'a [T], mut f: F) -> Result<usize, usize>
where
    T: Ord,
    F: FnMut(&'a T) -> Ordering,
{
    let mut size = slice.len();
    let mut left = 0;
    let mut right = size;
    while left < right {
        let mid = left + size / 2;
        let cmp = f(unsafe { slice.get_unchecked(mid) });
        if cmp == Equal {
            return Ok(mid);
        } else if cmp == Greater {
            right = mid;
        } else {
            left = mid + 1;
        }
        size = right - left;
    }
    Err(left)
}

#[test]
fn test_binary_search() {
    let slice = &[1, 2, 3, 4, 5];
    let mut target = 3;
    let mut index = binary_search(slice, |v| v.cmp(&target)).unwrap();
    assert_eq!(index, 2);
    target = 10;
    // should return insertion index
    index = binary_search(slice, |v| v.cmp(&target)).err().unwrap();
    assert_eq!(index, 5);
    target = 0;
    index = binary_search(slice, |v| v.cmp(&target)).err().unwrap();
    assert_eq!(index, 0);
}

#[test]
fn test_binary_search_overflow() {
    let slice = &[(); usize::MAX];
    assert_eq!(binary_search(slice, |_| Equal), Ok(usize::MAX / 2));
    assert_eq!(binary_search(slice, |_| Greater), Err(0));
    assert_eq!(binary_search(slice, |_| Less), Err(usize::MAX));
}
