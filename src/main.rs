use crate::components::page::*;
use dioxus::prelude::*;

mod components;

const _: Asset = asset!(
    "/assets/styling/main.css",
    AssetOptions::css()
        .with_static_head(true)
        .with_preload(true)
);

#[used]
static SPACE_MONO_REGULAR: Asset = asset!(
    "/assets/fonts/SpaceMono-Regular.woff2",
    AssetOptions::builder().with_hash_suffix(false)
);

#[used]
static SPACE_MONO_BOLD: Asset = asset!(
    "/assets/fonts/SpaceMono-Bold.woff2",
    AssetOptions::builder().with_hash_suffix(false)
);

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
        document::Link {
            rel: "icon",
            href: "data:image/svg+xml,<svg xmlns=%22http://www.w3.org/2000/svg%22 viewBox=%220 0 100 100%22><text y=%22.9em%22 font-size=%2290%22>🧑🏻‍💻</text></svg>"
        }

        Router::<Route> {}
    }
}
