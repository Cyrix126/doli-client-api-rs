# doli-client-api-rs
High-level Rust client library to interact with Dolibarr API

This library is not meant to fully support every endpoints of Dolibarr OpenAPI spec.  
It is opiniated and features will be added as needed.

Example:
```rust,ignore
// construct the client and give the url including the token of the api
let uri = Url::parse("https://:token@dolibarr.example.com/api/index.php")?;
let doli_client = doli_client_api_rs::Client::new(uri);

// get the barcode with the id
let barcode = doli_client.get_barcode_from_id(1094)?;
```
