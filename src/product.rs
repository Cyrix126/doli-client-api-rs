use dolibarr_lib_rs::product::Product;
use reqwest::StatusCode;
use serde_json::Value;

use crate::{client::Client, error::DoliApiClientError};

impl Client {
    /// create a product, returns the id
    pub async fn create_product(&self, product: &Product) -> Result<u32, DoliApiClientError> {
        let url = [self.uri.as_str(), "/products/"].concat();
        let id = self
            .client
            .post(url)
            .json(product)
            .send()
            .await?
            .text()
            .await?
            .parse()
            .map_err(|_| DoliApiClientError::UnexpectedResponse)?;
        Ok(id)
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
    /// update a product
    pub async fn update_product(&self, product: &Product) -> Result<(), DoliApiClientError> {
        let url = [self.uri.as_str(), "/products/", &product.rowid.to_string()].concat();
        self.client.put(url).json(product).send().await?;
        Ok(())
    }
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
