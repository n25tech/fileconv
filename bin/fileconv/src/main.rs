use comic2pdf::{cbr_to_pdf};

fn main() {
    let file = "/tmp/comics/V for Vendetta (Complete)/V for Vendetta 01 (1988) (c2c) (theProletariat-DCP).cbr".to_string();
    let out = cbr_to_pdf(file);
    println!("{:?}", out);
}
