use dalet::{daletl::ToDaletlPage, typed::Tag::*};
use mime::Mime;

use crate::types::{VigiError, VigiOutput};

pub async fn process_data(mime: Mime, data: Vec<u8>) -> Result<VigiOutput, VigiError> {
    let result = match (mime.type_().as_str(), mime.subtype().as_str()) {
        ("text", "plain") => {
            process_text(String::from_utf8(data).map_err(|_| VigiError::TextIsNotUtf8)?).await
        }
        // ("text", "gemini") => {
        //     process_text(String::from_utf8(data).map_err(|_| VigiError::TextIsNotUtf8)?).await
        // }
        _ => Err(VigiError::UnsupportedMimeType)?,
    };

    Ok(result)
}

async fn process_text(data: String) -> VigiOutput {
    let mut truncated = data.clone();
    truncated.truncate(50);

    VigiOutput::new(truncated, vec![El(data.into())].to_dl_page())
}
