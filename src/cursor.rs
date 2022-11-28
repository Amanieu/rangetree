use std::{
    mem,
    ops::{Bound, Range},
};

use crate::{Inner, RangeTree};

impl<K: Ord + Copy, V: Clone> RangeTree<K, V> {
    pub fn get(&self, key: K) -> Option<(Range<K>, &V)> {
        // Find the last segment that starts before or at the key.
        let cursor = self.tree.upper_bound(Bound::Included(&key));

        // Check if the segment includes the key.
        let (&start, &Inner { end, ref value }) = cursor.key_value()?;
        (key < end).then_some((start..end, value))
    }

    pub fn insert(&mut self, range: Range<K>, new_value: V) {
        // Ignore empty ranges.
        if range.is_empty() {
            return;
        }

        // Iterate in reverse order: we want to process any entries that begin
        // before the end of the range and that end after the start of the range.
        let mut cursor = self.tree.upper_bound_mut(Bound::Excluded(&range.end));
        while let Some((
            &start,
            &mut Inner {
                ref mut end,
                ref mut value,
            },
        )) = cursor.key_value_mut()
        {
            // This segment is entirely before the start of the range.
            if *end <= range.start {
                // We're done.
                break;
            }

            // This segment is entirely within the range.
            if start > range.start && *end <= range.end {
                // Just remove it.
                cursor.remove_current_and_move_back();
                continue;
            }

            // This segment starts at the same position as the range.
            if start == range.start {
                // Replace the current segment with our new range.
                let old_end = mem::replace(end, range.end);
                let old_value = mem::replace(value, new_value);

                // Create a suffix segment if the old segment extended past the
                // end of the range.
                if *end > range.end {
                    cursor.insert_after_unchecked(
                        range.end,
                        Inner {
                            end: old_end,
                            value: old_value,
                        },
                    );
                }

                // This must be the last segment, and we've already inserted the
                // new range into the tree.
                return;
            }

            // This segment starts before the range.
            if start < range.start {
                // Shrink the segment to only cover the part before the range.
                let old_end = *end;
                *end = range.start;

                // Create a suffix segment if the old segment extended past the
                // end of the range.
                if old_end > range.end {
                    let value = value.clone();
                    cursor.insert_after_unchecked(
                        range.end,
                        Inner {
                            end: old_end,
                            value,
                        },
                    );
                }

                // This must be the last segment, exit the loop to insert the
                // new range between the two parts of the old segment.
                break;
            }

            // Last remaining case: this segment extends past the end of the
            // range. Modify it so that it only covers the suffix.
            *cursor.key_mut_unchecked().unwrap() = range.end;
            cursor.move_prev();
        }

        // We've removed all intersecting segments, now insert our new value.
        // At this point the cursor is pointing to the segment just below the
        // new range.
        cursor.insert_after_unchecked(
            range.start,
            Inner {
                end: range.end,
                value: new_value,
            },
        );
    }

    pub fn remove(&mut self, range: Range<K>) {
        // Iterate in reverse order: we want to process any entries that begin
        // before the end of the range and that end after the start of the range.
        let mut cursor = self.tree.upper_bound_mut(Bound::Excluded(&range.end));
        while let Some((
            &start,
            &mut Inner {
                ref mut end,
                ref value,
            },
        )) = cursor.key_value_mut()
        {
            // This segment is entirely before the start of the range.
            if *end <= range.start {
                // We're done.
                break;
            }

            // This segment is entirely within the range.
            if start > range.start && *end <= range.end {
                // Just remove it.
                cursor.remove_current_and_move_back();
                continue;
            }

            // This segment starts before the range.
            if start < range.start {
                // Shrink the segment to only cover the part before the range.
                let old_end = *end;
                *end = range.start;

                // Create a suffix segment if the old segment extended past the
                // end of the range.
                if old_end > range.end {
                    let value = value.clone();
                    cursor.insert_after_unchecked(
                        range.end,
                        Inner {
                            end: old_end,
                            value,
                        },
                    );
                }

                // This must be the last segment, exit the loop.
                break;
            }

            // Last remaining case: this segment extends past the end of the
            // range. Modify it so that it only covers the suffix.
            *cursor.key_mut_unchecked().unwrap() = range.end;

            // Optimization: avoid looking at the next entry if we know this is
            // the last one.
            if start <= range.start {
                break;
            } else {
                cursor.move_prev();
            }
        }
    }
}
