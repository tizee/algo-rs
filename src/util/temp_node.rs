use std::cmp::Ordering;
// stable sorting verification
pub struct TempNode {
    pub val: i32,
    pub key: i32,
}

impl Ord for TempNode {
    fn cmp(&self, other: &Self) -> Ordering {
        return self.val.cmp(&other.val);
    }
}

impl PartialOrd for TempNode {
    fn partial_cmp(&self, other: &TempNode) -> Option<Ordering> {
        return Some(self.val.cmp(&other.val));
    }
}

impl Eq for TempNode {}

impl PartialEq for TempNode {
    fn eq(&self, other: &TempNode) -> bool {
        return self.val == other.val;
    }
}

// Copy
impl Copy for TempNode {}

impl Clone for TempNode {
    fn clone(&self) -> Self {
        *self
    }
}
