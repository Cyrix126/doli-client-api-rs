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
    /// Dolibarr API always needs a token. If it is empty or contains invalid caracters, this error will be returned.
    #[error("Invalid token for Dolibarr API")]
    InvalidToken,
}
