use std::path::Path;
use comic2pdf::{cbr_to_pdf};
use std::process::exit;

gflags::define! {
    -i, --input-file: &Path
}

fn main() {
    gflags::parse();
    let file_name = INPUT_FILE.flag;
    if !file_name.exists() {
        println!("can't find input file: {:?}", file_name.display());
        exit(1);
    }

    //let file = "/tmp/comics/V for Vendetta (Complete)/V for Vendetta 01 (1988) (c2c) (theProletariat-DCP).cbr".to_string();
    let file = file_name.to_str().unwrap();
    let out = cbr_to_pdf(file);
    println!("{:?}", out);
}
