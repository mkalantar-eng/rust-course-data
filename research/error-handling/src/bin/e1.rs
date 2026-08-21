use std::fs::File;
use std::io;
use std::io::Read;
use std::path::Path;

fn read_file(path: &str) -> Result<(usize, String), io::Error> {
    let mut file = File::open(Path::new(path))?;
    let mut buf = String::new();
    let n = file.read_to_string(&mut buf)?;

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
            println!("kind: {:?}", e.kind());
            println!("code: {:?}", e.raw_os_error().unwrap_or(-1));
        }
    }
}
