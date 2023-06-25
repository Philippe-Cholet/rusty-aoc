use std::cmp::{Ordering, Reverse};

/// Arbitrary item that can be compared by its heuristic.
///
/// Meant to be used in a priority queue, where only the heuristic matter.
///
/// `std::collections::BinaryHeap` being a **max**-heap,
/// the method "new" is meant for elements of **max**-heaps
/// while the method "rev" is meant for elements of **min**-heaps.
#[derive(Debug)]
pub struct HeuristicItem<H, T> {
    pub heuristic: H,
    pub item: T,
}

impl<H, T> HeuristicItem<H, T> {
    /// New instance.
    pub const fn new(heuristic: H, item: T) -> Self {
        Self { heuristic, item }
    }
}

impl<H, T> HeuristicItem<Reverse<H>, T> {
    /// New instance, wrapping the heuristic in `std::cmp::Reverse`.
    pub const fn rev(heuristic: H, item: T) -> Self {
        Self::new(Reverse(heuristic), item)
    }
}

impl<H: PartialEq, T> PartialEq for HeuristicItem<H, T> {
    fn eq(&self, other: &Self) -> bool {
        self.heuristic == other.heuristic
    }
}

impl<H: PartialEq, T> Eq for HeuristicItem<H, T> {}

impl<H: PartialOrd, T> PartialOrd for HeuristicItem<H, T> {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        self.heuristic.partial_cmp(&other.heuristic)
    }
}

impl<H: Ord, T> Ord for HeuristicItem<H, T> {
    fn cmp(&self, other: &Self) -> Ordering {
        self.heuristic.cmp(&other.heuristic)
    }
}
