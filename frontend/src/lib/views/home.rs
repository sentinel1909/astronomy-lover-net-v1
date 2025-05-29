// content.rs

// dependencies
use crate::{ApiResponse, domain::State};
use gloo_net::http::Request;
use yew::{Html, classes, function_component, html, use_effect_with};
use yewdux::prelude::*;

// function to fetch data from the NASA Astronomy Photo of the Day API
async fn get_api_response(api_url: &str) -> ApiResponse {
    Request::get(api_url)
        .send()
        .await
        .expect("Unable to fetch data from NASA API")
        .json()
        .await
        .expect("Data received from API is not valid.")
}

// the content component, renders select data returned from the NASA APOD API
#[function_component(Home)]
pub fn home() -> Html {
    let (state, dispatch) = use_store::<State>();
    {
        use_effect_with((), move |_| {
            wasm_bindgen_futures::spawn_local(async move {
                let api_response = get_api_response("/fetch").await;
                dispatch.reduce(|_| {
                    {
                        State {
                            fetched_data: api_response,
                        }
                    }
                    .into()
                });
            });
            || ()
        });
    }

    html! {
        <main>
            <section>
                <h3>{ "Date: " } {&state.fetched_data.content.date}</h3>
                <h3>{ "Title: " } {&state.fetched_data.content.title}</h3>
                <h3>{ "Explanation: " } </h3>
                <p> {&state.fetched_data.content.explanation} </p>
                <br />
                if state.fetched_data.content.media_type == "image"  {
                    if let Some(hdurl) = state.fetched_data.content.hdurl.clone() {
                        <h3>{ "Image: " }</h3>
                        <img src={hdurl} class={classes!("img-fluid")} alt={"NASA Astronomy Photo of the Day "} />
                    } else {
                        <h3>{ "Image: " }</h3>
                        <img src={state.fetched_data.content.url.clone()} class={classes!("img-fluid")} alt={"NASA Astronomy Photo of the Day "} />
                    }
                } else {
                    <h3>{ "Video: "}</h3>
                    <iframe width="960" height="540" src={state.fetched_data.content.url.clone()}></iframe>
                }
                if let Some(copyright) = &state.fetched_data.content.copyright {
                    <h3>{ "Image by: "} {&copyright}</h3>
                } else {
                    <p>{ "Today's image or video has no attributed copyright data. Copyright may embedded in a watermark."}</p>
                }
            </section>
        </main>
    }
}
