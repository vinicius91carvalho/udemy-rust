//! # Online Business
//! This is a rust library for online store
mod customer;
mod order;
mod product;

pub use customer::Customer;
pub use order::Order;
pub use product::{Category, Product};
