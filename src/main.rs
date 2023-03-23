use std::{
    error::Error,
    fs::File,
    io::{Read, Write},
};

use badpiggies_cryptor::{
    cli::{Cli, CryptoModes, FileTypes},
    constant_items,
    crypto::{Cryptor, Sha1HashError},
};
use clap::Parser;

fn main() -> Result<(), Box<dyn Error>> {
    let cli = Cli::parse();
    let mut input_file = File::open(cli.input_file)?;
    let mut input_buffer: Vec<u8> = Vec::new();
    input_file.read_to_end(&mut input_buffer)?;
    let output_buffer: Vec<u8>;
    match cli.file_type {
        FileTypes::Progress => {
            let cryptor = Cryptor::new(constant_items::PROGRESS_PASSWORD, constant_items::SALT);
            match cli.crypto_mode {
                CryptoModes::Encrypt => {
                    let cipher_buffer = cryptor.encrypt(&input_buffer)?;
                    let sha1_buffer = Cryptor::sha1_hash(&cipher_buffer);
                    output_buffer = [sha1_buffer, cipher_buffer].concat();
                }
                CryptoModes::Decrypt => {
                    let sha1_slice = &input_buffer[..20];
                    let cipher_slice = &input_buffer[20..];
                    if sha1_slice != &Cryptor::sha1_hash(cipher_slice) {
                        return Err(From::from(Sha1HashError));
                    }
                    let cipher_buffer: Vec<u8> = cipher_slice.to_vec();
                    output_buffer = cryptor.decrypt(&cipher_buffer)?;
                }
            }
        }
        FileTypes::Contraption => {
            let cryptor = Cryptor::new(constant_items::CONTRAPTION_PASSWORD, constant_items::SALT);
            match cli.crypto_mode {
                CryptoModes::Encrypt => {
                    output_buffer = cryptor.encrypt(&input_buffer)?;
                }
                CryptoModes::Decrypt => {
                    output_buffer = cryptor.decrypt(&input_buffer)?;
                }
            }
        }
    }
    let mut output_file = File::create(cli.output_file)?;
    output_file.write_all(&output_buffer)?;
    Ok(())
}
