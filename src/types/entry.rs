// @TODO: Make an iterator over ListEntry.

/// Link of Doubly-linked list.
#[derive(Debug)]
#[repr(C)]
pub struct ListEntry<T> {
    /// Next
    pub flink: *mut T,
    /// Previous
    pub blink: *mut T,
}

impl<T> Clone for ListEntry<T> {
    fn clone(&self) -> Self {
        Self {
            flink: self.flink,
            blink: self.blink,
        }
    }
}

impl<T> Copy for ListEntry<T> {}