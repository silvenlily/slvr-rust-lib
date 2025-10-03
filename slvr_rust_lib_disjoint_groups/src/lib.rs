#![feature(slice_ptr_get)]

#[cfg(test)]
mod test;

use std::error::Error;
use std::fmt::{Display, Formatter};
use std::ops::Range;

type GroupRanges = Vec<Range<usize>>;
type Groups = Vec<GroupRanges>;

#[derive(Debug)]
pub enum GetManyDisjointGroupedError {
    OutOfBounds,
    NotDisjoint,
}

impl Display for GetManyDisjointGroupedError {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            GetManyDisjointGroupedError::OutOfBounds => {
                f.write_str("At least one group exceeds the bounds of the slice")
            }
            GetManyDisjointGroupedError::NotDisjoint => {
                f.write_str("At least one group overlaps another group")
            }
        }
    }
}

impl Error for GetManyDisjointGroupedError {}

/// Allows borrowing groups of ranges at the same time from an underlying data structure
///
/// # UNSTABLE
/// Requires the `slice_ptr_get` unstable nightly feature to be enabled and
/// therefore is unstable and cannot be stabilized until `slice_ptr_get` is stabilized
pub trait GetManyDisjointGrouped<Item> {
    /// gets the bounds of the underlying data store
    fn bounds(&self) -> Range<usize>;

    /// checks that all groups are disjoint and inside the bounds of self
    ///
    /// performs the following checks in order:
    ///
    /// returns `Err(GetManyDisjointGroupedError::Empty)` if all groups are empty
    ///
    /// returns `Err(GetManyDisjointGroupedError::NotDisjoint)` if any group overlaps any other group
    ///
    /// returns `Err(GetManyDisjointGroupedError::OutOfBounds)` if the bounds of `groups` exceeds the bounds of `self`
    ///
    /// returns `Ok(())` all the above passes
    ///
    /// # UNSTABLE
    /// Requires the `slice_ptr_get` unstable nightly feature to be enabled and
    /// therefore is unstable and cannot be stabilized until `slice_ptr_get` is stabilized
    fn check_valid_disjoint_groups(
        &self,
        groups: &Groups,
    ) -> Result<(), GetManyDisjointGroupedError> {
        let mut values: Vec<&Range<usize>> = groups
            .iter()
            .flatten()
            .filter(|r| r.start < r.end)
            .collect();

        values.sort_by_key(|a| a.start);

        // getting no values is stupid but safe
        if values.is_empty() {
            return Ok(());
        }

        // check for overlap
        for pair in values.windows(2) {
            if pair[0].end > pair[1].start {
                return Err(GetManyDisjointGroupedError::NotDisjoint);
            }
        }

        // get the bounds of the groups
        let start = values.first().unwrap().start;
        let end = values.last().unwrap().end;

        let groups_bounds = Range { start, end };
        let data_bounds = self.bounds();

        if groups_bounds.start >= data_bounds.start && groups_bounds.end <= data_bounds.end {
            Ok(())
        } else {
            Err(GetManyDisjointGroupedError::OutOfBounds)
        }
    }

    /// Gets groups of references to items in the underlying data structure
    /// # UNSTABLE
    /// Requires the `slice_ptr_get` unstable nightly feature to be enabled and therefore is
    /// unstable and cannot be stabilized until `slice_ptr_get` is stabilized
    /// # SAFETY
    /// Groups must be disjoint and in bounds. Groups overlapping or being out of bounds will
    /// produce undefined behavior. See `GetManyDisjointGrouped::get_many_disjoint_grouped`
    /// for a safe alternative.
    unsafe fn get_many_disjoint_grouped_unchecked(&self, groups: Groups) -> Vec<Vec<&Item>>;

    /// Gets groups of mutable references to items in the underlying data structure
    /// # UNSTABLE
    /// Requires the `slice_ptr_get` unstable nightly feature to be enabled and therefore is
    /// unstable and cannot be stabilized until `slice_ptr_get` is stabilized
    /// # SAFETY
    /// Groups must be disjoint and in bounds. Groups overlapping or being out of bounds will
    /// produce undefined behavior. See `GetManyDisjointGrouped::get_many_disjoint_grouped`
    /// for a safe alternative.
    unsafe fn get_many_disjoint_grouped_mut_unchecked(
        &mut self,
        groups: Groups,
    ) -> Vec<Vec<&mut Item>>;

