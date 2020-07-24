pub trait Provider {
    fn boot(&self) -> ();
}

pub struct ServiceProvider {
    providers: Vec<Box<dyn Provider>>,
}

impl ServiceProvider {

    pub fn new() -> ServiceProvider {
        return ServiceProvider{ providers: Vec::new() };
    }

    pub fn boot(&self) -> () {
        for provider in self.providers.iter() {
            provider.boot();
        }
    }

    pub fn register_provider(&mut self, provider: Box<dyn Provider>) -> () {
        self.providers.push(provider);
    }
}