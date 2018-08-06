extern crate clap;

use clap::{App, Arg};

fn main() {
    let matches = App::new("QRs code generator")
        .version("0.1.0")
        .author("Steven <steven@ui.co.id>")
        .about("Generate QR code for transaction information")
        .arg(
            Arg::with_name("key")
                .short("k")
                .long("secret-key")
                .value_name("KEY")
                .help("Secret Key for the encryption")
                .takes_value(true),
        ).get_matches();

    println!("Hello, world!");
}
