use std::error::Error;
use std::fmt::{Display, Formatter};
use std::fs::File;
use std::io;
use std::io::Read;
use std::path::Path;

#[derive(Debug)]
struct FileError<'a> {
    path: &'a Path,
    source: io::Error,
}

impl<'a> Display for FileError<'a> {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}: {}", self.path.display(), self.source)
    }
}

/*
Implement std::error::Error for FileError returning the inner io::Error,
so it composes with things that expect dyn Error (not required, just optional polish)
*/
impl<'a> Error for FileError<'a> {
    fn source(&self) -> Option<&(dyn Error + 'static)> {
        Some(&self.source)
    }
}

fn read_file(path_str: &str) -> Result<(usize, String), FileError<'_>> {
    let path = Path::new(path_str);
    let mut file = File::open(path).map_err(|source| FileError { path, source })?;
    let mut buf = String::new();
    let n = file
        .read_to_string(&mut buf)
        .map_err(|source| FileError { path, source })?;

    Ok((n, buf))
}

fn main() {
    let res = read_file("tx0s.txt");
    match res {
        Ok((n, text)) => {
            println!("READ: {n} bytes");
            println!();
            println!("{text}");
        }
        Err(e) => {
            println!("error: {e}");
            println!("kind: {:?}", e.source.kind());
            println!("code: {:?}", e.source.raw_os_error().unwrap_or(-1));
        }
    }
}
