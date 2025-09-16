#![feature(try_trait_v2)]
#![feature(try_trait_v2_residual)]

use std::convert::Infallible;
use std::ops::{ControlFlow, FromResidual, Try};

pub enum OptionalResult<Value, Err> {
    Ok(Value),
    Err(Err),
    None,
}

impl<Ok, Err> From<Result<Ok, Err>> for OptionalResult<Ok, Err> {
    fn from(value: Result<Ok, Err>) -> Self {
        match value {
            Ok(value) => OptionalResult::Ok(value),
            Err(err) => OptionalResult::Err(err),
        }
    }
}

impl<Ok, Err> From<Option<Ok>> for OptionalResult<Ok, Err> {
    fn from(value: Option<Ok>) -> Self {
        match value {
            None => OptionalResult::None,
            Some(value) => OptionalResult::Ok(value),
        }
    }
}

impl<Ok, Err> From<Option<Result<Ok, Err>>> for OptionalResult<Ok, Err> {
    fn from(value: Option<Result<Ok, Err>>) -> Self {
        match value {
            None => OptionalResult::None,
            Some(result) => result.into(),
        }
    }
}

impl<Ok, Err> From<Result<Option<Ok>, Err>> for OptionalResult<Ok, Err> {
    fn from(value: Result<Option<Ok>, Err>) -> Self {
        match value {
            Ok(optional) => optional.into(),
            Err(err) => OptionalResult::Err(err),
        }
    }
}

impl<Ok, Err> Try for OptionalResult<Ok, Err> {
    type Output = Ok;
    type Residual = OptionalResult<Infallible, Err>;

    fn branch(self) -> ControlFlow<OptionalResult<Infallible, Err>, Ok> {
        match self {
            OptionalResult::Ok(val) => ControlFlow::Continue(val),
            OptionalResult::Err(err) => ControlFlow::Break(OptionalResult::Err(err)),
            OptionalResult::None => ControlFlow::Break(OptionalResult::None),
        }
    }

    fn from_output(value: Ok) -> Self {
        OptionalResult::Ok(value)
    }
}

impl<Ok, Err> FromResidual<OptionalResult<Infallible, Err>> for OptionalResult<Ok, Err> {
    fn from_residual(residual: OptionalResult<Infallible, Err>) -> Self {
        match residual {
            OptionalResult::Ok(x) => match x {},
            OptionalResult::Err(err) => OptionalResult::Err(err),
            OptionalResult::None => OptionalResult::None,
        }
    }
}

impl<Ok, Err> FromResidual<Result<Infallible, Err>> for OptionalResult<Ok, Err> {
    fn from_residual(residual: Result<Infallible, Err>) -> Self {
        match residual {
            Ok(x) => match x {},
            Err(err) => OptionalResult::Err(err),
        }
    }
}

impl<Ok, Err> FromResidual<Option<Infallible>> for OptionalResult<Ok, Err> {
    fn from_residual(residual: Option<Infallible>) -> Self {
        match residual {
            Some(x) => match x {},
            None => OptionalResult::None
        }
    }
}
