#![cfg_attr(feature = "cursors", feature(btree_cursors))]

use std::{collections::BTreeMap, ops::Range};

cfg_if::cfg_if! {
    if #[cfg(feature = "cursors")] {
        mod cursor;
    } else {
        mod normal;
    }
}

#[derive(Clone, PartialEq, Eq, Hash, PartialOrd, Ord, Default)]
struct Inner<K, V> {
    end: K,
    value: V,
}

#[derive(Clone, PartialEq, Eq, Hash, PartialOrd, Ord, Default)]
pub struct RangeTree<K, V> {
    tree: BTreeMap<K, Inner<K, V>>,
}

impl<K: Ord + Copy, V: Clone> RangeTree<K, V> {
    pub fn new() -> Self {
        Self {
            tree: BTreeMap::new(),
        }
    }

    pub fn iter(&self) -> impl DoubleEndedIterator<Item = (Range<K>, &V)> + ExactSizeIterator {
        self.tree
            .iter()
            .map(|(&start, &Inner { end, ref value })| (start..end, value))
    }

    pub fn iter_mut(
        &mut self,
    ) -> impl DoubleEndedIterator<Item = (Range<K>, &mut V)> + ExactSizeIterator {
        self.tree
            .iter_mut()
            .map(|(&start, &mut Inner { end, ref mut value })| (start..end, value))
    }
}
