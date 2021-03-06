extern crate qrcodegen;
extern crate structopt;
extern crate structopt_derive;

use std::process::exit;
use std::io::{stdout, Write};

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
        help = "specifies the error correction level to use",
        parse(try_from_str = "ecc_from_str"),
        default_value = "high"
    )]
    coding: QrCodeEcc,

    #[structopt(
        help = "text that will be encoded",
        name = "text"
    )]
    text: String,

    #[structopt(
        short = "b",
        long = "bech32",
        help = "treat text as bech32 (save space by transforming it to all uppercase)",
    )]
    bech32: bool,
}

fn print_qr(qr: &QrCode) -> std::io::Result<()> {
    let stdout = stdout();
    let mut stdout_handle = stdout.lock();
    writeln!(stdout_handle)?;
    for y in 0 .. qr.size() {
        write!(stdout_handle, " ")?;
        for x in 0 .. qr.size() {
            write!(stdout_handle, "{}", if qr.get_module(x, y) {
                "██"
            } else {
                "  "
            })?;
        }
        writeln!(stdout_handle)?;
    }
    writeln!(stdout_handle)?;

    Ok(())
}

fn main() {
    let options = Options::from_args();

    let text = if options.bech32 {
        options.text.to_uppercase()
    } else {
        options.text
    };

    let qr = QrCode::encode_text(&text, options.coding).unwrap_or_else(|_| {
        eprintln!("Could not encode given data as QR code (it's probably too large).");
        exit(-1);
    });

    print_qr(&qr).unwrap_or_else(|e| {
        eprintln!("IO error: {:?}", e);
        exit(-2);
    });
}
