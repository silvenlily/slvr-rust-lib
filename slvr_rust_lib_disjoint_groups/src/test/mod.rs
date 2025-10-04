use crate::Groups;
use crate::test::groups_test::groups_test;
use proptest::prelude::*;
use proptest::test_runner::TestRunner;
use std::ops::Range;
use proptest::test_runner;

#[derive(Debug)]
enum ExpectedResult {
    Pass,
    Fail(super::GetManyDisjointGroupedError),
}

type GroupsBoundsTest = (Groups, Range<usize>, ExpectedResult);

mod groups_test;
mod strategies;

fn pass_err<Ok>(
    expected_result: ExpectedResult,
    actual_result: Result<(), super::GetManyDisjointGroupedError>,
) -> Result<Ok, TestCaseError> {
    Err(TestCaseError::fail(format!(
        "got {:?} expected {:?}",
        actual_result, expected_result
    )))
}

#[test]
fn test_groups() {
    let mut runner_config = test_runner::Config::default();
    runner_config.failure_persistence = None;
    
    for _ in 0..=100 {
        TestRunner::new(runner_config.clone())
            .run(
                &strategies::disjoint_groups_strategy(20, 20, 20),
                groups_test,
            )
            .unwrap();
    }
}
