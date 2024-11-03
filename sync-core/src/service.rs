use crate::route::Route;

pub struct Service {
    pub routes: Vec<Route>,
}

impl Service {
    pub fn new() -> Self {
        Self { routes: Vec::new() }
    }
}
