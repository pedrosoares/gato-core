use std::collections::HashMap;
use serde_json::Value;

pub struct Response {
    code: i32,
    headers: HashMap<String, String>,
    body: String
}

impl Response {

    pub fn json(body: Value) -> Response {
        let mut headers: HashMap<String, String> = HashMap::new();
        headers.insert("Content-Type".to_string(), "application/json".to_string());
        return Response { code: 200, body: body.to_string(), headers };
    }

    pub fn new(body: &str) -> Self {
        let mut headers = HashMap::new();
        headers.insert("Content-Type".to_string(), "text/html".to_string());
        return Response{ code: 200, headers, body: body.to_string() };
    }

    pub fn get_body(&self) -> String {
        return self.body.clone();
    }

    pub fn add_header(&mut self, name: &str, value: &str) {
        self.headers.insert(name.to_string(), value.to_string());
    }

    pub fn get_headers(&self) -> HashMap<String, String> {
        return self.headers.clone();
    }

    pub fn get_code(&self) -> i32 {
        return self.code;
    }

    pub fn set_code(&mut self, code: i32) -> &mut Self {
        self.code = code;
        return self;
    }

    pub fn set_body(&mut self, body: &str) -> &mut Self {
        self.body = body.to_string();
        return self;
    }
}