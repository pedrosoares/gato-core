use crate::kernel::singleton::{Singleton, new_singleton};
use crate::kernel::{Request, Response};

pub struct RouterHandler {

}

impl RouterHandler {
    pub fn set_driver(driver: Box<dyn Router>) {
        unsafe {
            ROUTER_CORE.set_instance(driver);
        }
    }

    // TODO Implement Option Validation for Clean Message
    pub fn get_driver() -> &'static Box<dyn Router> {
        unsafe {
            return ROUTER_CORE.get_instance();
        }
    }
}

pub trait Router {
    fn boot(&self) -> ();
    fn handle(&self, request: &mut Request) -> Response;
}

static mut ROUTER_CORE : Singleton<Box<dyn Router>> = new_singleton();

