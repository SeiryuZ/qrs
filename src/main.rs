extern crate base64;
extern crate clap;
extern crate hmac;
extern crate sha2;
#[macro_use]
extern crate serde_derive;
extern crate serde;
extern crate serde_json;

extern crate qrcode;
extern crate image;

use qrcode::QrCode;
use image::Luma;

use base64::encode;
use clap::{App, Arg};
use hmac::{Hmac, Mac};
use sha2::Sha256;

#[derive(Serialize, Debug)]
struct Transaction {
    merchant_id: String,
    store_id: String,
    invoice_number: String,
    value: String,
    timestamp: String,
}

#[derive(Serialize, Debug)]
struct Payload {
    transaction: Transaction,
    mac: String,
}

impl Transaction {
    pub fn build(self, key: String) -> Payload {
        let transaction_json = serde_json::to_string(&self).unwrap();

        println!("{}", transaction_json);
        let mut mac = Hmac::<Sha256>::new(&key.as_bytes()).unwrap();
        mac.input(transaction_json.as_bytes());
        let result = mac.result().code().into_iter().collect::<Vec<u8>>();

        Payload {
            transaction: self,
            mac: encode(&result),
        }
    }
}

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
                .takes_value(true)
                .required(true),
        )
        .arg(
            Arg::with_name("merchant_id")
                .short("m")
                .long("merchant-id")
                .value_name("MERCHANT_ID")
                .help("Merchant ID provided from merchant interface")
                .takes_value(true)
                .required(true),
        )
        .arg(
            Arg::with_name("store_id")
                .short("s")
                .long("store-id")
                .value_name("STORE_ID")
                .help("Store ID provided from merchant interface")
                .takes_value(true)
                .required(true),
        )
        .arg(
            Arg::with_name("invoice_number")
                .short("i")
                .long("invoice-number")
                .value_name("INVOICE_NUMBER")
                .help("Invoice Number for this transaction")
                .takes_value(true)
                .required(true),
        )
        .arg(
            Arg::with_name("transaction_value")
                .short("v")
                .long("transaction-value")
                .value_name("TRANSACTION_VALUE")
                .help("Value of this transaction")
                .takes_value(true)
                .required(true),
        )
        .arg(
            Arg::with_name("timestamp")
                .short("t")
                .long("timestamp")
                .value_name("Timestamp")
                .help("Unix timestamp of transaction")
                .takes_value(true)
                .required(true),
        )
        .get_matches();

    // Never fails
    let key = matches.value_of("key").unwrap();
    let merchant_id = matches.value_of("merchant_id").unwrap();
    let store_id = matches.value_of("store_id").unwrap();
    let invoice_number = matches.value_of("invoice_number").unwrap();
    let transaction_value = matches.value_of("transaction_value").unwrap();
    let timestamp = matches.value_of("timestamp").unwrap();

    let transaction = Transaction {
        merchant_id: merchant_id.to_string(),
        store_id: store_id.to_string(),
        invoice_number: invoice_number.to_string(),
        value: transaction_value.to_string(),
        timestamp: timestamp.to_string(),
    };
    let payload = transaction.build(key.to_string());

    // Encode some data into bits.
    let code = QrCode::new(serde_json::to_string(&payload).unwrap()).unwrap();

    // Render the bits into an image.
    let image = code.render::<Luma<u8>>().build();

    // Save the image.
    image.save("qrcode.png").unwrap();
    println!("qrcode.png generated");
}
