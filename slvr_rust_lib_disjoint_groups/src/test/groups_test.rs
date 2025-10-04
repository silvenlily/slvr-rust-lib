use crate::test::{pass_err, ExpectedResult, GroupsBoundsTest};
use crate::{GetManyDisjointGroupedError, check_valid_disjoint_groups};
use proptest::prelude::TestCaseError;

pub(super) fn groups_test(input: GroupsBoundsTest) -> Result<(), TestCaseError> {
    let (group, bounds, expected_result) = input;

    let result = check_valid_disjoint_groups(&bounds, &group);

    match (result, expected_result) {
        (Ok(()), ExpectedResult::Pass) => Ok(()),
        (
            Err(GetManyDisjointGroupedError::OutOfBounds),
            ExpectedResult::Fail(GetManyDisjointGroupedError::OutOfBounds),
        ) => Ok(()),
        (
            Err(GetManyDisjointGroupedError::NotDisjoint),
            ExpectedResult::Fail(GetManyDisjointGroupedError::NotDisjoint),
        ) => Ok(()),
        (res,expected) => {
            pass_err(expected,res)
        }
    }
}
