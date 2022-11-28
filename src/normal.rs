use crate::{Inner, RangeTree};
use smallvec::SmallVec;
use std::{mem, ops::Range};

impl<K: Ord + Copy, V: Clone> RangeTree<K, V> {
    pub fn insert(&mut self, range: Range<K>, new_value: V) {
        // Ignore empty ranges.
        if range.is_empty() {
            return;
        }

        // We can't remove entries while iterating.
        let mut remove = SmallVec::<[_; 8]>::new();
        let mut insert = None;
        let mut value_to_insert = Some(new_value);

        // Iterate in reverse order: we want to process any entries that begin
        // before the end of the range and that end after the start of the range.
        for (
            &start,
            &mut Inner {
                ref mut end,
                ref mut value,
            },
        ) in self
            .tree
            .range_mut(..range.end)
            .rev()
            .take_while(|(_, inner)| inner.end > range.start)
        {
            // This segment is entirely within the range.
            if start > range.start && *end <= range.end {
                // Just remove it.
                remove.push(start);
                continue;
            }

            // This segment starts before or at the range.
            if start <= range.start {
                let old_end = *end;
                let mut old_value = None;
                if start == range.start {
                    // Optimization: re-use this segment for the one we are
                    // about to insert.
                    *end = range.end;
                    old_value = Some(mem::replace(value, value_to_insert.take().unwrap()));
                } else {
                    // Shrink the segment.
                    *end = range.start;
                }

                // Create a suffix segment if the old segment extended past the
                // end of the range.
                if old_end > range.end {
                    // TODO: cloning
                    let value = value.clone();
                    self.tree.insert(
                        range.end,
                        Inner {
                            end: old_end,
                            value: old_value.unwrap_or_else(|| value.clone()),
                        },
                    );
                }

                // This must be the last segment, stop here to make the borrow
                // checker happy.
                break;
            }

            // Last remaining case: this segment extends past the end of the
            // range. Replace it with a suffix segment (to be inserted after the
            // loop ends due to the borrow checker).
            debug_assert!(insert.is_none());
            insert = Some((range.end, *end, value.clone()));
            remove.push(start);
        }

        for key in &remove {
            self.tree.remove(key);
        }
        if let Some((start, end, value)) = insert {
            self.tree.insert(start, Inner { end, value });
        }
        if let Some(new_value) = value_to_insert {
            self.tree.insert(
                range.start,
                Inner {
                    end: range.end,
                    value: new_value,
                },
            );
        }
    }

    pub fn remove(&mut self, range: Range<K>) {
        // We can't remove entries while iterating.
        let mut remove = SmallVec::<[_; 8]>::new();
        let mut insert = None;

        // Iterate in reverse order: we want to process any entries that begin
        // before the end of the range and that end after the start of the range.
        for (
            &start,
            &mut Inner {
                ref mut end,
                ref mut value,
            },
        ) in self
            .tree
            .range_mut(..range.end)
            .rev()
            .take_while(|(_, inner)| inner.end > range.start)
        {
            // This segment is entirely within the range.
            if start > range.start && *end <= range.end {
                // Just remove it.
                remove.push(start);
                continue;
            }

            // This segment starts before or at the range.
            if start <= range.start {
                // Shrink/delete the segment.
                let old_end = *end;
                if start == range.start {
                    remove.push(start);
                } else {
                    *end = range.start;
                }

                // Create a suffix segment if the old segment extended past the
                // end of the range.
                if old_end > range.end {
                    let value = value.clone();
                    self.tree.insert(
                        range.end,
                        Inner {
                            end: old_end,
                            value,
                        },
                    );
                }

                // This must be the last segment, stop here to make the borrow
                // checker happy.
                break;
            }

            // Last remaining case: this segment extends past the end of the
            // range. Create a suffix segment (to be inserted after the loop
            // ends due to the borrow checker).
            debug_assert!(insert.is_none());
            insert = Some((range.end, *end, value.clone()));
            remove.push(start);
        }

        for key in &remove {
            self.tree.remove(key);
        }
        if let Some((start, end, value)) = insert {
            self.tree.insert(start, Inner { end, value });
        }
    }
}
