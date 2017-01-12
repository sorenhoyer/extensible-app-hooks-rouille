extern crate app_registry;
extern crate app_trait;
extern crate app_model;


extern crate serde_json;
#[macro_use]
extern crate rouille;

use std::collections::BTreeMap;

use app_registry::plugin_registry::PLUGIN_REGISTRY;
use app_registry::hook_registry::HOOK_REGISTRY;
use app_registry::route_registry::ROUTE_REGISTRY;
use app_trait::{Plugin};
use app_model::{Route};

use serde_json::value::{self, Value};

use rouille::Request;
use rouille::Response;

pub struct App{}

impl App {
    pub fn new() -> App {
        App {}
    }

    pub fn add_plugin<P: Plugin + 'static + Send>(plugin: P) {
        PLUGIN_REGISTRY.lock().unwrap().add_plugin(plugin);
    }

    pub fn start(&self) {
        for plugin in PLUGIN_REGISTRY.lock().unwrap().plugins.iter_mut() {
            plugin.init();
        }

        rouille::start_server("localhost:3000", move |request| {

            let mut result = None;

            let request = &request;

            // ignoring the GET parameters (everything after `?`)
            let request_url = request.url();
            let request_url = {
                let pos = request_url.find('?').unwrap_or(request_url.len());
                &request_url[..pos]
            };

            (HOOK_REGISTRY.lock().unwrap().action_add_route).sort_by(|a, b| (a.priority()).cmp((&b.priority())));

            for hook in HOOK_REGISTRY.lock().unwrap().action_add_route.iter_mut() {
                println!("{:?}", hook.priority());
                hook.action_add_route();
            }

            for route in ROUTE_REGISTRY.lock().unwrap().routes.iter() {
                if result.is_none() {
                    // This checking of the path is terrible, limited, and hacky
                    if request.method() == route.method {
                        let (first, _) = (route.path).split_at((route.path).len()-1);
                        if (request_url == route.path) || (route.path.ends_with("*") && request_url.starts_with(first)) { // &"Können"[..6]; str.len()
                            result = Some((route.func)(request));
                        }
                    }
                }
            }

            result.unwrap_or_else(|| Response::text("Default!"))
        });
        
    }
}