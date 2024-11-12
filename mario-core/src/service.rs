use crate::route::Router;

#[derive(Default, Clone)]
pub struct Service {
    pub routes: Router,
}

impl Service {
    pub fn new() -> Self {
        Service {
            routes: Router::default(),
        }
    }

    // set routes
    pub fn set_routes(&mut self, routes: Router) {
        self.routes = routes;
    }

    // get routes
    pub fn get_routes(&self) -> &Router {
        &self.routes
    }
}
