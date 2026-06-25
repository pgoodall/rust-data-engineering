use csv::{Reader, Writer};
use std::io;
use std::fmt;
use std::error::Error;

// Even though it is beyond the brief, I'm implementing custom error messages.
// This is for practice.
// #[derive(Debug)]
// enum FileError {
//     Read(io::Error),
//     Write(io::Error),
//     Perms(io::Error),
// }

// // Need to specify how a FileError would be displayed.
// impl fmt::Display for FileError {
//     fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
//         match self {
//             FileError::Read(_) => write!(f, "I/O error while reading file"),
//             FileError::Write(_) => write!(f, "I/O error while writing file"),
//             FileError::Perms(_) => write!(f, "Permissions error for file"),
//         }
//     }
// }


// impl Error for FileError {
//     fn source(&self) -> Option<&(dyn Error + 'static)> {
//         match self {
//             FileError::Read(err) => Some(err),
//             FileError::Write(err) => Some(err),
//             FileError::Perms(err) => Some(err),
//         }
//     }
// }

// impl From<io::Error> for FileError {
//     fn from(err: io::Error) -> Self {
//         FileError
//     }
// }


fn main() -> Result<(), FileError> {
    
    Ok()
}