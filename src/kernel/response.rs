use std::collections::HashMap;
use serde_json::Value;

pub struct Response {
    code: i32,
    headers: HashMap<String, String>,
    body: String
}

impl Response {

    fn clone(&self) -> Self {
        return Response{
            code: self.code.clone(), headers: self.headers.clone(), body: self.body.clone()
        };
    }

    pub fn new() -> Self {
        return Response{ code: 200, headers: HashMap::new(), body: String::new() };
    }

    pub fn json(&mut self, body: Value) -> Self {
        self.headers.insert("Content-Type".to_owned(), "application/json".to_owned());
        self.body = body.to_string();
        return self.clone();
    }

    pub fn raw(&mut self, body: &str) -> Self {
        let content_type = "Content-Type".to_owned();
        if !self.headers.contains_key(&content_type) {
            self.headers.insert(content_type, "text/html".to_owned());
        }
        self.body = body.to_string();
        return self.clone();
    }

    pub fn header(&mut self, name: &str, value: &str) -> &mut Self {
        self.headers.insert(name.to_string(), value.to_string());
        return self;
    }

    pub fn status(&mut self, code: i32) -> &mut Self {
        self.code = code;
        return self;
    }

    pub fn get_body(&self) -> String {
        return self.body.clone();
    }

    pub fn get_headers(&self) -> HashMap<String, String> {
        return self.headers.clone();
    }

    pub fn get_code(&self) -> i32 {
        return self.code;
    }

}
