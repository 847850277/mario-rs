use crate::route::route::Route;

pub struct RouteMatcher {
    routes: Vec<Route>,
}

impl RouteMatcher {
    pub fn new(routes: Vec<Route>) -> Self {
        Self {
            routes,
        }
    }

    pub fn set_routes(&mut self, routes: Vec<Route>) {
        self.routes = routes;
    }



}