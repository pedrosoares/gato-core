use std::collections::HashMap;
use serde_json::Value;

pub struct RequestBuilder {
    request: Request
}

impl RequestBuilder {

    pub fn new() -> Self {
        return RequestBuilder {
            request: Request::new()
        };
    }

    pub fn add_method(&mut self, method: String) {
        self.request.method = method;
    }

    pub fn add_uri(&mut self, uri: String) {
        self.request.uri = uri;
    }

    pub fn add_param(&mut self, name: String, value: String) {
        self.request.params.insert(name, value);
    }

    pub fn add_params(&mut self, params: HashMap<String, String>) {
        self.request.params = params;
    }

    pub fn add_header(&mut self, name: String, value: String) {
        self.request.headers.insert(name, value);
    }

    pub fn add_headers(&mut self, headers: HashMap<String, String>) {
        self.request.headers = headers;
    }

    pub fn add_body(&mut self, body: String) {
        self.request.body = body;
    }

    pub fn get_request(&self) -> Request {
        return self.request.clone();
    }

}

pub struct Request {
    method: String,
    uri: String,
    headers: HashMap<String, String>,
    params: HashMap<String, String>,
    body: String,
    body_json: Option<Value>
}

impl Request {

    fn new() -> Self {
        return Request {
            method: String::new(),
            uri: String::new(),
            body: String::new(),
            headers: HashMap::new(),
            params: HashMap::new(),
            body_json: None
        };
    }

    fn clone(&self) -> Self {
        return Request {
            method: self.method.clone(),
            uri:  self.uri.clone(),
            body:  self.body.clone(),
            headers:  self.headers.clone(),
            params:  self.params.clone(),
            body_json: None
        };
    }

    pub fn get_uri(&self) -> String {
        return self.uri.clone();
    }

    pub fn get_method(&self) -> String {
        return self.method.clone();
    }

    // TODO add multipart/form-data handler
    pub fn get(&mut self, field: String) -> Value {
        let mut json = self.json();
        let fields = field.split(".");
        for field in fields {
            json = json[field].clone();
        }
        return json;
    }

    pub fn json(&mut self) -> Value {
        if self.body_json.is_none() {
            let v: Value = serde_json::from_str(self.body.as_str()).unwrap();
            self.body_json = Some(v);
        }
        return self.body_json.as_ref().unwrap().clone();
    }

    pub fn get_param(&self, name: &str, default_value: &str) -> String {
        if self.params.contains_key(name) {
            return self.params.get(&name.to_string()).unwrap().clone();
        }
        return default_value.to_string();
    }

    pub fn get_params(&self) -> HashMap<String, String> {
        return self.params.clone();
    }

    pub fn get_headers(&self) -> HashMap<String, String> {
        return self.headers.clone();
    }

    pub fn get_header(&self, header: String) -> String {
        return self.headers[&header].clone();
    }

}
