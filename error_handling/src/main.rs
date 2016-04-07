use std::fs::File;
use std::io::{self, Read};
use std::num;
use std::path::Path;

#[derive(Debug)]
enum FunError {
    Io(io::Error),
    ParseInt(num::ParseIntError),
}

fn main() {
}

fn file_double_verbose<P: AsRef<Path>>(file_path: P) -> Result<i32, FunError> {
    let mut file = try!(File::open(file_path).map_err(FunError::Io));
    let mut contents = String::new();
    try!(file.read_to_string(&mut contents).map_err(FunError::Io));
    let n: i32 = try!(contents.trim().parse().map_err(FunError::ParseInt));
    Ok(2 * n)
}
