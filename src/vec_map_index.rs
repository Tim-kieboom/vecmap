/// A trait representing a type that can act as an index into a [`VecMap`].
///
/// Implementers of this trait define how to create a new index from a raw `usize`
/// and how to extract its underlying numeric value.
pub trait VecMapIndex {
    /// Constructs a new index from a raw `usize` value.
    fn new_index(value: usize) -> Self;

    /// Returns the numeric representation of this index.
    fn index(&self) -> usize;
}
impl VecMapIndex for usize {
    fn new_index(value: usize) -> Self {
        value
    }

    fn index(&self) -> usize {
        *self
    }
}
