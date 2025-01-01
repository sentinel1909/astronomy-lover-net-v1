// src/components/footer.rs

// dependencies
use leptos::prelude::*;

// function which creates the main application
#[component]
pub fn Footer() -> impl IntoView {
    view! {
      <footer>
        <section>"\u{00A9} Jeffery D Mitchell | All Rights Reserved"</section>
      </footer>
    }
}
