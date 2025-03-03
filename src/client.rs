#![warn(missing_docs)]
use get_pass::get_password;
use reqwest::{
    header::{HeaderMap, ACCEPT, AUTHORIZATION, CONTENT_TYPE},
    Client as ReqClient, ClientBuilder, Url,
};

use crate::{config::Config, error::DoliApiClientError};
/// Client to interact with Dolibarr API
/// contains a reqwest::Client with predefined headers and an Url.
#[derive(Clone, Debug)]
pub struct Client {
    pub(crate) client: ReqClient,
    pub(crate) uri: Url,
}

impl Client {
    /// construct a doli-client-api-rs Client struct to be used with every high level functions.
    pub fn new(config: Config) -> Result<Client, DoliApiClientError> {
        let token = get_password(&config.token).map_err(|_| DoliApiClientError::InvalidToken)?;
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
            uri: config.url,
        })
    }
}
