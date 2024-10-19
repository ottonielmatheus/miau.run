use std::collections::HashMap;

use crate::request::{Meowquest, HttpMethod};
use crate::response::Meowsponse;

type Lambda = &'static dyn Fn(&mut Meowsponse);

pub struct Router {
  mapper: HashMap<(String, HttpMethod), Box<Lambda>>
}

impl Router {
  pub fn new() -> Router {
    Router {
      mapper: HashMap::new()
    }
  }

  pub fn get(&mut self, uri: &str, action: Lambda) -> &mut Router {
    self.mapper.insert((uri.into(), HttpMethod::Get), Box::new(action));
    self
  }

  pub fn post(&mut self, uri: &str, action: Lambda) -> &mut Router {
    self.mapper.insert((uri.into(), HttpMethod::Post), Box::new(action));
    self
  }

  pub fn run(&self, request: Meowquest) {
    let mut response = Meowsponse::new(request);
    if let Some(action) =
      self.mapper.get(&(response.request.path.clone(), response.request.method)) {
      action(&mut response);
      return;
    }
  }
}