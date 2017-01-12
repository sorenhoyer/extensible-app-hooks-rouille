#[macro_use]
extern crate rouille;

use rouille::Request;
use rouille::Response;

use std::cmp::Ordering;

//#[derive(Clone)]
pub struct Route {
	pub method: &'static str,
	pub path: &'static str,
	pub priority: i32,
	pub func: fn (_: &Request) -> Response
}

impl Route {
    pub fn new(m: &'static str, u: &'static str, p: i32, f: fn (_: &Request)  -> Response) -> Route {
        Route {
        	method: m,
        	path: u,
        	priority: p,
        	func: f
        }
    }
}

impl<'a> PartialOrd for Route {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        //Some(Ordering::Equal)
        Some(self.cmp(other))
    }
}

impl<'a> PartialEq for Route {
    fn eq(&self, other: &Self) -> bool {
        (self.priority) == (other.priority)
    }
}

impl<'a> Ord for Route {
    fn cmp(&self, other: &Self) -> Ordering {
        (self.priority).cmp(&(other.priority))
    }
}

impl<'a> Eq for Route { }