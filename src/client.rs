#![warn(missing_docs)]
use reqwest::{
    header::{HeaderMap, ACCEPT, AUTHORIZATION, CONTENT_TYPE},
    Client as ReqClient, ClientBuilder, Url,
};

use crate::error::DoliApiClientError;
/// Client to interact with Dolibarr API
/// contains a reqwest::Client with predefined headers and an Url.
#[derive(Clone, Debug)]
pub struct Client {
    pub(crate) client: ReqClient,
    pub(crate) uri: Url,
}

impl Client {
    /// construct a doli-client-api-rs Client struct to be used with every high level functions.
    pub fn new(uri: Url) -> Result<Client, DoliApiClientError> {
        let token = uri
            .password()
            .ok_or(DoliApiClientError::InvalidToken)?
            .to_owned();
        let mut headers = HeaderMap::new();
        headers.insert(
            ACCEPT,
            "application/json"
                .parse()
                .expect("can not parse value of header"),
        );
        headers.insert(
            AUTHORIZATION,
            token
                .parse()
                .map_err(|_| DoliApiClientError::InvalidToken)?,
        );
        headers.insert(
            CONTENT_TYPE,
            "application/json"
                .parse()
                .expect("can not parse value of header"),
        );
        Ok(Client {
            client: ClientBuilder::new()
                .default_headers(headers)
                .build()
                .expect("client can not be build"),
            uri,
        })
    }
}
