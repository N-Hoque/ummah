use std::io::Write;

use bytes::Bytes;
use reqwest::IntoUrl;

use crate::types::{UmmahError, UmmahResult};

pub async fn download_file<T: IntoUrl>(url: T, progress_message: &str) -> UmmahResult<Bytes> {
    print!("{}...\r", progress_message);
    std::io::stdout()
        .flush()
        .map_err(|x| UmmahError::Unknown(Box::new(x)))?;
    let data = request_file(url).await?;
    print!("{:<32}\r", "");
    std::io::stdout()
        .flush()
        .map_err(|x| UmmahError::Unknown(Box::new(x)))?;
    Ok(data)
}

async fn request_file<T: IntoUrl>(url: T) -> UmmahResult<Bytes> {
    let response = reqwest::get(url)
        .await
        .map_err(|x| UmmahError::Unknown(Box::new(x)))?;
    let content = response
        .bytes()
        .await
        .map_err(|x| UmmahError::Unknown(Box::new(x)))?;
    Ok(content)
}
