use thiserror::Error;

/// Possible errors from the Dolibarr API
#[derive(Error, Debug)]
pub enum DoliApiClientError {
    /// the product with the id provided does not exist
    #[error("product requested does not exist")]
    IdDoesNotExist,
    /// The API response status code is an error.
    #[error(transparent)]
    ReqwestError(#[from] reqwest::Error),
}
