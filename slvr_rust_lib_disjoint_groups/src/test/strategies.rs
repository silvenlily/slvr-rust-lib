use super::{GroupsBoundsTest};
use crate::Groups;
use proptest::prelude::Strategy;
use std::ops::Range;
use crate::GetManyDisjointGroupedError::{NotDisjoint, OutOfBounds};
use crate::test::ExpectedResult::{Fail, Pass};

pub fn disjoint_groups_strategy(
    max_segment_count: usize,
    max_group_count: usize,
    max_segment_length: usize,
) -> impl Strategy<Value = GroupsBoundsTest> {
    (
        /* out of bounds shift: */ proptest::option::of(1..max_segment_length * 3),
        /* segments & groupings: */
        proptest::collection::vec(
            (
                /* group id / skip */ proptest::option::of(0..max_group_count),
                /* segment length */ 1..=max_segment_length,
                /* overlap shift */ proptest::option::of(1..max_segment_length),
            ),
            /* number of segments */ 0..max_segment_count,
        ),
    )
        .prop_map(move |params| {
            let (oob_count, segments) = params;

            let mut max_group: usize = 0;
            for (group, _len, _may_overlap) in &segments {
                if let Some(g) = group {
                    if *g > max_group {
                        max_group = *g
                    }
                }
            }

            let mut groups: Groups = Vec::with_capacity(max_group + 1);
            for _ in 0..=max_group {
                groups.push(Vec::new())
            }

            let mut expected_result = Pass;
            if !oob_count.is_none() {
                expected_result = Fail(OutOfBounds)
            }

            let mut count: usize = 0;
            let mut last_end: usize = 0;
            let mut last_start: usize = 0;
            let mut max: usize = 0;
            let mut min: usize = usize::MIN;
            for (group, len, overlap_shift) in segments {
                match group {
                    None => {
                        count += len;
                    }
                    Some(group) => {
                        if let Some(shift) = overlap_shift {
                            if count > shift && last_end > shift && last_start < count.saturating_sub(shift) {
                                count = count.saturating_sub(shift);
                                if count < last_end {
                                    expected_result = Fail(NotDisjoint)
                                }
                            }
                        }

                        min = min.min(len);

                        groups[group].push(Range {
                            start: count,
                            end: count + len,
                        });

                        last_start = count;
                        count += len;
                        last_end = count;

                        max = max.max(count);
                    }
                }
            }

            let mut bounds = Range {
                start: min,
                end: max,
            };

            if let Some(oob) = oob_count {
                if bounds.len() > 1 {
                    bounds.end = bounds.end.saturating_sub(oob);
                } else if let Fail(OutOfBounds) = &expected_result {
                    expected_result = Pass;
                }
            };

            (
                groups,
                bounds,
                expected_result
            )
        })
}
