use std::fmt;
use std::fs::File;
use std::io;
use std::io::Read;
use std::path::{Path, PathBuf};

struct FileError {
    path: PathBuf,
    source: io::Error,
}

impl fmt::Display for FileError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}: {}", self.path.display(), self.source)
    }
}

fn read_file(path: &str) -> Result<(usize, String), FileError> {
    let path = Path::new(path);
    let mut file = File::open(path).map_err(|source| FileError {
        path: path.to_path_buf(),
        source,
    })?;
    let mut buf = String::new();
    let n = file.read_to_string(&mut buf).map_err(|source| FileError {
        path: path.to_path_buf(),
        source,
    })?;

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
            println!("path: {}", e.path.display());
            println!("kind: {:?}", e.source.kind());
            println!("code: {:?}", e.source.raw_os_error().unwrap_or(-1));
        }
    }
}
