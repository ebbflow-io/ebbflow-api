#[macro_use]
extern crate serde;
use rand::distributions::Alphanumeric;
use rand::prelude::*;

pub mod generatedmodels;

pub fn new_keyrequest_id() -> String {
    format!("{}", id(7))
}

fn id(len: usize) -> String {
    let mut rng = thread_rng();
    std::iter::repeat(())
        .map(|()| rng.sample(Alphanumeric))
        .filter(|c| c.is_ascii_hexdigit())
        .map(|c| c.to_ascii_lowercase())
        .take(len)
        .collect()
}