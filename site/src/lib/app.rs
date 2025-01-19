// src/lib/app.rs

// dependencies
use crate::views::Root;
use leptos::prelude::*;
use leptos_router::components::{Route, Router, Routes};
use leptos_router_macro::path;

// function which creates the main application
#[component]
pub fn App() -> impl IntoView {
    view! {
       <div class="container mx-auto">
        <Router>
            <Routes fallback=|| "This page cannot be found">
                <Route path=path!("/") view=Root />
            </Routes>
        </Router>
       </div>
    }
}
