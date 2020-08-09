pub struct ValidationError(pub String);

pub trait Validation<V> {
    fn validate(&self, value: &V) -> Result<(), ValidationError>;
}

pub struct Required;

impl Validation<String> for Required {
    fn validate(&self, value: &String) -> Result<(), ValidationError> {
        if value.is_empty() {
            Err(ValidationError("required".to_owned()))
        } else {
            Ok(())
        }
    }
}

impl<T> Validation<Vec<T>> for Required {
    fn validate(&self, value: &Vec<T>) -> Result<(), ValidationError> {
        if value.is_empty() {
            Err(ValidationError("should be at least one".to_owned()))
        } else {
            Ok(())
        }
    }
}

pub struct MinLen(pub usize);

impl Validation<String> for MinLen {
    fn validate(&self, value: &String) -> Result<(), ValidationError> {
        if value.len() >= self.0 {
            Ok(())
        } else {
            Err(ValidationError(format!(
                "Should at least {} charecters long",
                self.0
            )))
        }
    }
}

impl<T> Validation<Vec<T>> for MinLen {
    fn validate(&self, value: &Vec<T>) -> Result<(), ValidationError> {
        if value.len() >= self.0 {
            Ok(())
        } else {
            Err(ValidationError(format!("Should at least {}", self.0)))
        }
    }
}

pub struct MaxLen(pub usize);

impl Validation<String> for MaxLen {
    fn validate(&self, value: &String) -> Result<(), ValidationError> {
        if value.len() >= self.0 {
            Ok(())
        } else {
            Err(ValidationError("at least length".to_owned()))
        }
    }
}

impl<T> Validation<Vec<T>> for MaxLen {
    fn validate(&self, value: &Vec<T>) -> Result<(), ValidationError> {
        if value.len() <= self.0 {
            Ok(())
        } else {
            Err(ValidationError("at least length".to_owned()))
        }
    }
}

pub struct Parse<P>(std::marker::PhantomData<P>);

impl<P> Parse<P> {
    pub fn new() -> Parse<P> {
        Parse(std::marker::PhantomData)
    }
}

impl<P: std::str::FromStr> Validation<String> for Parse<P>
where
    P::Err: std::error::Error,
{
    fn validate(&self, value: &String) -> Result<(), ValidationError> {
        match P::from_str(value) {
            Ok(_) => Ok(()),
            Err(e) => Err(ValidationError(e.to_string())),
        }
    }
}

pub struct ValidationFn<F>(F);

impl<F, T> Validation<T> for ValidationFn<F>
where
    F: Fn(&T) -> Result<(), ValidationError>,
{
    fn validate(&self, value: &T) -> Result<(), ValidationError> {
        (self.0)(value)
    }
}

pub fn validation<F, S>(cb: F) -> impl Validation<S>
where
    F: Fn(&S) -> Result<(), ValidationError>,
{
    ValidationFn(cb)
}
