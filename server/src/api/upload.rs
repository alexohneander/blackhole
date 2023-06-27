use serde::Serialize;
use actix_web::{post, Error as ActixError, HttpResponse};
use actix_multipart::Multipart;
use futures_util::TryStreamExt as _;
use std::fs;

use crate::helpers;

#[derive(Serialize)]
struct Stats {
    length: usize
}


#[post("/api/upload")]
pub async fn file(mut payload: Multipart) -> Result<HttpResponse, ActixError> {
    let mut file_data = Vec::<u8>::new();

    while let Some(mut field) = payload.try_next().await? {
        let content_disposition = field.content_disposition();
        let field_name = content_disposition.get_name().unwrap();

        // try to match the field name
        match field_name {
            "file" => {
                while let Some(chunk) = field.try_next().await? {
                    file_data.extend_from_slice(&chunk);
                }
            }
            _ => {}
        }
    }

    // get file length from file_data
    let length = file_data.len();

    // save file
    let saved = save_file(file_data).await;

    if saved == false {
        return Ok(HttpResponse::InternalServerError().finish());
    }

    Ok(HttpResponse::Ok().json(Stats {
        length: length,
    }))
}

async fn save_file(file_data: Vec<u8>) -> bool {
    // let token = helpers::token::generate_troken();
    let uuid = helpers::token::generate_uuid();

    let filename = "./data/".to_owned() + &uuid + ".tmp";

    let result = fs::write(&filename, file_data);

    match result {
        Ok(_) => {
            println!("File saved as: {}", filename);
        }
        Err(_) => {
            println!("Error saving file");
            return false;
        }
    }

    return true;
}