use crate::kernel::Logger;

pub struct Singleton<T> {
    instance: Option<T>,
}

impl<T> Singleton<T> {

    #[allow(dead_code)]
    pub fn new() -> Singleton<T> {
        return new_singleton();
    }

    #[allow(dead_code)]
    pub fn is_none(&self) -> bool {
        return self.instance.is_none();
    }

    pub fn get_instance(&self) -> &T {
        match self.instance {
            Some(ref x) => {
                return x;
            },
            None => {
                Logger::error("Singleton is None");
                panic!("Singleton is None")
            }
        }
    }

    pub fn set_instance(&mut self, http_core: T) {
        self.instance = Some(http_core);
    }

}

pub const fn new_singleton<T>() -> Singleton<T>  {
    return Singleton { instance: None };
}
