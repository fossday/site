use rocket::tokio::fs;
use serde::Deserialize;
use std::io::Error;

#[derive(Deserialize)]
pub struct File {
    pub header: String,
    pub content: String,
}

pub async fn get_content(name: &str) -> Result<File, Error> {

    let path = format!("content/{}", name);
    let content_file = fs::read_to_string(path)
        .await
        .map_err(Into::<Error>::into)?;

    let content_vector: Vec<&str> = content_file
        .split("+++")
        .collect();

    let header = content_vector[1].to_string();
    let content = content_vector[2].to_string();
    let file: File = File{ header, content };
    
    Ok(file)
}
