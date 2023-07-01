use encryptfile as ef;

use std::fs::File;
use std::io::{BufReader, BufWriter, Read, Write};

use crate::helpers;


pub fn encrypt_file(path: &str) -> Result<String, Box<dyn std::error::Error>> {    
    let mut c = ef::Config::new();

    let enc_token = helpers::token::generate_token();

    let file_id = helpers::token::generate_uuid() + ".ef";
    let data_path = "../data/".to_string() + &file_id;

    c.input_stream(ef::InputStream::File(path.to_owned()))
        .output_stream(ef::OutputStream::File(data_path))
        .add_output_option(ef::OutputOption::AllowOverwrite)
        .initialization_vector(ef::InitializationVector::GenerateFromRng)
        .password(ef::PasswordType::Text(enc_token, ef::scrypt_defaults()))
        .encrypt();

    let _ = ef::process(&c).map_err(|e| panic!("error encrypting: {:?}", e));

    // let result = decrypt_file(&data_path, &enc_token);

    Ok("File encrypted successfully ".to_string())
}

pub fn decrypt_file(path: &str, token: &str) -> Result<String, Box<dyn std::error::Error>> {
    let mut c = ef::Config::new();

    c.input_stream(ef::InputStream::File(path.to_owned()))
        .output_stream(ef::OutputStream::File("/tmp/__encrypted_bash_history.txt".to_owned()))
        .add_output_option(ef::OutputOption::AllowOverwrite)
        .password(ef::PasswordType::Text(token.to_owned(), ef::PasswordKeyGenMethod::ReadFromFile))
        .decrypt();

    let _ = ef::process(&c).map_err(|e| panic!("error decrypting: {:?}", e));  

    Ok("File decrypted successfully ".to_string())
}