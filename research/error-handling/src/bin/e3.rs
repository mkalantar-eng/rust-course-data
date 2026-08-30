use fmt::Display;
use std::fmt;
use std::fmt::Formatter;
use std::fs::File;
use std::io;
use std::io::Read;
use std::path::{Path, PathBuf};

trait FormatError {
    fn line(&self) -> u8 {
        0
    }

    fn msg(&self) -> &str {
        ""
    }
}

struct NoHeaderError;
impl FormatError for NoHeaderError {}

struct BodyFormatError {
    line: u8,
    msg: String,
}
impl FormatError for BodyFormatError {
    fn line(&self) -> u8 {
        self.line
    }

    fn msg(&self) -> &str {
        &self.msg
    }
}
struct FileError {
    path: PathBuf,
    source: io::Error,
}

impl Display for FileError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}: {}", self.path.display(), self.source)
    }
}

enum ImportError {
    IoError(FileError),
    FormatError(Box<dyn FormatError>),
}

impl Display for ImportError {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        match self {
            ImportError::IoError(fe) => write!(f, "{}", fe),
            ImportError::FormatError(fe) => write!(f, "line #{} {}", fe.line(), fe.msg()),
        }
    }
}

fn validate(text: &str) -> Result<(), ImportError> {
    if !text.contains("payment_id|debit_account|credit_account|amount|reference") {
        return Err(ImportError::FormatError(Box::new(NoHeaderError)));
    }

    let mut line_no = 1;
    for line in text.lines().skip(1) {
        let parts: Vec<&str> = line.split("|").collect();
        if parts.len() != 5 {
            return Err(ImportError::FormatError(Box::new(BodyFormatError {
                line: line_no,
                msg: "Number of parts must be equal 5".to_string(),
            })));
        }

        if !line.starts_with("PAY-") {
            return Err(ImportError::FormatError(Box::new(BodyFormatError {
                line: line_no,
                msg: "payment_id must start with PAY-".to_string(),
            })));
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
