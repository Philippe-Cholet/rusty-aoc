use std::cmp::Ordering;

pub trait Extension<T> {
    fn remove_insert(&mut self, i: usize, j: usize);
}

impl<T> Extension<T> for [T] {
    /// Efficiently remove the element at index `i` and insert back at index `j`.
    ///
    /// Better alternative to
    /// ```ignore
    /// # let mut data = vec![0; 10];
    /// # let (i, j) = (2, 7);
    /// if j != i {
    ///     let elem = data.remove(i);
    ///     data.insert(j, elem);
    /// }
    /// ```
    fn remove_insert(&mut self, i: usize, j: usize) {
        match i.cmp(&j) {
            Ordering::Less => self[i..=j].rotate_left(1),
            Ordering::Greater => self[j..=i].rotate_right(1),
            Ordering::Equal => {}
        }
    }
}