    /// Gets groups of mutable references to items in the underlying data structure
    /// returns an error if any groups are not disjoint or out of bounds
    ///
    /// # UNSTABLE
    /// Requires the `slice_ptr_get` unstable nightly feature to be enabled and therefore is
    /// unstable and cannot be stabilized until `slice_ptr_get` is stabilized
    fn get_many_disjoint_grouped(
        &self,
        groups: Groups,
    ) -> Result<Vec<Vec<&Item>>, GetManyDisjointGroupedError> {
        self.check_valid_disjoint_groups(&groups)?;
        // SAFETY: `self.check_valid_disjoint_groups(&groups)` ensures that
        // safety requirements are upheld
        Ok(unsafe { self.get_many_disjoint_grouped_unchecked(groups) })
    }

    /// Gets groups of mutable references to items in the underlying data structure
    /// returns an error if any groups are not disjoint or out of bounds
    ///
    /// UNSTABLE: Requires the `slice_ptr_get` unstable nightly feature to be enabled and
    /// therefore is unstable and cannot be stabilized until `slice_ptr_get` is stabilized
    fn get_many_disjoint_grouped_mut(
        &mut self,
        groups: Groups,
    ) -> Result<Vec<Vec<&mut Item>>, GetManyDisjointGroupedError> {
        self.check_valid_disjoint_groups(&groups)?;
        // SAFETY: `self.check_valid_disjoint_groups(&groups)` ensures that
        // safety requirements are upheld
        Ok(unsafe { self.get_many_disjoint_grouped_mut_unchecked(groups) })
    }
}

impl<Item> GetManyDisjointGrouped<Item> for [Item] {
    fn bounds(&self) -> Range<usize> {
        Range {
            start: 0,
            end: self.len(),
        }
    }

    /// Gets groups of references to items in the underlying data structure
    /// # UNSTABLE
    /// Requires the `slice_ptr_get` unstable nightly feature to be enabled and therefore is
    /// unstable and cannot be stabilized until `slice_ptr_get` is stabilized
    /// # SAFETY
    /// Groups must be disjoint and in bounds. Groups overlapping or being out of bounds will
    /// produce undefined behavior. See `GetManyDisjointGrouped::get_many_disjoint_grouped`
    /// for a safe alternative.
    unsafe fn get_many_disjoint_grouped_unchecked(&self, groups: Groups) -> Vec<Vec<&Item>> {
        let mut output: Vec<Vec<&Item>> = Vec::with_capacity(groups.len());
        for group in groups {
            // we know that our vec will need to be at least group.len items long so we might as well
            // create it with that much capacity. It might be worth trying to figure out a better guess
            // of the total length later as the vec will still have to reallocate at least once most of the time.
            let mut group_out = Vec::with_capacity(group.len());

            for range in group {
                // SAFETY: We expect all ranges in `groups` to be disjoint and in bounds of `self`.
                for item in unsafe { self.get_unchecked(range) } {
                    group_out.push(item)
                }
            }
            output.push(group_out)
        }

        output
    }

    /// Gets groups of mutable references to items in the underlying data structure
    /// # UNSTABLE
    /// Requires the `slice_ptr_get` unstable nightly feature to be enabled and therefore is
    /// unstable and cannot be stabilized until `slice_ptr_get` is stabilized
    /// # SAFETY
    /// Groups must be disjoint and in bounds. Groups overlapping or being out of bounds will
    /// produce undefined behavior. See `GetManyDisjointGrouped::get_many_disjoint_grouped`
    /// for a safe alternative.
    unsafe fn get_many_disjoint_grouped_mut_unchecked(
        &mut self,
        groups: Groups,
    ) -> Vec<Vec<&mut Item>> {
        let raw_self: *mut [Item] = self;

        let mut output: Vec<Vec<&mut Item>> = Vec::with_capacity(groups.len());
        for group in groups {
            // we know that our vec will need to be at least group.len items long so we might as well
            // create it with that much capacity. It might be worth trying to figure out a better guess
            // of the total length later as the vec will still have to reallocate at least once most of the time.
            let mut group_out: Vec<&mut Item> = Vec::with_capacity(group.len());

            for range in group {
                // SAFETY: We expect all ranges in `groups` to be disjoint and in bounds of `self`.
                for item in unsafe { &mut *raw_self.get_unchecked_mut(range) } {
                    group_out.push(item)
                }
            }
            output.push(group_out)
        }

        output
    }
}
