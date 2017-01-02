extern crate app;
extern crate my_test_plugin;

use app::App;
use my_test_plugin::MyTestPlugin;

fn main() {
    let my_test_plugin = MyTestPlugin{};
    App::add_plugin(my_test_plugin);

    let app = App::new();
    app.start();
}