use std::path::PathBuf;

use reqwest::Url;
use serde::{Deserialize, Serialize};
/// Configuration struct
#[derive(Serialize, Deserialize)]
pub struct Config {
    /// user of Dolibarr DB
    pub db_username: String,
    /// path file of where the password for the Dolibarr DB is stored in pass
    pub db_pass_file: PathBuf,
    /// any login info will be overwritten by the username/pass file configured
    pub url: Url,
    /// pass file for the token used with the API of Dolibarr
    pub token: PathBuf,
}

impl Default for Config {
    fn default() -> Self {
        Self {
            db_username: String::default(),
            db_pass_file: PathBuf::default(),
            url: Url::parse("https://example.net").unwrap(),
            token: PathBuf::default(),
        }
    }
}
