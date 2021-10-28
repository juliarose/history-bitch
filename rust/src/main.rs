
use sycamore::prelude::*;
use history_bitch::app::App;

fn main() {
    console_error_panic_hook::set_once();
    console_log::init_with_level(log::Level::Debug).unwrap();
    
    web_sys::console::time_with_label("render");
    sycamore::render(|| template! { 
        App()
    });
    web_sys::console::time_end_with_label("render");
}