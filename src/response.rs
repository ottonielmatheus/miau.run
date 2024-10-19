use std::{collections::HashMap, fs, io::Write};

use crate::request::{HttpVersion, Meowquest};

static ABSOLUTE_PAGES_PATH: &'static str = "./src/pages";

#[allow(dead_code)]
#[derive(Debug)]
pub enum StatusCode {
  Ok,
  NotFound,
  Accepted
}

#[allow(dead_code)]
#[derive(Debug)]
pub struct Meowsponse {
  status: StatusCode,
  headers: HashMap<String, String>,
  body: Option<String>,
  pub request: Meowquest
}

#[allow(dead_code)]
impl Meowsponse {
  pub fn new(request: Meowquest) -> Meowsponse {
    Meowsponse {
      status: StatusCode::Ok,
      headers: HashMap::new(),
      body: None,
      request
    }
  }

  pub fn status(&mut self, status: StatusCode) -> &mut Meowsponse {
    self.status = status;
    self
  }

  fn send(&mut self, body: String) {
    if !body.is_empty() {
      self.headers.insert(String::from("Content-Length"), body.len().to_string());
    }

    let version = match self.request.http_version {
      HttpVersion::Two => "HTTP/2.0".to_string(),
      _ => "HTTP/1.1".to_string()
    };

    let status = match self.status {
      StatusCode::Ok => "200 Ok",
      StatusCode::NotFound => "404 Not Found",
      StatusCode::Accepted => "202 Accepted"
    };

    let mut headers = String::new();
    for (key, value) in &self.headers {
      headers += format!("{key}: {value}").as_str();
    }

    let res_buff = format!("{version} {status}\r\n{headers}\r\n\r\n{}", body);

    self.request.stream.write(res_buff.as_bytes()).unwrap();
    self.request.stream.flush().unwrap();
  }

  pub fn html(&mut self, path: &str) {
    let html_buff = fs::read_to_string(format!("{}/{}", ABSOLUTE_PAGES_PATH, path)).unwrap();
    self.send(html_buff);
  }

  pub fn text(&mut self, text: String) {
    self.send(text);
  }
}