use dioxus::prelude::*;
use serde::Deserialize;

mod backend;
mod components;

use crate::backend::*;
use crate::components::*;

static CSS: Asset = asset!("/assets/main.css");

#[derive(Routable, Clone, PartialEq)]
enum Route {
    #[layout(NavBar)]
    #[route("/")]
    DogView,

    #[route("/favorites")]
    Favorites,
}



#[derive(Deserialize)]
struct DogApi {
    message: String,
}

fn main() {
    dioxus::launch(App);
}

#[component]
fn App() -> Element {
    rsx! {
        document::Stylesheet { href: CSS }

        Router::<Route> {}
    }
}

#[component]
fn DogView() -> Element {
    let mut img_src = use_resource(|| async move {
        reqwest::get("https://dog.ceo/api/breeds/image/random")
            .await
            .unwrap()
            .json::<DogApi>()
            .await
            .unwrap()
            .message
    });

    rsx! {
        div { id: "dogview",
            img { src: img_src.cloned().unwrap_or_default() }
        }

        div { id: "buttons",
            button { onclick: move |_| img_src.restart(), id: "skip", "skip" }
            button {
                id: "save",
                onclick: move |_| {
                    let current = img_src.cloned().unwrap_or_default();
                    img_src.restart();
                    spawn(async move {
                        match save_dog(current).await {
                            Ok(_) => {}
                            Err(_e) => {}
                        }
                    });
                },
                "save!"
            }
        }
    }
}

// #[post("/api/save_dog")]
// async fn save_dog(image: String) -> Result<(), ServerFnError> {
//     use std::io::Write;

//     let mut file = std::fs::OpenOptions::new()
//         .write(true)
//         .append(true)
//         .create(true)
//         .open("dogs.txt")
//         .map_err(|e| ServerFnError::new(format!("无法打开文件: {}", e)))?;

//     file.write_fmt(format_args!("{image}\n"))
//         .map_err(|e| ServerFnError::new(format!("写入文件失败: {}", e)))?;

//     Ok(())
// }
