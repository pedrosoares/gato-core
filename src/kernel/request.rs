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
    
    pub fn add_querystring(&mut self, querystring: HashMap<String, String>) {
        self.request.querystring = querystring;
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
    querystring: HashMap<String, String>,
    body: String
}

impl Request {

    fn new() -> Self {
        return Request {
            method: String::new(),
            uri: String::new(),
            body: String::new(),
            headers: HashMap::new(),
            querystring: HashMap::new(),
            params: HashMap::new()
        };
    }

    fn clone(&self) -> Self {
        return Request {
            method: self.method.clone(),
            uri:  self.uri.clone(),
            body:  self.body.clone(),
            headers:  self.headers.clone(),
            querystring:  self.querystring.clone(),
            params:  self.params.clone()
        };
    }

    pub fn get_uri(&self) -> String {
        return self.uri.clone();
    }

    pub fn get_method(&self) -> String {
        return self.method.clone();
    }

    // TODO add multipart/form-data handler
    pub fn get(&self, field: &str) -> Value {
        let mut json = self.json();
        let fields = field.split(".");
        for field in fields {
            json = json[field].clone();
        }
        return json;
    }

    pub fn json(&self) -> Value {
        return if self.body != "" {
            serde_json::from_str(self.body.as_str()).unwrap()
        } else {
            Value::Null
        };
    }

    pub fn get_query(&self, name: &str) -> Option<String> {
        if self.querystring.contains_key(name) {
            return Some(self.querystring.get(&name.to_string()).unwrap().clone());
        }
        return None;
    }
    
    pub fn get_querystring(&self) -> HashMap<String, String> {
        return self.querystring.clone();
    }

    pub fn get_param(&self, name: &str) -> Option<String> {
        if self.params.contains_key(name) {
            return Some(self.params.get(&name.to_string()).unwrap().clone());
        }
        return None;
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
