use std::sync::Arc;
use crate::route::route::Route;

pub struct Routers{
    routes: Vec<Arc<Route>>,
}
impl Routers {
    pub fn new() -> Self {
        Self {
            routes: Vec::new(),
        }
    }

    pub fn add_route(&mut self, route: Arc<Route>) {
        self.routes.push(route.clone());
        println!("Add Route: {:?}", route);
    }

    pub fn remove_route(&mut self, route_to_remove: Arc<Route>) {
        self.routes.retain(|route| !Arc::ptr_eq(route, &route_to_remove));
    }

    pub fn get_routes(&self) -> &Vec<Arc<Route>> {
        &self.routes
    }

    pub fn set_routes(&mut self, routes: Vec<Arc<Route>>) {
        self.routes = routes;
    }
}

