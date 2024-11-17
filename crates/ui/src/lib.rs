use dioxus::prelude::*;

pub fn CoolComponent() -> Element {
    rsx! {
        div {
            img { src: asset!("/assets/logo.png") }
        }
    }
}

pub use ui2::ReallyCoolComponent;
