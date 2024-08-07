use crate::types::{VigiError, VigiOutput, VigiState};
use bytes::Bytes;
use mime::Mime;
use url::Url;

mod process_data;
mod process_url;

use process_data::process_data;
use process_url::process_url;

type ReqResult = (Mime, Bytes);

pub async fn process_input(input: &String, state: &VigiState) -> Result<VigiOutput, VigiError> {
    let parsed = Url::parse(input);

    let (mime, data) = match parsed {
        Ok(url) => process_url(url, state).await?,
        Err(_) => Err(VigiError::Network)?,
    };

    let result = process_data(mime, data).await?;

    Ok(result)
}
