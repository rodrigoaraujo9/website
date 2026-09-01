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
        Router::<Route> {}
    }
}
