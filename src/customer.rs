use dolibarr_lib_rs::customer::CustomerData;

use crate::{client::Client, error::DoliApiClientError};

impl Client {
    /// return personal data from id
    pub async fn customer_data_from_id(&self, id: u32) -> Result<CustomerData, DoliApiClientError> {
        let url = format!("{}/thirdparties/{id}", self.uri);
        let resp = self.client.get(url).send().await?;
        let json = resp.json::<CustomerData>().await?;
        Ok(json)
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
    /// return personal data of client from email.
    /// Can be used to retrieve the id.
    pub async fn get_data_from_id(&self, id: u32) -> Result<CustomerData, DoliApiClientError> {
        let url = [self.uri.as_str(), "thirdparties/", &id.to_string()].concat();
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
}
