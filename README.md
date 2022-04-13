# gato-core
This repo contains all generics needed to build a modular framework in rust.

## Service Provider
Is a trait responsible to modularize the framework, you can register customs ServiceProviders that will be
executed during the framework bootstrap, at this moment you can register new drivers or configure the environment, etc.

See SimpleRouter for example:
```
use crate::SimpleRouter;
use gato_core::kernel::Provider;
use gato_core::kernel::RouterHandler;

pub struct SimpleRouterServiceProvider { }
impl SimpleRouterServiceProvider {
    pub fn new() -> Box<Self> { Box::new(SimpleRouterServiceProvider {}) }
}

impl Provider for SimpleRouterServiceProvider {
    // This method is executed during boot only one time
    fn boot(&self) -> () {
        // Register the Router driver
        RouterHandler::set_driver(Box::new(SimpleRouter::new()));
    }
}
```
