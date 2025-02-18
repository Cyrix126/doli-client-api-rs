use dolibarr_lib_rs::document::Document;

use crate::error::DoliApiClientError;

impl crate::client::Client {
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
