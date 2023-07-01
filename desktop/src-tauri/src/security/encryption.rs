use aes_gcm::{
    aead::{Aead, AeadCore, KeyInit, OsRng},
    Aes256Gcm, Nonce, Key // Or `Aes128Gcm`
};

use std::fs::File;
use std::io::{BufReader, BufWriter, Read, Write};




pub fn encrypt_file(path: &str) -> Result<String, Box<dyn std::error::Error>> {
    // Open File and BufferReader
    let mut file = File::open(path)?;
    let mut reader = BufReader::new(&file);
    
    // Create output file
    let mut output_file = File::create("encrypted_file")?;
    let mut writer = BufWriter::new(&output_file);

    // The encryption key can be generated randomly:
    let key = Aes256Gcm::generate_key(OsRng);

    // Create cipher
    let cipher = Aes256Gcm::new(&key);

    // Encrypt file
    let mut buffer = [0; 4096];
    let mut total_bytes_read = 0;
    
    // loop {
    //     let bytes_read = reader.read(&mut buffer)?;
    //     if bytes_read == 0 {
    //         break;
    //     }
    //     total_bytes_read += bytes_read;
    //     let mut ciphertext = vec![0; bytes_read + 16];
    //     let tag = cipher.encrypt(nonce, &buffer[..bytes_read], &mut ciphertext)?;
    //     writer.write_all(&ciphertext)?;
    //     writer.write_all(tag.as_ref())?;
    // }

    println!("File Contents: {} bytes", total_bytes_read);
    Ok("File encrypted successfully ".to_string())
}