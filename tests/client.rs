extern crate etherscan;
extern crate reqwest;

use etherscan::client::Client;
use std::env;

static PSEUDO_KEY: &'static str = "123456";

#[test]
fn works_for_invalid_key() {
  let requestor = Client::new(PSEUDO_KEY.to_string());
  let resp = requestor
    .get("module=stats&action=ethsupply".to_string())
    .unwrap();
  assert_eq!(resp.status(), reqwest::StatusCode::Ok);
}

#[test]
fn works_for_valid_key() {
  let real_key: String = env::var("ETHERSCAN_APIKEY").unwrap();
  let requestor = Client::new(real_key.to_string());
  let resp = requestor
    .get("module=stats&action=ethsupply".to_string())
    .unwrap();
  assert_eq!(resp.status(), reqwest::StatusCode::Ok);
}
