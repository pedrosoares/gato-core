use std::env::Args;
use crate::kernel::{HttpCore, RouterHandler, Router, Logger};
use crate::kernel::http_core::HttpCoreHandler;
use crate::kernel::application::service_provider::ServiceProvider;
use crate::kernel::logger::FileLogger;

pub struct Application {
    args: Args,
    service_provider: ServiceProvider,
    http_core: Option<Box<dyn HttpCore>>,
    bootstrap_handler: &'static dyn Fn(&mut ServiceProvider) -> ()
}

impl Application {

    pub fn new(args: Args, bootstrap_handler: &'static dyn Fn(&mut ServiceProvider) -> ()) -> Application {
        return Application {
            args, service_provider: ServiceProvider::new(), http_core: Option::None, bootstrap_handler
        };
    }

    pub fn run(&mut self) {
        // Set Default Logger Driver
        Logger::set_driver(Box::new(FileLogger { }));
        (self.bootstrap_handler)(&mut self.service_provider); // Load the application configuration
        self.service_provider.boot(); // Load all providers

        //Load routers into the memory
        let router : &Box<dyn Router> = RouterHandler::get_driver();
        router.boot();

        //Handle the Request made by the client thought the driver
        let http_core : &Box<dyn HttpCore> = HttpCoreHandler::get_driver();
        http_core.handle();
    }

    pub fn get_args(&self) -> &Args {
        return &self.args;
    }

    pub fn get_http_core(&mut self) -> Option<Box<dyn HttpCore>> {
        return self.http_core.take();
    }

}
