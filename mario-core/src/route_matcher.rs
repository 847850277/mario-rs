use crate::request::Request;
use crate::route::Route;
use std::sync::Arc;

pub struct RouteMatcher {
    routes: Arc<Vec<Route>>,
}

impl RouteMatcher {
    pub fn new(routes: Arc<Vec<Route>>) -> Self {
        Self { routes }
    }

    pub fn set_routes(&mut self, routes: Arc<Vec<Route>>) {
        self.routes = routes;
    }

    pub(crate) fn match_route(&self, req: &Request) -> Option<&Route> {
        self.routes.as_ref().iter().find(|&route| {
            route.http_method == req.head.method && route.path == req.head.uri.path()
        })
    }
}
