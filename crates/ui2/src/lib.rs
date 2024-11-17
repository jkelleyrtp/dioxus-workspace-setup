use dioxus::prelude::*;

pub fn ReallyCoolComponent() -> Element {
    rsx! {
        div {
            img { src: asset!("/assets/logo2.png") }
        }
    }
}
