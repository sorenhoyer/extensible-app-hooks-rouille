use std::sync::Mutex;
use app_trait::Hook;

lazy_static! {
    pub static ref HOOK_REGISTRY: Mutex<HookRegistry> = Mutex::new(HookRegistry::new());
}

pub struct HookRegistry {
    //pub action_mount_static_file_path: Vec<Box<Hook + Send>>,
    pub action_add_route: Vec<Box<Hook + Send>>,
    pub filter_the_content: Vec<Box<Hook + Send>>,
}

impl HookRegistry {
    pub fn new() -> HookRegistry {
        HookRegistry {
            //action_mount_static_file_path: Vec::new(),
            action_add_route: Vec::new(),
            filter_the_content: Vec::new(),
        }
    }

}