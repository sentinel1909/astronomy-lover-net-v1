// src/lib/views/home.rs

// dependencies
use domain::NasaData;
use gloo_net::http::Request;
use leptos::prelude::*;
use std::rc::Rc;
use url::Url;

// declare the NASA API key as a constant
const NASA_API_KEY: &str = "lsULnkmChaJlS3fZO85M3cnGA8TFCAm2peEfd9QS";

// function to fetch data from the NASA Astronomy Photo of the Day API
async fn get_api_data(api_url: &str) -> NasaData {
    Request::get(api_url)
        .send()
        .await
        .expect("Unable to fetch data from NASA API")
        .json()
        .await
        .expect("Data received from API is not valid.")
}

// function to build the API url, including the API key
fn build_api_url() -> Url {
    let key = NASA_API_KEY;
    let api_key = ["apod?api_key=", key].concat();
    let api_url = Url::parse("https://api.nasa.gov/planetary/")
        .expect("Failed to parse the url to fetch data from.");
    api_url
        .join(&api_key)
        .expect("Failed to join URL with the API key.")
}

// function which returns the home or index page view
#[component]
pub fn Home() -> impl IntoView {
    let api_url = Rc::new(build_api_url());
    let (nasa_data, set_nasa_data) = signal(NasaData::default());
    let api_url_clone = Rc::clone(&api_url);
    Effect::new(move |_| {
        let api_url_clone = Rc::clone(&api_url_clone);
        wasm_bindgen_futures::spawn_local(async move {
            let fetched_data = get_api_data(api_url_clone.as_ref().as_str()).await;
            set_nasa_data.set(fetched_data);
        });
    });

    // set up a closure which outputs copyright info for the photo or video
    let copyright_info = move || match nasa_data.get().copyright {
        Some(copyright) => copyright,
        None => {
            "No attributed copyright information. There might be a watermark in the image or video."
                .to_string()
        }
    };

    view! {
      <main>
        <section>
          <h3>"Date: " {move || nasa_data.get().date}</h3>
          <h3>"Title: " {move || nasa_data.get().title}</h3>
          <h3>"Explanation: "</h3>
          <p>{move || nasa_data.get().explanation}</p>
          <br />
          <Show when=move || { nasa_data.get().media_type == "image"}
            fallback=move || view! { <h3>"Video: "</h3>
                                     <iframe src={move || nasa_data.get().url} />}
          >
            <h3>"Image: "</h3>
            <img src={move || nasa_data.get().url} />
          </Show>
          <h3>"Copyright: "</h3>
          <p>{copyright_info}</p>

        </section>
      </main>
    }
}
