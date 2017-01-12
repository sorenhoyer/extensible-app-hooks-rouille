use std::sync::{Mutex};
use app_model::Route;

lazy_static! {
	pub static ref ROUTE_REGISTRY: Mutex<RouteRegistry> = Mutex::new(RouteRegistry::new());
}

pub struct RouteRegistry {
    pub routes: Vec<Box<Route>>,
}

impl RouteRegistry {
    pub fn new() -> RouteRegistry {
        RouteRegistry {
            routes: Vec::new()
        }
    }

    /// Adds a route to the stack
    pub fn add_route(&mut self, route: Route) {
        self.routes.push(Box::new(route));
    }
}