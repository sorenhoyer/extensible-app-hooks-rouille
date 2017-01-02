extern crate app_registry;
extern crate app_trait;

extern crate iron;
extern crate mount;
extern crate router;
extern crate handlebars_iron as hbs;
extern crate serde_json;

use std::collections::BTreeMap;

use app_registry::plugin_registry::PLUGIN_REGISTRY;
use app_registry::hook_registry::HOOK_REGISTRY;
use app_trait::{Plugin};

use iron::prelude::*;
use iron::status;
use mount::Mount;
use router::Router;
use hbs::{Template, HandlebarsEngine, DirectorySource};
use serde_json::value::{self, Value};

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

        let mut router = Router::new();

        router.get("/*", render_template, "wildcard");
        router.get("/", render_template, "root");

        let mut main = Chain::new(router);

        let mut hbse = HandlebarsEngine::new();
        hbse.add(Box::new(DirectorySource::new("../app/src/templates/", ".hbs")));
        
        // load templates from all registered sources
        if let Err(r) = hbse.reload() {
            panic!("{}", r);
        }

        main.link_after(hbse);

        let mut mount = Mount::new();
        mount.mount("/", main);

        (HOOK_REGISTRY.lock().unwrap().action_mount_static_file_path).sort_by(|a, b| (a.priority()).cmp((&b.priority())));

        for hook in HOOK_REGISTRY.lock().unwrap().action_mount_static_file_path.iter_mut() {
            println!("{:?}", hook.priority());
            hook.action_mount_static_file_path(&mut mount);
        }

        println!("{}", "Starting server...");
        Iron::new(mount).http("localhost:3000").unwrap();
    }
}

fn render_template(_: &mut Request) -> IronResult<Response> {
    println!("{}", "front facing site..");
    let mut resp = Response::new();

    let mut data: BTreeMap<String, Value> = BTreeMap::new();
    let mut content: String = "Hello world, this is content.".to_string();

    for hook in HOOK_REGISTRY.lock().unwrap().filter_the_content.iter_mut() {
        content = hook.filter_the_content(content);
    }
    
    data.insert("content".to_string(), value::to_value(&content));

    resp.set_mut(Template::new("index-front", data)).set_mut(status::Ok);
    println!("{:?}", resp);
    Ok(resp)
}