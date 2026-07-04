use dioxus::prelude::*;

use views::{AppShell, Dashboard, Graph, PrivateLinks, ProjectDetail, Projects, Settings, Spaces};

mod components;
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
const COMPONENTS_CSS: Asset = asset!("/assets/dx-components-theme.css");
const MAIN_CSS: Asset = asset!("/assets/styling/main.css");
const TAILWIND_CSS: Asset = asset!("/assets/tailwind.css");

fn main() {
    dioxus::launch(App);
}

#[component]
fn App() -> Element {
    rsx! {
        document::Meta { name: "viewport", content: "width=device-width, initial-scale=1.0" }
        document::Link { rel: "icon", href: FAVICON }
        document::Link { rel: "stylesheet", href: COMPONENTS_CSS }
        document::Link { rel: "stylesheet", href: MAIN_CSS }
        document::Link { rel: "stylesheet", href: TAILWIND_CSS }

        Router::<Route> {}
    }
}
