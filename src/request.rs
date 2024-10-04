use std::io::Read;
use std::{collections::HashMap as HashMap, net::TcpStream};

#[derive(Debug)]
pub enum HttpVersion {
  One,
  Two
}

#[derive(Debug, Clone)]
enum Meowthod {
  Get
}

#[allow(dead_code)]
#[derive(Debug)]
pub struct Meowquest {
  pub stream: TcpStream,
  pub http_version: HttpVersion,
  pub method: Meowthod,
  pub path: String,
  pub headers: HashMap<String, String>,
  pub body: Option<String>,
}

impl Meowquest {
  pub fn new(mut stream: TcpStream) -> Meowquest {
    let mut parts = Meowquest::parse(&mut stream);

    let head: Vec<String> = parts.remove(0)
      .split_whitespace()
      .map(|s| s.to_string())
      .collect();

    let body = parts.pop();

    Meowquest {
      stream,
      method: Meowquest::get_meowthod(&head[0]),
      path: String::from(&head[1]),
      http_version: Meowquest::get_meow_version(&head[2]),
      headers: Meowquest::get_meaders(parts),
      body: Meowquest::get_mowdy(body)
    }
  }

  fn parse(stream: &mut TcpStream) -> Vec<String> {
    let mut buffer: [u8; 1024] = [0; 1024];
    stream.read(&mut buffer).unwrap();

    let req_buff = String::from_utf8_lossy(&buffer);

    req_buff
      .split_terminator("\r\n")
      .map(|s| s.to_string())
      .collect()
  }

  fn get_meow_version(raw_http_version: &String) -> HttpVersion {
    match raw_http_version.as_str() {
      "HTTP/1.1" => HttpVersion::One,
      "HTTP/2.0" => HttpVersion::Two,
      _ => panic!("Invalid http version")
    }
  }

  fn get_meowthod(raw_method: &String) -> Meowthod {
    match raw_method.as_str() {
      "GET" => Meowthod::Get,
      _ => panic!("Invalid method")
    }
  }

  fn get_meaders(raw_headers: Vec<String>) -> HashMap<String, String> {
    let mut headers: HashMap<String, String> = HashMap::new();

    for line in raw_headers {
      let line: Vec<&str> = line.split(": ").collect();
      if line[0].is_empty() || line[1].is_empty() {
        continue
      };

      headers.insert(line[0].to_string(), line[1].to_string());
    }

    headers
  }

  fn get_mowdy(body: Option<String>) -> Option<String> {
    match body {
      Some(v) => if v.starts_with('\n') { Some(v.to_string()) } else { None },
      None => None
    }
  }
}