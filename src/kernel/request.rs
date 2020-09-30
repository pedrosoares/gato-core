use std::collections::HashMap;

pub struct Request {
    method: String,
    uri: String,
    headers: HashMap<String, String>,
    params: HashMap<String, String>,
    body: HashMap<String, String>,
}

impl Request {
    pub fn new(method: String, uri: String, headers: HashMap<String, String>) -> Self {
        return Request {method, uri, headers, params: HashMap::new(), body: HashMap::new()};
    }

    pub fn get_uri(&self) -> String {
        return self.uri.clone();
    }

    pub fn add_params(&mut self, params: HashMap<String, String>) {
        self.params = params;
    }

    pub fn add_body(&mut self, body: HashMap<String, String>) {
        self.body = body;
    }

    pub fn get_method(&self) -> String {
        return self.method.clone();
    }

    pub fn get(&self, name: &str, default_value: &str) -> String {
        if self.body.contains_key(name) {
            return self.body.get(&name.to_string()).unwrap().clone();
        }
        return default_value.to_string();
    }

    pub fn get_param(&self, name: &str, default_value: &str) -> String {
        if self.params.contains_key(name) {
            return self.params.get(&name.to_string()).unwrap().clone();
        }
        return default_value.to_string();
    }

    pub fn all(&self) -> HashMap<String, String> {
        return self.body.clone();
    }

    pub fn get_headers(&self) -> HashMap<String, String> {
        return self.headers.clone();
    }

}
