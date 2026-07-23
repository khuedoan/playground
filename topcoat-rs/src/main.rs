mod components;

use components::{
    badge::{BadgeVariant, badge},
    button::{ButtonSize, ButtonVariant, button, button_variants},
    card::{card, card_content, card_description, card_header, card_title},
    dropdown_menu::{
        dropdown_menu, dropdown_menu_content, dropdown_menu_item, dropdown_menu_label,
        dropdown_menu_separator, dropdown_menu_trigger,
    },
    progress::progress,
};
use topcoat::{
    Result,
    asset::{AssetBundle, RouterBuilderAssetExt},
    icon::{icon, iconify::iconify_icon},
    router::{Router, RouterBuilderDiscoverExt, page},
    tailwind,
    view::{attributes, view},
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
        <html lang="en">
            <head>
                <meta charset="utf-8">
                <meta name="viewport" content="width=device-width, initial-scale=1">
                <title>"Orbit Cloud | acme-storefront"</title>
                <link rel="stylesheet" href=(tailwind::stylesheet!())>
                topcoat::dev::script()
            </head>
            <body>
                <header class="border-b border-border">
                    <nav class="mx-auto flex h-16 max-w-6xl items-center px-4 sm:px-6" aria-label="Global">
                        <a href="#" class="flex items-center gap-2 font-semibold">
                            icon(data: iconify_icon!("feather:hexagon"), attrs: attributes! { class="size-6" })
                            "Orbit Cloud"
                        </a>
                        <div class="ml-auto flex items-center gap-2">
                            <a href="#" class=(button_variants(ButtonVariant::Ghost, ButtonSize::Sm))>
                                "Documentation"
                            </a>
                            dropdown_menu(
                                dropdown_menu_trigger(
                                    attrs: attributes! {
                                        class=(button_variants(ButtonVariant::Outline, ButtonSize::Icon))
                                        aria-label="Open account menu"
                                    },
                                    icon(data: iconify_icon!("feather:user"), attrs: attributes! { class="size-4" })
                                )
                                dropdown_menu_content(
                                    attrs: attributes! { class="right-0 left-auto" },
                                    dropdown_menu_label("Kara Smith")
                                    dropdown_menu_separator()
                                    dropdown_menu_item("Account settings")
                                    dropdown_menu_item("Sign out")
                                )
                            )
                        </div>
                    </nav>
                </header>

                <main class="mx-auto max-w-6xl px-4 py-8 sm:px-6">
                    <header class="mb-6 flex flex-wrap items-center justify-between gap-4">
                        <div>
                            <p class="text-sm text-muted-foreground">"Personal apps"</p>
                            <h1 class="flex items-center gap-2 text-2xl font-semibold">
                                icon(data: iconify_icon!("feather:box"), attrs: attributes! { class="size-6" })
                                "acme-storefront"
                                badge(variant: BadgeVariant::Secondary, "Running")
                            </h1>
                        </div>
                        <div class="flex gap-2">
                            <a href="#" class=(button_variants(ButtonVariant::Outline, ButtonSize::Md))>
                                icon(data: iconify_icon!("feather:external-link"), attrs: attributes! { class="size-4" })
                                "Open app"
                            </a>
                            button(
                                icon(data: iconify_icon!("feather:upload-cloud"), attrs: attributes! { class="size-4" })
                                "Deploy"
                            )
                        </div>
                    </header>

                    <nav class="mb-6 flex gap-2 overflow-x-auto" aria-label="Application">
                        <a href="#" class=(button_variants(ButtonVariant::Secondary, ButtonSize::Sm))>"Overview"</a>
                        <a href="#" class=(button_variants(ButtonVariant::Ghost, ButtonSize::Sm))>"Resources"</a>
                        <a href="#" class=(button_variants(ButtonVariant::Ghost, ButtonSize::Sm))>"Deployments"</a>
                        <a href="#" class=(button_variants(ButtonVariant::Ghost, ButtonSize::Sm))>"Settings"</a>
                    </nav>

                    <div class="grid gap-6 lg:grid-cols-[minmax(0,1fr)_19rem]">
                        <section class="space-y-6" aria-label="Application overview">
                            card(
                                card_header(
                                    attrs: attributes! { class="flex-row justify-between" },
                                    <div>
                                        card_title("Latest deployment")
                                        card_description("Production is healthy and serving traffic.")
                                    </div>
                                    badge(variant: BadgeVariant::Outline, "v184")
                                )
                                card_content(
                                    <article>
                                        <header class="flex flex-wrap items-center justify-between gap-2">
                                            badge(
                                                variant: BadgeVariant::Secondary,
                                                icon(data: iconify_icon!("feather:check"), attrs: attributes! { class="size-3" })
                                                "Succeeded"
                                            )
                                            <time datetime="2026-07-23T22:48:00Z" class="text-xs text-muted-foreground">"12 minutes ago"</time>
                                        </header>
                                        <h4 class="mt-3 font-medium">"Fix cart totals on discounted orders"</h4>
                                        <p class="mt-1 flex items-center gap-1 text-sm text-muted-foreground">
                                            icon(data: iconify_icon!("feather:git-branch"), attrs: attributes! { class="size-4" })
                                            "main · 8f72c1a · Kara Smith"
                                        </p>
                                    </article>
                                )
                            )

                            card(
                                card_header(
                                    attrs: attributes! { class="flex-row items-center justify-between" },
                                    <div>
                                        card_title("Resources")
                                        card_description("Services running for this application.")
                                    </div>
                                    button(variant: ButtonVariant::Outline, size: ButtonSize::Sm, "Manage")
                                )
                                card_content(
                                    <table class="w-full text-sm">
                                        <thead class="text-left text-muted-foreground">
                                            <tr>
                                                <th scope="col" class="pb-2 font-medium">"Process"</th>
                                                <th scope="col" class="pb-2 font-medium">"Plan"</th>
                                                <th scope="col" class="pb-2 text-right font-medium">"Status"</th>
                                            </tr>
                                        </thead>
                                        <tbody class="divide-y divide-border">
                                            <tr>
                                                <th scope="row" class="py-3 text-left font-mono font-medium">
                                                    icon(data: iconify_icon!("feather:server"), attrs: attributes! { class="mr-2 inline size-4" })
                                                    "web"
                                                </th>
                                                <td class="py-3 text-muted-foreground">"Standard · 2"</td>
                                                <td class="py-3 text-right">badge(variant: BadgeVariant::Secondary, "Healthy")</td>
                                            </tr>
                                            <tr>
                                                <th scope="row" class="py-3 text-left font-mono font-medium">
                                                    icon(data: iconify_icon!("feather:server"), attrs: attributes! { class="mr-2 inline size-4" })
                                                    "worker"
                                                </th>
                                                <td class="py-3 text-muted-foreground">"Standard · 1"</td>
                                                <td class="py-3 text-right">badge(variant: BadgeVariant::Secondary, "Healthy")</td>
                                            </tr>
                                            <tr>
                                                <th scope="row" class="py-3 text-left font-mono font-medium">
                                                    icon(data: iconify_icon!("feather:database"), attrs: attributes! { class="mr-2 inline size-4" })
                                                    "postgres"
                                                </th>
                                                <td class="py-3 text-muted-foreground">"Essential"</td>
                                                <td class="py-3 text-right">badge(variant: BadgeVariant::Outline, "Attached")</td>
                                            </tr>
                                        </tbody>
                                    </table>
                                )
                            )
                        </section>

                        <aside class="space-y-6" aria-label="Application details">
                            card(
                                card_header(
                                    card_title("App health")
                                    card_description("Last 24 hours")
                                )
                                card_content(
                                    <dl class="space-y-5">
                                        <div>
                                            <dt class="flex justify-between text-sm">
                                                <span class="text-muted-foreground">"Uptime"</span>
                                                <span>"99.99%"</span>
                                            </dt>
                                            <dd class="mt-2">progress(value: 99.99, attrs: attributes! { aria-label="Uptime" })</dd>
                                        </div>
                                        <div>
                                            <dt class="flex justify-between text-sm">
                                                <span class="text-muted-foreground">"Error budget"</span>
                                                <span>"86%"</span>
                                            </dt>
                                            <dd class="mt-2">progress(value: 86.0, attrs: attributes! { aria-label="Error budget" })</dd>
                                        </div>
                                    </dl>
                                )
                            )

                            card(
                                card_header(card_title("App information"))
                                card_content(
                                    <dl class="space-y-4 text-sm">
                                        <div class="flex justify-between gap-4">
                                            <dt class="text-muted-foreground">"Stack"</dt>
                                            <dd>"orbit-24"</dd>
                                        </div>
                                        <div class="flex justify-between gap-4">
                                            <dt class="text-muted-foreground">"Region"</dt>
                                            <dd>"us-east"</dd>
                                        </div>
                                        <div class="flex justify-between gap-4">
                                            <dt class="text-muted-foreground">"Repository"</dt>
                                            <dd>"acme/storefront"</dd>
                                        </div>
                                    </dl>
                                )
                            )
                        </aside>
                    </div>
                </main>
            </body>
        </html>
    }
}
