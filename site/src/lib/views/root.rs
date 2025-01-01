// src/lib/views/root.rs

// dependencies
use crate::components::{Footer, Header};
use crate::views::home::Home;
use leptos::{component, view, IntoView};

// function which assemblies and returns the root component
#[component]
pub fn Root() -> impl IntoView {
    view! {
      <>
        <Header />
        <Home />
        <Footer />
      </>
    }
}
