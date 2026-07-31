use crate::ChevronError;

pub trait Prompt {
    type Output;

    fn ask(&self) -> Result<Self::Output, ChevronError>;
}
