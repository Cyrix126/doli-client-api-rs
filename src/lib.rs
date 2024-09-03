#![warn(missing_docs)]
#![doc = include_str!("../README.md")]
use dolibarr_lib_rs::product::Product;
use error::DoliApiClientError;
use reqwest::{
    header::{HeaderMap, ACCEPT, AUTHORIZATION, CONTENT_TYPE},
    Client as ReqClient, ClientBuilder, StatusCode, Url,
};
use serde::{Deserialize, Serialize};
use serde_json::Value;
/// errors module for this crate, transmitting error made by misuse of the client at runtime or issue with the Dolibarr API backend.
pub mod error;
/// Client to interact with Dolibarr API
/// contains a reqwest::Client with predefined headers and an Url.
#[derive(Clone, Debug)]
pub struct Client {
    client: ReqClient,
    uri: Url,
}

impl Client {
    /// construct a doli-client-api-rs Client struct to be used with every high level functions.
    pub fn new(token: &str, uri: Url) -> Client {
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
    /// get product details from id
    pub async fn get_product_from_id(&self, id: u32) -> Result<Product, DoliApiClientError> {
        let url = [self.uri.as_str(), "/products/", &id.to_string()].concat();
        let resp = self.client.get(url).send().await?;
        let product = resp.json::<Product>().await?;
        Ok(product)
    }

    /// Return the barcode of a product if it exist.
    /// Return an error if it doesn't exist.
    pub async fn get_barcode_from_id(&self, id: u32) -> Result<Option<String>, DoliApiClientError> {
        let url = [self.uri.as_str(), "/products/", &id.to_string()].concat();
        let resp = self.client.get(url).send().await?;
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
    pub async fn get_label_from_id(&self, id: u32) -> Result<String, DoliApiClientError> {
        let url = [self.uri.as_str(), "/products/", &id.to_string()].concat();
        let resp = self.client.get(url).send().await?;
        let status = resp.status();
        let json = resp.json::<Value>().await?;
        product_exist(&status, &json)?;

        Ok(json
            .get("label")
            .expect("field label is always present for product")
            .to_string())
    }
    /// get all existents ids of the product table
    pub async fn get_all_products(&self) -> Result<Vec<u32>, DoliApiClientError> {
        let url = [self.uri.as_str(), "/products?limit=0&ids_only=true"].concat();
        let resp = self.client.get(url).send().await?;
        let json = resp.json::<Vec<String>>().await?;
        let mut ids = vec![];
        for id in json {
            ids.push(id.parse().unwrap());
        }
        Ok(ids)
    }
    /// return personal data of client from email.
    /// Can be used to retrieve the id.
    pub async fn get_data_from_email(
        &self,
        email: String,
    ) -> Result<CustomerData, DoliApiClientError> {
        let url = [self.uri.as_str(), "thirdparties/email/", &email].concat();
        let resp = self.client.get(url).send().await?;
        let json = resp.json::<CustomerData>().await?;
        Ok(json)
    }
    /// update data of client
    /// Can be used to retrieve the id.
    pub async fn update_customer_data(
        &self,
        id: u32,
        data: &CustomerData,
    ) -> Result<(), DoliApiClientError> {
        let url = format!("{}thirdparties/{}", self.uri, id);
        self.client
            .put(url)
            .json(data)
            .send()
            .await?
            .error_for_status()?;
        Ok(())
    }
    /// return personal data from id
    pub async fn customer_data_from_id(&self, id: u32) -> Result<CustomerData, DoliApiClientError> {
        let url = format!("{}/thirdparties/{id}", self.uri);
        let resp = self.client.get(url).send().await?;
        let json = resp.json::<CustomerData>().await?;
        Ok(json)
    }
    /// return orders of a client
    pub async fn get_order_from_id(&self, id: u32) -> Result<Vec<Document>, DoliApiClientError> {
        let url = format!("{}/orders?thirdparty_ids={id}", self.uri);
        let resp = self.client.get(url).send().await?;
        let json = resp.json::<Vec<Document>>().await?;
        Ok(json)
    }
    /// return invoices of a client
    pub async fn get_invoices_from_id(&self, id: u32) -> Result<Vec<Document>, DoliApiClientError> {
        let url = format!("{}/invoices?thirdparty_ids={id}", self.uri);
        let resp = self.client.get(url).send().await?;
        let json = resp.json::<Vec<Document>>().await?;
        Ok(json)
    }
}
/// Contact information of custommer
#[derive(Deserialize, Serialize)]
pub struct CustomerData {
    /// the id present in the database of the thirdparty
    pub id: u32,
    /// Full name of the customer
    pub name: String,
    /// Phone number
    pub phone: String,
    /// Email address
    pub email: String,
    /// Street number and Street name
    pub address: String,
    /// Postal code
    pub zip: String,
    /// Town
    pub town: String,
}
#[derive(Deserialize)]
/// Document can be an order or an invoice. They share the same attributes.
pub struct Document {
    /// id of document in the database
    pub id: u32,
    #[serde(rename(deserialize = "ref"))]
    /// reference publicly available
    pub reference: String,
    #[serde(rename(deserialize = "total_ht"))]
    /// total price HT
    pub price: f32,
    /// the lines composing the document
    pub lines: Vec<Line>,
}

#[derive(Deserialize)]
/// Line of a document.
pub struct Line {
    /// id of line in dolibarr database
    pub id: u32,
    /// qty of the product in this line
    pub qty: u32,
    /// id of product
    pub fk_product: u32,
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
