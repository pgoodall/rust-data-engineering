use csv::{Reader, Writer};
use std::io;
// use std::fmt; // Not needed for now
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

fn write_csv(list: (&str, f64), path: Option<&str>) -> Result<(), Box<dyn Error>> {
    match path {
        Some(path) => { let mut wtr = Writer::from_path(path)? },
        None => { for (name, price) in list {
                    
        }}
    }
    
    for (name, price) in list {
        wtr.write_record([name, price.to_string()])?;
    }

    Ok(())
}
fn read_csv(file: &str) -> Result<(), Box<dyn Error>> {
    let mut rdr = Reader::from_path(file)?;
    for result in rdr.records() {
        let record = result?;
        println!("{:?}", record);
    }
    Ok(())
}
fn main() -> Result<(), Box<dyn Error>> {
    let fruits = [
        ("Apple", 1.25),
        ("Banana", 0.75),
        ("Orange", 1.00),
        ("Mango", 2.50),
        ("Pineapple", 3.00),
    ];

    let product_list = write_csv(fruits, "data/product_list.csv")?;
    
    let _rdr = read_csv("data/output.csv")?;
    Ok(())
}