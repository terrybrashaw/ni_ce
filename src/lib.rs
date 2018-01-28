#![feature(try_from)]
#![feature(associated_type_defaults)]
#![feature(crate_in_paths)]
#![allow(warnings)]

#[macro_use] extern crate failure;
#[macro_use] extern crate serde_derive;
extern crate base64;
extern crate chrono;
extern crate decimal;
extern crate hex;
extern crate hmac;
extern crate serde;
extern crate serde_json;
extern crate sha2;
extern crate url;
extern crate reqwest;
extern crate tungstenite;
extern crate uuid;

pub mod api;
// pub mod gemini;
// pub mod gdax;
pub mod liqui;
mod model;
mod status;

pub use model::*;

pub trait RestExchange: std::fmt::Debug {
    fn place_order(&mut self, order: NewOrder) -> Order;
    fn balances(&mut self) -> Vec<Balance>;
    fn orders(&mut self, product: CurrencyPair) -> Vec<Order>;
    fn orderbook(&mut self, product: CurrencyPair) -> Orderbook;
    // fn exchange(&mut self) -> MutexGuard<Exchange>;
}

// use url::Url;

// pub type Header = (&'static str, String);

// #[derive(Clone, Debug, Eq, Hash, PartialEq)]
// pub enum Method {
// 	Post,
// 	Get,
// }

// #[derive(Clone, Debug, Eq, Hash, PartialEq)]
// pub struct Request {
//     pub address: Url,
//     pub headers: Option<Vec<Header>>,
//     pub method: Method,
//     pub payload: Option<String>,
// }