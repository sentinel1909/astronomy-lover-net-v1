// src/components/header.rs

// dependencies
use yew::{Html, function_component, html};

// the site header component
#[function_component(Header)]
pub fn header() -> Html {
    html! {
        <header>
            <h1 class="text-2xl"> { "NASA Imagery Viewer" }</h1>
            <h2 class="text-xl"> { "...a new photo or video every day"}</h2>
        </header>
    }
}
