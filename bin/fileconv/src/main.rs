use comic2pdf::{read_cbr_file};

fn main() {
    let file = "/tmp/comics/V for Vendetta (Complete)/V for Vendetta 01 (1988) (c2c) (theProletariat-DCP).cbr".to_string();
    let out = read_cbr_file(file);
    println!("{:?}", out);
}
