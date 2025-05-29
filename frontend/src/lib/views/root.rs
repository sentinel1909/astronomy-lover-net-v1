// src/lib/views/root.rs

// dependencies
use crate::components::{Footer, Header};
use crate::views::Home;
use yew::function_component;
use yew::{Html, html};

// the root component
#[function_component(Root)]
pub fn root() -> Html {
    html! {
      <div class="container mx-auto px-4 bg-gray-700 text-white">
        <Header />
        <br />
        <Home />
        <Footer />
      </div>
    }
}
