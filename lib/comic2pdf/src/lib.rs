use std::fs;
use std::io;
use std::path;
use unrar::Archive;

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

pub fn read_cbr_file<P: AsRef<path::Path>>(path: P) -> Result<(), io::Error> {
    let path = path.as_ref();
    //let output_dir = path::Path::new("/tmp/comics/output/");
    let output_dir = path::Path::new("");
    fs::create_dir_all(output_dir)?;
    let mut archive = Archive::new(path)
        .open_for_processing()
        .map_err(|e| io::Error::new(io::ErrorKind::Other, e.to_string()))?;

    while let Some(header) = archive
        .read_header()
        .map_err(|e| io::Error::new(io::ErrorKind::Other, e.to_string()))?
    {
        let entry_path = output_dir.join(header.entry().filename.clone());
        if let Some(parent) = entry_path.parent() {
            println!("{:?}", parent);
            fs::create_dir_all(parent)?;
        }
        archive = if header.entry().is_file() {
            println!("{:?}", header.entry());
            header
                .extract_to(output_dir)
                .map_err(|e| io::Error::new(io::ErrorKind::Other, e.to_string()))?
        } else {
            // build the directory path straight from the header entry
            let dir_name = header.entry().filename.clone();
            println!("{:?}", dir_name);

            fs::create_dir_all(dir_name)?;

            header
                .skip()
                .map_err(|e| io::Error::new(io::ErrorKind::Other, e.to_string()))?
        }
    }

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn check_read_path() {
        let path = "/tmp/comics/V for Vendetta (Complete)".to_string();
        let output = read_path(path).expect("error reading path!");
        println!("{:?}", output);
        let expected_output: Vec<path::PathBuf> = [].to_vec();
        //assert_eq!(output, expected_output);
    }

    #[test]
    fn check_cbr_file() {
        let file = "/tmp/comics/V for Vendetta (Complete)/V for Vendetta 01 (1988) (c2c) (theProletariat-DCP).cbr".to_string();
        let output = read_cbr_file(file).unwrap();
        println!("{:?}", output);
        assert_eq!(1, 0);
    }
}
