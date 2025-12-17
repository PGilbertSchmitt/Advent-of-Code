use std::{cmp::Ordering, collections::BinaryHeap};

// Borrowed from the https://github.com/maneatingape/advent-of-code-rust/blob/main/src/util/heap.rs
// That repo runs all AofC problems from 2015 to 2025 in under 1 second, so I trust the implementation

struct Wrapper<K: Ord, V> {
    key: K,
    value: V,
}

impl<K: Ord, V> PartialEq for Wrapper<K, V> {
    #[inline]
    fn eq(&self, other: &Self) -> bool {
        self.key == other.key
    }
}

impl<K: Ord, V> Eq for Wrapper<K, V> {}

impl<K: Ord, V> PartialOrd for Wrapper<K, V> {
    #[inline]
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl<K: Ord, V> Ord for Wrapper<K, V> {
    #[inline]
    fn cmp(&self, other: &Self) -> Ordering {
        // Reversed for min-heap rather than max-heap
        other.key.cmp(&self.key)
    }
}

#[derive(Default)]
pub struct MinHeap<K: Ord, V>(BinaryHeap<Wrapper<K, V>>);

impl<K: Ord, V> MinHeap<K, V> {
    pub fn with_capacity(capacity: usize) -> Self {
        MinHeap(BinaryHeap::with_capacity(capacity))
    }

    pub fn push(&mut self, key: K, value: V) {
        self.0.push(Wrapper { key, value });
    }

    pub fn pop(&mut self) -> Option<(K, V)> {
        self.0.pop().map(|wrapper| (wrapper.key, wrapper.value))
    }

    pub fn peak(&self) -> Option<(&K, &V)> {
        self.0.peek().map(|wrapper| (&wrapper.key, &wrapper.value))
    }
}
