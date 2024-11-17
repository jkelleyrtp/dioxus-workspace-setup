use dioxus::prelude::*;
use ui::CoolComponent;
use ui::ReallyCoolComponent;

fn main() {
    dioxus::launch(app);
}

fn app() -> Element {
    rsx! {
        div { "hmm" }
        CoolComponent {}
        ReallyCoolComponent {}
    }
}
