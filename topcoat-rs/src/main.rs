use topcoat::{
    Result,
    asset::{AssetBundle, RouterBuilderAssetExt},
    router::{Router, RouterBuilderDiscoverExt, page},
    tailwind,
    view::{component, view},
};

#[tokio::main]
async fn main() {
    topcoat::start(
        Router::builder()
            .discover()
            .assets(AssetBundle::load().unwrap())
            .build(),
    )
    .await
    .unwrap();
}

#[page("/")]
async fn home() -> Result {
    view! {
        <!DOCTYPE html>
        <html>
            <head>
                <title>"Hello world"</title>
                <link rel="stylesheet" href=(tailwind::stylesheet!())>
                topcoat::dev::script()
            </head>
            <body>
                hello(name: "World")
            </body>
        </html>
    }
}

#[component]
async fn hello(name: &str) -> Result {
    view! {
        <h1>"Hello, " (name) "!"</h1>
    }
}
