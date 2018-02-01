extern crate qrcodegen;
extern crate structopt;
#[macro_use]
extern crate structopt_derive;

use std::process::exit;
use qrcodegen::{QrCode, QrCodeEcc};
use structopt::StructOpt;

fn ecc_from_str(ecc: &str) -> Result<QrCodeEcc, String> {
    match ecc {
        "low" => Ok(QrCodeEcc::Low),
        "medium" => Ok(QrCodeEcc::Medium),
        "high" => Ok(QrCodeEcc::High),
        "quartile" => Ok(QrCodeEcc::Quartile),
        _ => Err(format!("unknown error correction level'{}'", ecc))
    }
}

#[derive(StructOpt)]
#[structopt(name = "qrcli")]
struct Options {
    #[structopt(
        short = "e",
        long = "error-correction",
        help = "specifies the error correction code to use",
        parse(try_from_str = "ecc_from_str"),
        default_value = "high"
    )]
    coding: QrCodeEcc,

    #[structopt(
        help = "text that will be encoded",
        name = "text"
    )]
    text: String,
}

fn main() {
    let options = Options::from_args();

    let qr = QrCode::encode_text(&options.text, options.coding).unwrap_or_else(|| {
        eprintln!("Could not encode given data as QR code (it's probably too large).");
        exit(-1);
    });

    println!();
    for y in 0 .. qr.size() {
        print!(" ");
        for x in 0 .. qr.size() {
            print!("{}", if qr.get_module(x, y) {
                "██"
            } else {
                "  "
            });
        }
        println!();
    }
    println!();
}
