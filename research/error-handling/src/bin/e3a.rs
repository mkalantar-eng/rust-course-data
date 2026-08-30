use std::fmt;
use std::fmt::{Display, Formatter};
use std::fs::File;
use std::io;
use std::io::Read;
use std::path::{Path, PathBuf};

struct FileError {
    path: PathBuf,
    source: io::Error,
}

impl Display for FileError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}: {}", self.path.display(), self.source)
    }
}

enum FormatError {
    NoHeader,
    WrongFieldCount {
        line: u32,
        found: usize,
        expected: usize,
    },
    BadPrefix {
        line: u32,
        expected: &'static str,
    },
}

impl Display for FormatError {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        match self {
            FormatError::NoHeader => write!(f, "missing header"),
            FormatError::WrongFieldCount {
                line,
                found,
                expected,
            } => write!(f, "line #{line}: expected {expected} fields, found {found}"),
            FormatError::BadPrefix { line, expected } => {
                write!(f, "line #{line}: payment_id must start with {expected}")
            }
        }
    }
}

enum ImportError {
    IoError(FileError),
    FormatError(FormatError),
}

impl Display for ImportError {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        match self {
            ImportError::IoError(fe) => write!(f, "{}", fe),
            ImportError::FormatError(fe) => write!(f, "{}", fe),
        }
    }
}

fn validate(text: &str) -> Result<(), ImportError> {
    if !text.contains("payment_id|debit_account|credit_account|amount|reference") {
        return Err(ImportError::FormatError(FormatError::NoHeader));
    }

    let mut line_no = 1;
    for line in text.lines().skip(1) {
        let parts: Vec<&str> = line.split("|").collect();
        if parts.len() != 5 {
            return Err(ImportError::FormatError(FormatError::WrongFieldCount {
                line: line_no,
                found: parts.len(),
                expected: 5,
            }));
        }

        if !line.starts_with("PAY-") {
            return Err(ImportError::FormatError(FormatError::BadPrefix {
                line: line_no,
                expected: "PAY-",
            }));
        }
        line_no += 1;
    }

    Ok(())
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
    // let path = "txs.txt";
    let path = "e:/rust-programming-ztm/rust-course-data/research/error-handling/src/bin/txs.txt";

    let res = read_file(path);
    match res {
        Ok((n, text)) => {
            println!("READ: {n} bytes");
            match validate(text.as_str()) {
                Ok(_) => println!("VALID: {}", text),
                Err(e) => println!("INVALID: {}", e),
            }
        }
        Err(e) => {
            println!("error: {e}");
            println!("path: {}", e.path.display());
            println!("kind: {:?}", e.source.kind());
            println!("code: {:?}", e.source.raw_os_error().unwrap_or(-1));
        }
    }
}
