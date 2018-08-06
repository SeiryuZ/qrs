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
                .value_name("TOKEN")
                .help("Secret Key(Token) provided from merchant interface")
                .takes_value(true),
        )
        .arg(
            Arg::with_name("Merchant ID")
                .short("m")
                .long("merchant-id")
                .value_name("MERCHANT_ID")
                .help("Merchant ID provided from merchant interface")
                .takes_value(true),
        )
        .arg(
            Arg::with_name("Store ID")
                .short("s")
                .long("store-id")
                .value_name("STORE_ID")
                .help("Store ID provided from merchant interface")
                .takes_value(true),
        )
        .arg(
            Arg::with_name("Invoice Number")
                .short("i")
                .long("invoice-number")
                .value_name("INVOICE_NUMBER")
                .help("Invoice Number for this transaction")
                .takes_value(true),
        )
        .arg(
            Arg::with_name("Value")
                .short("v")
                .long("value")
                .value_name("TRANSACTION_VALUE")
                .help("Value of this transaction")
                .takes_value(true),
        )
        .arg(
            Arg::with_name("Timestamp")
                .short("t")
                .long("timestamp")
                .value_name("Timestamp")
                .help("Unix timestamp of transaction")
                .takes_value(true),
        )
        .get_matches();

    println!("Hello, world!");
}
