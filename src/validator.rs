pub struct ValidationError {}

pub trait Validator<S> {
    fn validate(&self, input: &S) -> Result<(), ValidationError>;
}
