use dioxus::prelude::*;

use crate::backend::list_dogs;

#[component]
pub fn Favorites() -> Element {
    let favorites = use_server_future(list_dogs)?;

    rsx! {
        div { id: "favorites",
            div { id: "favorites-container",
                for (id , url) in favorites().unwrap().unwrap() {
                    div { key: "{id}", class: "favorite-dog",
                        img { src: "{url}" }
                    }
                }
            }
        }
    }
}
