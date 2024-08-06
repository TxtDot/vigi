use crate::types::VigiError;
use insecure_gemini_client::insecure_gemini_client_config;
use mime::Mime;
use reqwest::header::CONTENT_TYPE;
use url::Url;

use super::{insecure_gemini_client, ReqResult};

pub async fn process_url(url: Url) -> Result<ReqResult, VigiError> {
    let result = match url.scheme() {
        "http" | "https" => process_http(url.to_string()).await?,
        "gemini" => process_gemini(url.to_string()).await?,
        _ => Err(VigiError::UnsupportedProtocol)?,
    };

    Ok(result)
}

async fn process_http(url: String) -> Result<ReqResult, VigiError> {
    let res = reqwest::get(&url).await.map_err(|_| VigiError::Network)?;

    let mime_type = {
        match res.headers().get(CONTENT_TYPE) {
            Some(header) => match header.to_str() {
                Ok(mime) => mime.to_owned(),
                Err(_) => "text/plain".to_owned(),
            },
            None => "text/plain".to_owned(),
        }
    }
    .parse::<Mime>()
    .map_err(|_| VigiError::InvalidMimeType)?;

    Ok((
        mime_type,
        res.bytes().await.map_err(|_| VigiError::Network)?.into(),
    ))
}

async fn process_gemini(url: String) -> Result<ReqResult, VigiError> {
    let mut res = tokio_gemini::Client::from(insecure_gemini_client_config())
        .request(&url)
        .await
        .map_err(|_| VigiError::Network)?;

    let mime_type = res.mime().map_err(|_| VigiError::InvalidMimeType)?;

    Ok((
        mime_type,
        res.bytes().await.map_err(|_| VigiError::Network)?.into(),
    ))
}
