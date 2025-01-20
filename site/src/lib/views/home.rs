// src/lib/views/home.rs

// dependencies
use domain::NasaData;
use gloo_net::http::Request;
use leptos::prelude::*;

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

// function which returns the home or index page view
#[component]
pub fn Home() -> impl IntoView {
    let api_url = "http://localhost:8000/api/nasa_cached";
    let (nasa_data, set_nasa_data) = signal(NasaData::default());
    Effect::new(move |_| {
        wasm_bindgen_futures::spawn_local(async move {
            let fetched_data = get_api_data(api_url).await;
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
                                     <iframe src={move || nasa_data.get().url} width={"1024"} height={"768"} />}
          >
            <h3>"Image: "</h3>
            <img src={move || nasa_data.get().url} width={"1024"} height={"768"} />
          </Show>
          <h3>"Copyright: "</h3>
          <p>{copyright_info}</p>

        </section>
      </main>
    }
}
