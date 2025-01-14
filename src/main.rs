use dioxus::prelude::*;

const FAVICON: Asset = asset!("/assets/favicon.ico");
const MAIN_CSS: Asset = asset!("/assets/main.css");
const HEADER_SVG: Asset = asset!("/assets/header.svg");
const TAILWIND_CSS: Asset = asset!("/assets/tailwind.css");
const PRELINE_UI_SCRIPT: Asset = asset!("./node_modules/preline/dist/preline.js");

mod components;
mod route;
mod pages;

fn main() {
    launch(App);
}

#[component]
fn App() -> Element {
    rsx! {
        document::Stylesheet { href: MAIN_CSS }
        // div { id: "title",
        //     h1 { "HotDog! 🌭" }
        // }
        // div { id: "dogview",
        //     img { src: "https://images.dog.ceo/breeds/pitbull/dog-3981540_1280.jpg" }
        // }
        // div { id: "buttons",
        //     button { id: "skip", "skip" }
        //     button { id: "save", "save!" }
        // }
        document::Script { src: PRELINE_UI_SCRIPT}
    }
}
