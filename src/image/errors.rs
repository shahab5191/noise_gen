use std::{fmt, io};

#[derive(Debug)]
pub enum ImageErrors {
    OutOfBounds,
    IoError(io::Error)
}

impl fmt::Display for ImageErrors {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            ImageErrors::OutOfBounds => write!(f, "Out of bound coordinates"),
            ImageErrors::IoError(err) => write!(f, "I/O Error: {}", err),
        }
    }
}

impl From<io::Error> for ImageErrors {
    fn from(error: io::Error) -> Self {
        ImageErrors::IoError(error)
    }
}

impl std::error::Error for ImageErrors {}
