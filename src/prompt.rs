use std::io;

pub trait Prompt {
    type Output;

    fn ask(&self) -> io::Result<Self::Output>;
}
