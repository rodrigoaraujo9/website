use crate::components::page::*;
use dioxus::prelude::*;

mod components;

const MAIN_CSS: &str = include_str!("../assets/styling/main.css");

#[derive(Clone, Routable, Debug, PartialEq)]
enum Route {
    #[route("/")]
    Home {},

    #[route("/about")]
    About {},

    #[route("/projects")]
    Projects {},

    #[route("/music")]
    Music {},

    #[route("/photos")]
    Photos {},

    #[route("/social")]
    Social {},
}

fn main() {
    dioxus::launch(App);
}

#[component]
fn App() -> Element {
    rsx! {
        document::Title { "Rodrigo Araújo" }

        document::Link {
            rel: "icon",
            href: "data:image/svg+xml,<svg xmlns=%22http://www.w3.org/2000/svg%22 viewBox=%220 0 100 100%22><text y=%22.9em%22 font-size=%2290%22>🧑🏻‍💻</text></svg>"
        }

        document::Style {
            {MAIN_CSS}
        }

        Router::<Route> {}
    }
}
