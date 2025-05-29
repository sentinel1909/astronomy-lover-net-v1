// src/bin/main.rs

// dependencies
use astronomy_lover_net_lib::app::App;

// main function to render the site
fn main() {
    wasm_logger::init(wasm_logger::Config::new(log::Level::Trace));
    console_error_panic_hook::set_once();
    yew::Renderer::<App>::new().render();
}
