use std::fs;
use std::io;
use std::path;
use unrar::Archive;
use krilla::Document;
use krilla::page::PageSettings;
use krilla::image::Image;
use krilla::geom::Size;

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
    let output = "/tmp/comics/output/";
    let output_dir = path::Path::new("");
    std::env::set_current_dir(output)?; // TODO: Fix HACK
    //let output_dir = path::Path::new("");
    fs::create_dir_all(output_dir)?;
    let mut list_of_files: Vec<path::PathBuf> = [].to_vec();

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
            list_of_files.push(header.entry().filename.clone());
            println!("{:?}", header.entry());
            header
                .extract_to(output_dir)
                .map_err(|e| io::Error::new(io::ErrorKind::Other, format!("could not extract file: {}", e.to_string())))?
        } else {
            // build the directory path straight from the header entry
            //let dir_name = header.entry().filename.clone();
            //println!("{:?}", dir_name);

            //fs::create_dir_all(dir_name)?;

            header
                .skip()
                .map_err(|e| io::Error::new(io::ErrorKind::Other, e.to_string()))?
        }
    }

    list_of_files.sort();
    println!("{:?}", list_of_files);

    let mut document = Document::new();

    for file in list_of_files {
        let image_data = fs::read(file)?;
        let mut page = document.start_page();
        let mut surface = page.surface();
        let interpolate = false;
        let image = match file.extension().and_then(|s| s.to_str()) {
            Some("jpg") => Image::from_jpeg(image_data.into(), interpolate)?,
            Some("png") => Image::from_png(image_data.into(), interpolate)?,
            Some(ext) => {
                println!("Other extension: {}", ext);
                continue;
            }
            None => {
                println!("No extension found");
                continue;
            }
        };
        let u32_image_size = image.size();
        let image_size: (f32, f32) = (u32_image_size.0 as f32, u32_image_size.1 as f32);
        let size = Size::from_wh(image_size.0, image_size.1)?;
        surface.draw_image(image, size);
        surface.finish();
        page.finish();
    }
    let pdf_bytes = document.finish()?;
    std::fs::write("output.pdf", &pdf_bytes)?;
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
