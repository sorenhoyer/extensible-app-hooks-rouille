extern crate mount;
extern crate router;
extern crate iron;

use mount::Mount;
use std::cmp::Ordering;

pub trait Plugin {
    fn name(&self) -> &str;
    fn init(&self);
}

pub trait Hook {
    #[allow(unused_variables)]
    fn action_mount_static_file_path(&self, mount: &mut Mount) {}
    #[allow(unused_variables)]
    fn filter_the_content(&self, content: String) -> String { "".to_string() }
    fn priority(&self) -> i32;
}

impl<'a> PartialOrd<Hook + 'a> for Hook + 'a {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl<'a> PartialEq<Hook + 'a> for Hook + 'a {
    fn eq(&self, other: &Self) -> bool {
        (self.priority()) == (other.priority())
    }
}

impl<'a> Ord for Hook + 'a {
    fn cmp(&self, other: &Self) -> Ordering {
        (self.priority()).cmp(&(other.priority()))
    }
}

impl<'a> Eq for Hook + 'a { }