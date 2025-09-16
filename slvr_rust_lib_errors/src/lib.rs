pub enum OptionalResult<Value,Err> {
    Ok(Value),
    Err(Err),
    None,
}

impl <Ok,Err> From<Result<Ok,Err>> for OptionalResult<Ok,Err> {
    fn from(value: Result<Ok, Err>) -> Self {
        match value {
            Ok(value) => OptionalResult::Ok(value),
            Err(err) => OptionalResult::Err(err)
        }
    }
}

impl <Ok,Err> From<Option<Ok>> for OptionalResult<Ok,Err> {

    fn from(value: Option<Ok>) -> Self {
        match value {
            None => OptionalResult::None,
            Some(value) => OptionalResult::Ok(value)
        }
    }
}

impl <Ok,Err> From<Option<Result<Ok,Err>>> for OptionalResult<Ok,Err> {
    fn from(value: Option<Result<Ok,Err>>) -> Self {
        match value {
            None => OptionalResult::None,
            Some(result) => result.into()
        }
    }
}

impl <Ok,Err> From<Result<Option<Ok>,Err>> for OptionalResult<Ok,Err> {
    fn from(value: Result<Option<Ok>,Err>) -> Self {
        match value {
            Ok(optional) => optional.into(),
            Err(err) => {OptionalResult::Err(err)}
        }
    }
}


