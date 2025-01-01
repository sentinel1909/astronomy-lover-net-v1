// src/lib/app.rs

// dependencies
use crate::views::Root;
use leptos::prelude::*;

// function which creates the main application
#[component]
pub fn App() -> impl IntoView {
    view! {
      <Root />
    }
}
