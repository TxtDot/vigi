use bytes::Bytes;
use dalet::{
    daletl::DlPage,
    parsers::gemtext::parse_gemtext,
    typed::{ResolveTitle, Tag::*},
};
use mime::Mime;
use std::str;

use crate::{
    types::{VigiError, VigiOutput},
    utils::truncate,
};

pub async fn process_data(mime: Mime, data: Bytes) -> Result<VigiOutput, VigiError> {
    let result = match (mime.type_().as_str(), mime.subtype().as_str()) {
        ("text", "plain") => {
            process_text(str::from_utf8(&data).map_err(|_| VigiError::InvalidCharset)?).await
        }
        ("text", "gemini") => {
            process_gemini(str::from_utf8(&data).map_err(|_| VigiError::InvalidCharset)?).await?
        }
        _ => Err(VigiError::UnsupportedMimeType)?,
    };

    Ok(result)
}

async fn process_text(data: &str) -> VigiOutput {
    let mut truncated = data.to_owned();
    truncated.truncate(50);

    VigiOutput::new(truncated, vec![El(data.into()).into()])
}

async fn process_gemini(data: &str) -> Result<VigiOutput, VigiError> {
    let truncated = truncate(data, 20);

    let page = parse_gemtext(data).map_err(|_| VigiError::Parse)?;
    let title = page.resolve_title().unwrap_or(truncated.to_owned());

    Ok(VigiOutput::new(title.to_owned(), DlPage::from(page).data))
}
