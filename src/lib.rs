#[macro_use] extern crate serde_derive;
extern crate base64;
extern crate chrono;
extern crate decimal;
extern crate hex;
extern crate ring;
extern crate serde_json;
extern crate serde;
extern crate url;

pub mod gemini;
pub mod gdax;

use url::Url;

pub type Header = (&'static str, String);

#[derive(Clone, Debug, Eq, Hash, PartialEq)]
pub struct ConnectionInfo {
    pub address: Url,
    pub headers: Option<Vec<Header>>,
}