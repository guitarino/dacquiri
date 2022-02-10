
#[derive(Debug)]
pub enum ConstraintError {
    FailedToAddEntity,
    FailedToFetchSubject,
    FailedToFetchResource
}

impl From<ConstraintError> for () {
    fn from(_: ConstraintError) -> Self { }
}

impl From<ConstraintError> for String {
    fn from(err: ConstraintError) -> Self { format!("{:?}", err) }
}