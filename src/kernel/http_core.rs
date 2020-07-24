use crate::kernel::singleton::Singleton;
use crate::kernel::singleton::new_singleton;
use std::collections::HashMap;

pub struct HttpCoreHandler { }

impl HttpCoreHandler {
    pub fn set_driver(driver: Box<dyn HttpCore>) {
        unsafe {
            HTTP_CORE.set_instance(driver);
        }
    }

    // TODO Implement Option Validation for Clean Message
    pub fn get_driver() -> &'static Box<dyn HttpCore> {
        unsafe {
            return HTTP_CORE.get_instance();
        }
    }
}

pub trait HttpCore {
    fn handle(&self);
    fn get_request_headers(&self) -> HashMap<String, String>;
    fn get_post_data(&self) -> String;
}

static mut HTTP_CORE : Singleton<Box<dyn HttpCore>> = new_singleton();
