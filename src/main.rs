// Copyright (c) 2023 Petruknisme
//
// This software is released under the MIT License.
// https://opensource.org/licenses/MIT

use base64::{Engine as _, engine::general_purpose};
use clap::Parser;

#[derive(Parser)]
#[clap(name = "pwsh-b64-rs")]
#[clap(author = "Petruknisme <me@petruknisme.com>")]
#[clap(version = "1.0")]
#[clap(about = "Powershell implementation of ToBase64String UTF-16LE written in Rust", long_about = None)]

struct Cli {
    /// Text to be encoded
    #[clap(short, long)]
    text: String,
}

fn b64_encode(text: &str) -> String {
    let mut text_bytes: Vec<u8> = Vec::new();
    for x in text.encode_utf16() {
        let text_le = x.to_le_bytes();
        text_bytes.push(text_le[0]);
        text_bytes.push(text_le[1]);
    }
    general_purpose::STANDARD.encode(text_bytes.as_slice())
}

fn main() {
    let cli = Cli::parse();
    let text = cli.text;

    let out = b64_encode(&text);
    println!("{}", out);

}