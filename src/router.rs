use std::collections::HashMap;

use crate::{request::Meowquest, response::Meowsponse};

pub struct Router {
  mapper: HashMap<String, Box<dyn Fn(&mut Meowsponse)>>
}

impl Router {
  pub fn new() -> Router {
    Router {
      mapper: HashMap::new()
    }
  }

  pub fn get(&mut self, uri: &str, action: &'static dyn Fn(&mut Meowsponse)) -> &mut Router {
    self.mapper.insert(uri.into(), Box::new(action));
    self
  }

  pub fn run(&self, request: Meowquest) {
    let mut response = Meowsponse::new(request);
    if let Some(action) = self.mapper.get(&response.request.path) {
      action(&mut response);
      return
    }

    if let Some(action) = self.mapper.get("*") {
      action(&mut response)
    }
  }
}