use proptest::prelude::*;
use proptest::test_runner::TestRunner;
use std::ops::Range;

use crate::*;

fn pass_err<Msg: ToString, T, Ok>(msg: Msg, slice: &[T]) -> Result<Ok, TestCaseError> {
    Err(TestCaseError::fail(format!(
        "{} - slice len: {}",
        msg.to_string(),
        slice.len()
    )))
}

#[test]
fn test_valid_groups() {
    TestRunner::default()
        .run(&disjoint_groups_strategy(10, 10, 10), valid_groups_test)
        .unwrap();
}
#[test]
fn test_short_groups() {
    TestRunner::default()
        .run(&disjoint_groups_strategy(10, 10, 10), short_groups_test)
        .unwrap();
}

fn disjoint_groups_strategy(
    max_segment_count: usize,
    max_group_count: usize,
    max_segment_length: usize,
) -> impl Strategy<Value = Vec<Vec<Range<usize>>>> {
    prop::collection::vec(
        (prop::option::of(0..max_group_count), 1..=max_segment_length),
        0..max_segment_count,
    )
    .prop_map(move |segments| {
        let mut max_group: usize = 0;
        for (group, _len) in &segments {
            if let Some(g) = group {
                if *g > max_group {
                    max_group = *g
                }
            }
        }

        let mut groups: Vec<Vec<Range<usize>>> = Vec::with_capacity(max_group + 1);
        for _ in 0..=max_group {
            groups.push(Vec::new())
        }

        let mut count: usize = 0;

        for (group, len) in segments {
            match group {
                None => {
                    count += len;
                }
                Some(group) => {
                    groups[group].push(Range {
                        start: count,
                        end: count + len,
                    });
                    count += len;
                }
            }
        }

        groups
    })
}

fn valid_groups_test(group: Groups) -> Result<(), TestCaseError> {
    let mut max: usize = 0;

    group.iter().flatten().for_each(|r| {
        if r.end > max {
            max = r.end
        }
    });
    let mut vslice = Vec::new();
    for _ in 0..max {
        vslice.push(true);
    }
    let slice = vslice.as_slice();

    match slice.check_valid_disjoint_groups(&group) {
        Ok(_) => Ok(()),
        Err(e) => pass_err(e, slice),
    }
}

fn short_groups_test(group: Groups) -> Result<(), TestCaseError> {
    let mut max: usize = 0;

    group.iter().flatten().for_each(|r| {
        if r.end > max {
            max = r.end
        }
    });

    max = max.saturating_sub(1);
    if max == 0 {
        return Err(TestCaseError::reject("len is 0"))
    }

    let mut vslice = Vec::new();
    for _ in 0..max {
        vslice.push(true);
    }
    let slice = vslice.as_slice();

    match slice.check_valid_disjoint_groups(&group) {
        Err(GetManyDisjointGroupedError::OutOfBounds) => Ok(()),
        Ok(()) => pass_err("Test should have errored due to out of bounds", slice),
        Err(e) => pass_err(e, slice),
    }
}
