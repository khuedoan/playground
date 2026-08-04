use dioxus::prelude::*;

use views::{AppShell, Dashboard, Graph, PrivateLinks, ProjectDetail, Projects, Settings, Spaces};

mod views;

#[derive(Debug, Clone, Routable, PartialEq)]
#[rustfmt::skip]
enum Route {
    #[layout(AppShell)]
        #[route("/")]
        Dashboard {},
        #[route("/graph")]
        Graph {},
        #[route("/projects")]
        Projects {},
        #[route("/projects/:slug")]
        ProjectDetail { slug: String },
        #[route("/private-links")]
        PrivateLinks {},
        #[route("/spaces")]
        Spaces {},
        #[route("/settings")]
        Settings {},
}

const FAVICON: Asset = asset!("/assets/favicon.ico");
const MAIN_CSS: Asset = asset!("/assets/main.css");

fn main() {
    dioxus::launch(App);
}

#[component]
fn App() -> Element {
    rsx! {
        document::Meta { name: "viewport", content: "width=device-width, initial-scale=1.0" }
        document::Link { rel: "icon", href: FAVICON }
        document::Stylesheet { href: MAIN_CSS }

        Router::<Route> {}
    }
}
