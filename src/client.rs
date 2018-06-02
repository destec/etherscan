use reqwest;
use reqwest::{Response, StatusCode};
use serde::{Deserialize};
use serde_json;
use std::io::Error;

static API_HOST: &'static str = "https://api.etherscan.io/api";

#[derive(Clone)]
pub struct Client {
  api_key: String,
}

impl Client {
  pub fn new(api_key: String) -> Self {
    Client { api_key: api_key }
  }

  pub fn get<T>(&self, endpoint: String) -> T where T: Into<Deserialize>{
    let mut url = format!("{}?{}", API_HOST, endpoint);
    url.push_str(format!("&apikey={}", self.api_key).as_str());
    let response = reqwest::get(url.as_str()).unwrap();
    let result = self.process(response).unwrap();
    let object: T = serde_json::from_str(&result).unwrap();
    return object;
  }

  fn process(&self, response: Response) -> Result<String, Error> {
    match response.status() {
      StatusCode::Ok => {
        let body = response.text();
        match body {
          Ok(result) => return Ok(result),
          // Err(e) => Error::from_str(format!("{:?}", e)),
        }
      }
      StatusCode::RequestTimeout => {
        return Ok(format!("{}", "timeout"));
      }
      _ => {
        return Ok(format!("{:?}", response));
      }
    }
  }
}
