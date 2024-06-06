use thiserror::Error;

#[derive(Error, Debug)]
pub enum DoliApiClientError {
    #[error("product requested does not exist")]
    IdDoesNotExist,
    #[error(transparent)]
    ReqwestError(#[from] reqwest::Error),
}
