// footer.rs

// dependencies
use yew::{Html, function_component, html};

// the footer component
#[function_component(Footer)]
pub fn footer() -> Html {
    html! {
        <footer>
            <section>
                <br />
                <p>{ "\u{00A9} " } { " Jeffery D Mitchell | Site created in WebAssembly with " }<a href="https://yew.rs"> { "Yew" }</a></p>
                <p> {" Site hosting courtesy of: " }<a href="https://shuttle.dev">{ "shuttle.dev" }</a></p>
            </section>
        </footer>
    }
}
