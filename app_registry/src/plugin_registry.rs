use std::sync::Mutex;
use app_trait::Plugin;

lazy_static! {
    pub static ref PLUGIN_REGISTRY: Mutex<PluginRegistry> = Mutex::new(PluginRegistry::new());
}

pub struct PluginRegistry {
    pub plugins: Vec<Box<Plugin + Send>>,
}

impl PluginRegistry {
    pub fn new() -> PluginRegistry {
        PluginRegistry {
            // ...
            plugins: Vec::new(),
        }
    }

    pub fn add_plugin<P: Plugin + 'static + Send>(&mut self, plugin: P) {
        self.plugins.push(Box::new(plugin));
    }
}