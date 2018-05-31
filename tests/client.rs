extern crate etherscan_rs;
extern crate reqwest;

use etherscan_rs::client::Client;

#[test]
fn it_works() {
  let client = Client::new("aaa".to_string());
  let resp = client
    .get("/module=stats&action=ethsupply&apikey=aaa".to_string())
    .unwrap();
  println!("{:?}", resp);
  assert_eq!(resp.status(), reqwest::StatusCode::BadRequest);
}
