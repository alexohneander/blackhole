use encryptfile as ef;

use std::fs::File;
use std::io::{BufReader, BufWriter, Read, Write};

use crate::helpers;


pub struct EncryptionResponse {
    pub file_id: String,
    pub enc_token: String,
    pub file_name: String,
}

pub fn encrypt_file(path: &str) -> Result<EncryptionResponse, Box<dyn std::error::Error>> {    
    let mut c = ef::Config::new();

    let enc_token = helpers::token::generate_token();

    let file_id = helpers::token::generate_uuid() + ".ef";
    let data_path = "../data/".to_string() + &file_id;
    let file_name = path.clone().to_string();

    c.input_stream(ef::InputStream::File(path.to_owned()))
        .output_stream(ef::OutputStream::File(data_path.clone()))
        .add_output_option(ef::OutputOption::AllowOverwrite)
        .initialization_vector(ef::InitializationVector::GenerateFromRng)
        .password(ef::PasswordType::Text(enc_token.clone(), ef::scrypt_defaults()))
        .encrypt();

    let _ = ef::process(&c).map_err(|e| panic!("error encrypting: {:?}", e));

    Ok(EncryptionResponse {
        file_id,
        enc_token,
        file_name
    })
}

pub fn decrypt_file(path: &str, token: &str) -> Result<String, Box<dyn std::error::Error>> {
    let mut c = ef::Config::new();

    c.input_stream(ef::InputStream::File(path.to_owned()))
        .output_stream(ef::OutputStream::File("../data/test.txt".to_owned()))
        .add_output_option(ef::OutputOption::AllowOverwrite)
        .password(ef::PasswordType::Text(token.to_owned(), ef::PasswordKeyGenMethod::ReadFromFile))
        .decrypt();

    let _ = ef::process(&c).map_err(|e| panic!("error decrypting: {:?}", e));  

    Ok("File decrypted successfully ".to_string())
}