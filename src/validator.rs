pub type Validator<T> = Box<dyn Fn(&T) -> Result<(), String>>;

pub trait Validate<T> {
    fn validate(self, validator: Validator<T>) -> Self;
}
