#![warn(missing_docs)]
#![doc = include_str!("../README.md")]
/// client module to create an http client that can interact with the dolibarr API
pub mod client;
/// customer module for CRUD operations on customers
pub mod customer;
/// errors module for this crate, transmitting error made by misuse of the client at runtime or issue with the Dolibarr API backend.
pub mod error;
/// customer module for CRUD operations on order
pub mod order;
/// customer module for CRUD operations on products
pub mod product;
