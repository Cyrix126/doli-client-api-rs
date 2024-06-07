#![warn(missing_docs)]
#![doc = include_str!("../README.md")]
use error::DoliApiClientError;
use reqwest::{
    header::{HeaderMap, ACCEPT, AUTHORIZATION, CONTENT_TYPE},
    Client as ReqClient, ClientBuilder, StatusCode, Url,
};
use serde_json::Value;
/// errors module for this crate, transmitting error made by misuse of the client at runtime or issue with the Dolibarr API backend.
pub mod error;
/// Client to interact with Dolibarr API
/// contains a reqwest::Client with predefined headers and an Url.
#[derive(Clone)]
pub struct Client {
    client: ReqClient,
    uri: Url,
}

/// construct a doli-client-api-rs Client struct to be used with every high level functions.
pub fn client_doli(token: &str, uri: &Url) -> Client {
    let mut headers = HeaderMap::new();
    headers.insert(
        ACCEPT,
        "application/json"
            .parse()
            .expect("can not parse value of header"),
    );
    headers.insert(
        AUTHORIZATION,
        token.parse().expect("can not parse value of header"),
    );
    headers.insert(
        CONTENT_TYPE,
        "application/json"
            .parse()
            .expect("can not parse value of header"),
    );
    Client {
        client: ClientBuilder::new()
            .default_headers(headers)
            .build()
            .expect("client can not be build"),
        uri,
    }
}

/// Use only with a client constructed with the function client_doli().
/// Return the barcode of a product if it exist.
/// Return an error if it doesn't exist.
pub async fn get_barcode_from_id(
    client: &Client,
    id: u32,
) -> Result<Option<String>, DoliApiClientError> {
    let url = [client.uri.as_str(), "/products/", &id.to_string()].concat();
    let resp = client.client.get(url).send().await?;
    let status = resp.status();
    let json = resp.json::<Value>().await?;
    product_exist(&status, &json)?;
    if let Some(value) = json.get("barcode") {
        if let Some(str) = value.as_str() {
            return Ok(Some(str.to_string()));
        }
    }
    Ok(None)
}
/// get the label of a product with the id.
pub async fn get_label_from_id(client: &Client, id: u32) -> Result<String, DoliApiClientError> {
    let url = [client.uri.as_str(), "/products/", &id.to_string()].concat();
    let resp = client.client.get(url).send().await?;
    let status = resp.status();
    let json = resp.json::<Value>().await?;
    product_exist(&status, &json)?;

    Ok(json
        .get("label")
        .expect("field label is always present for product")
        .to_string())
}

/// check a response status and body to verify if an the error of product not found from Dolibarr is present.
/// return the error IdDoesNotExist if that's the case.
fn product_exist(status: &StatusCode, json: &Value) -> Result<(), DoliApiClientError> {
    if status == &StatusCode::NOT_FOUND
        && json.get("error/message").is_some_and(|message| {
            message
                .as_str()
                .is_some_and(|msg| msg == "Not Found: Product not found")
        })
    {
        // id does not exist
        return Err(DoliApiClientError::IdDoesNotExist);
    }
    Ok(())
}
