// src/bin/main.rs

// dependencies
use astronomy_lover_net_v1_lib::app::App;
use leptos::prelude::*;

// main function
fn main() {
    console_error_panic_hook::set_once();
    mount_to_body(|| view! { <App /> });
}
