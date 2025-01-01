// src/components/header.rs

// dependencies
use leptos::prelude::*;

// function which creates the main application
#[component]
pub fn Header() -> impl IntoView {
    view! {
      <header>
        <h1>"Astronomy Lover (NASA Astronomy Image of the Day)"</h1>
      </header>
    }
}
