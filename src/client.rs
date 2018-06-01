use reqwest;

static API_HOST: &'static str = "https://api.etherscan.io/api";

#[derive(Clone)]
pub struct Client {
  api_key: String,
}

impl Client {
  pub fn new(api_key: String) -> Self {
    Client { api_key: api_key }
  }

  pub fn get(&self, endpoint: String) -> Result<reqwest::Response, reqwest::Error> {
    let mut url = format!("{}?{}", API_HOST, endpoint);
    url.push_str(format!("&apikey={}", self.api_key).as_str());
    let client = reqwest::Client::new();
    let response = client.get(url.as_str()).send();
    return response;
  }
}
