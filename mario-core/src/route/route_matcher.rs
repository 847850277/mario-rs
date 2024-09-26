use crate::route::request::Request;
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


    pub(crate) fn match_route(&self,req: &Request) -> Option<&Route> {
        for route in &self.routes {
            if route.http_method == req.head.method && route.path == req.head.uri.path() {
                return Some(route);
            }
        }
        None
    }

}