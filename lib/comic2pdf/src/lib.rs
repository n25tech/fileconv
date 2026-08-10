use std::fs;
use std::io;
use std::path;

pub fn add(left: u64, right: u64) -> u64 {
    left + right
}

fn check_extension(path: &path::PathBuf) -> bool {
    if path.is_dir() {
        return false;
    }
    let extension = path.extension();
    match extension {
        Some(ext) => ext == "cbr",
        None => false,
    }
}

/// Caveats:
/// * Is not recursive
/// * Silently ignores errors and directories
pub fn read_path<P: AsRef<path::Path>>(path: P) -> Result<Vec<path::PathBuf>, io::Error> {
    let path = path.as_ref();
    if path.as_os_str().is_empty() {
        return Err(io::Error::new(
            io::ErrorKind::InvalidInput,
            "input is empty, path not specified",
        ));
    }
    let entries: Vec<std::path::PathBuf> = fs::read_dir(path)?
        .filter_map(|res: Result<fs::DirEntry, io::Error>| res.ok())
        .map(|e| e.path())
        .filter(|p| check_extension(p))
        .collect();
    println!("{:?}", entries);
    Ok(entries)
}

pub fn read_cbr_file<P: AsRef<path::Path>>(path: P) {
    let path = path.as_ref();
    println!("{:?}", path);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let result = add(2, 2);
        assert_eq!(result, 4);
    }

    #[test]
    fn check_read_path() {
        let path = "/tmp/comics/V for Vendetta (Complete)".to_string();
        let output = read_path(path).expect("error reading path!");
        let expected_output: Vec<path::PathBuf> = [].to_vec();
        //assert_eq!(output, expected_output);
    }

    #[test]
    fn check_cbr_file() {
        let file = "/tmp/comics/V for Vendetta (Complete)/V for Vendetta 01 (1988) (c2c) (theProletariat-DCP).cbr".to_string();
            read_cbr_file(file);
            assert_eq!(1, 0);
    }
}
