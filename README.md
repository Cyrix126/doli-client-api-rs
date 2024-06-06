# doli-client-api-rs
High-level Rust client library to interact with Dolibarr API

This library is not meant to fully support every endpoints of Dolibarr OpenAPI spec.  
It is opiniated and features will be added as needed.

Example:
```rust,ignore
// construct the client and give the url
let token = "secret";
let uri = Url::parse("https://dolibarr.example.com/api/index.php")?;
let client = client_doli(token, uri);

// get the barcode with the id
let barcode = get_barcode_from_id(client, 1094)?;
```
