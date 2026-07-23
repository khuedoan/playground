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
    view::{attributes, component, view},
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
            <body class="min-h-screen bg-foreground/5">
                <header class="bg-[#430098] text-white shadow-sm">
                    <div class="mx-auto flex h-16 max-w-7xl items-center gap-5 px-4 sm:px-6">
                        <a href="#" class="flex shrink-0 items-center gap-2 font-semibold tracking-tight">
                            <span class="grid size-8 place-items-center rounded-lg bg-white/15 text-lg">"O"</span>
                            <span class="hidden sm:inline">"Orbit Cloud"</span>
                        </a>
                        <div class="relative hidden max-w-lg flex-1 md:block">
                            icon(
                                data: iconify_icon!("feather:search"),
                                attrs: attributes! { class="absolute top-1/2 left-3 size-4 -translate-y-1/2 text-white/60" }
                            )
                            <input
                                type="search"
                                aria-label="Search apps and pipelines"
                                placeholder="Search apps, pipelines, and teams"
                                class="h-9 w-full rounded-md border border-white/15 bg-white/10 pr-3 pl-9 text-sm text-white outline-none placeholder:text-white/55 focus:border-white/40 focus:bg-white/15"
                            >
                        </div>
                        <nav class="ml-auto flex items-center gap-1 text-sm">
                            <a href="#" class="hidden rounded-md px-3 py-2 text-white/80 hover:bg-white/10 hover:text-white sm:block">"Docs"</a>
                            <a href="#" class="hidden rounded-md px-3 py-2 text-white/80 hover:bg-white/10 hover:text-white sm:block">"Support"</a>
                            dropdown_menu(
                                dropdown_menu_trigger(
                                    attrs: attributes! {
                                        class="flex size-9 items-center justify-center rounded-full bg-white/15 font-semibold outline-none hover:bg-white/20 focus-visible:ring-2 focus-visible:ring-white"
                                        aria-label="Open account menu"
                                    },
                                    "KS"
                                )
                                dropdown_menu_content(
                                    attrs: attributes! { class="right-0 left-auto w-52 text-foreground" },
                                    dropdown_menu_label("Kara Smith")
                                    dropdown_menu_separator()
                                    dropdown_menu_item("Account settings")
                                    dropdown_menu_item("Team preferences")
                                    dropdown_menu_separator()
                                    dropdown_menu_item("Sign out")
                                )
                            )
                        </nav>
                    </div>
                </header>

                <main>
                    <section class="border-b border-border bg-background">
                        <div class="mx-auto max-w-6xl px-4 pt-7 sm:px-6">
                            <div class="mb-5 flex flex-col justify-between gap-4 sm:flex-row sm:items-center">
                                <div>
                                    <div class="mb-2 flex items-center gap-2 text-xs text-muted-foreground">
                                        <a href="#" class="hover:text-foreground">"Personal apps"</a>
                                        <span>"/"</span>
                                        <span>"Production"</span>
                                    </div>
                                    <div class="flex flex-wrap items-center gap-3">
                                        <span class="grid size-11 place-items-center rounded-xl bg-primary/10 text-primary">
                                            icon(data: iconify_icon!("feather:box"), attrs: attributes! { class="size-5" })
                                        </span>
                                        <div>
                                            <h1 class="text-2xl font-semibold tracking-tight">"acme-storefront"</h1>
                                            <div class="mt-1 flex items-center gap-2 text-sm text-muted-foreground">
                                                <span class="size-2 rounded-full bg-emerald-500"></span>
                                                <span>"Deployed"</span>
                                                <span>"·"</span>
                                                <span>"United States"</span>
                                            </div>
                                        </div>
                                    </div>
                                </div>
                                <div class="flex items-center gap-2">
                                    <a
                                        href="#"
                                        class=(button_variants(ButtonVariant::Outline, ButtonSize::Md))
                                    >
                                        icon(data: iconify_icon!("feather:external-link"), attrs: attributes! { class="size-4" })
                                        "Open app"
                                    </a>
                                    button(
                                        attrs: attributes! { type="button" },
                                        icon(data: iconify_icon!("feather:plus"), attrs: attributes! { class="size-4" })
                                        "Create release"
                                    )
                                </div>
                            </div>
                            <nav class="flex gap-6 overflow-x-auto text-sm" aria-label="Application">
                                <a href="#" class="border-b-2 border-primary px-1 py-3 font-medium text-primary">"Overview"</a>
                                <a href="#" class="border-b-2 border-transparent px-1 py-3 text-muted-foreground hover:text-foreground">"Resources"</a>
                                <a href="#" class="border-b-2 border-transparent px-1 py-3 text-muted-foreground hover:text-foreground">"Deploy"</a>
                                <a href="#" class="border-b-2 border-transparent px-1 py-3 text-muted-foreground hover:text-foreground">"Activity"</a>
                                <a href="#" class="border-b-2 border-transparent px-1 py-3 text-muted-foreground hover:text-foreground">"Access"</a>
                                <a href="#" class="border-b-2 border-transparent px-1 py-3 text-muted-foreground hover:text-foreground">"Settings"</a>
                            </nav>
                        </div>
                    </section>

                    <div class="mx-auto grid max-w-6xl gap-6 px-4 py-7 sm:px-6 lg:grid-cols-[minmax(0,1fr)_19rem]">
                        <div class="space-y-6">
                            card(
                                card_header(
                                    attrs: attributes! { class="flex-row items-start justify-between gap-4" },
                                    <div>
                                        card_title("Latest release")
                                        card_description("Production is healthy and serving traffic.")
                                    </div>
                                    badge(variant: BadgeVariant::Secondary, "v184")
                                )
                                card_content(
                                    <div class="rounded-lg border border-border bg-foreground/[0.025] p-4">
                                        <div class="flex gap-3">
                                            <span class="grid size-8 shrink-0 place-items-center rounded-full bg-emerald-100 text-emerald-700">
                                                icon(data: iconify_icon!("feather:check"), attrs: attributes! { class="size-4" })
                                            </span>
                                            <div class="min-w-0 flex-1">
                                                <div class="flex flex-wrap items-center justify-between gap-2">
                                                    <p class="font-medium">"Deploy 8f72c1a succeeded"</p>
                                                    <span class="text-xs text-muted-foreground">"12 minutes ago"</span>
                                                </div>
                                                <p class="mt-1 text-sm text-muted-foreground">"Fix cart totals on discounted orders"</p>
                                                <div class="mt-3 flex flex-wrap items-center gap-3 text-xs text-muted-foreground">
                                                    <span class="inline-flex items-center gap-1.5">
                                                        icon(data: iconify_icon!("feather:git-branch"), attrs: attributes! { class="size-3.5" })
                                                        "main"
                                                    </span>
                                                    <span>"Deployed by Kara Smith"</span>
                                                </div>
                                            </div>
                                        </div>
                                    </div>
                                    <a href="#" class="mt-4 inline-flex items-center gap-1 text-sm font-medium text-primary hover:underline">
                                        "View release history"
                                        icon(data: iconify_icon!("feather:arrow-right"), attrs: attributes! { class="size-3.5" })
                                    </a>
                                )
                            )

                            card(
                                card_header(
                                    attrs: attributes! { class="flex-row items-center justify-between gap-4" },
                                    <div>
                                        card_title("Resources")
                                        card_description("Services currently running for this app.")
                                    </div>
                                    <a href="#" class="text-sm font-medium text-primary hover:underline">"Manage"</a>
                                )
                                card_content(
                                    <div class="divide-y divide-border rounded-lg border border-border">
                                        resource_row(
                                            name: "web",
                                            detail: "Standard-1X · 2 dynos",
                                            usage: "62% utilization"
                                        )
                                        resource_row(
                                            name: "worker",
                                            detail: "Standard-1X · 1 dyno",
                                            usage: "28% utilization"
                                        )
                                        resource_row(
                                            name: "scheduler",
                                            detail: "Eco · 1 dyno",
                                            usage: "Runs hourly"
                                        )
                                    </div>
                                )
                            )

                            card(
                                card_header(
                                    card_title("Recent activity")
                                    card_description("Changes made to this application.")
                                )
                                card_content(
                                    <ol class="space-y-5">
                                        activity_item(
                                            title: "Release v184 deployed",
                                            detail: "Kara Smith deployed 8f72c1a from main",
                                            time: "12m"
                                        )
                                        activity_item(
                                            title: "Config vars updated",
                                            detail: "Jon Bell changed STRIPE_API_VERSION",
                                            time: "2h"
                                        )
                                        activity_item(
                                            title: "Worker scaled to 1 dyno",
                                            detail: "Autoscaling policy adjusted resource count",
                                            time: "Yesterday"
                                        )
                                    </ol>
                                )
                            )
                        </div>

                        <aside class="space-y-6">
                            card(
                                attrs: attributes! { class="gap-4" },
                                card_header(
                                    card_title("App health")
                                    card_description("Last 24 hours")
                                )
                                card_content(
                                    <div class="space-y-5">
                                        metric(label: "Uptime", value: "99.99%", amount: 99.99)
                                        metric(label: "Error budget", value: "86%", amount: 86.0)
                                        <div class="grid grid-cols-2 gap-3 border-t border-border pt-4">
                                            <div>
                                                <p class="text-xs text-muted-foreground">"Requests"</p>
                                                <p class="mt-1 text-lg font-semibold">"1.8M"</p>
                                            </div>
                                            <div>
                                                <p class="text-xs text-muted-foreground">"p95 latency"</p>
                                                <p class="mt-1 text-lg font-semibold">"142ms"</p>
                                            </div>
                                        </div>
                                    </div>
                                )
                            )

                            card(
                                attrs: attributes! { class="gap-4" },
                                card_header(
                                    card_title("App information")
                                )
                                card_content(
                                    <dl class="space-y-4 text-sm">
                                        info_row(label: "Stack", value: "orbit-24")
                                        info_row(label: "Region", value: "us-east")
                                        info_row(label: "GitHub", value: "acme/storefront")
                                        info_row(label: "Framework", value: "Rust")
                                    </dl>
                                )
                            )

                            card(
                                attrs: attributes! { class="gap-4" },
                                card_header(
                                    card_title("Monthly estimate")
                                    card_description("Current resource usage")
                                )
                                card_content(
                                    <div class="flex items-end justify-between">
                                        <p class="text-2xl font-semibold">"$43.00"</p>
                                        badge(variant: BadgeVariant::Outline, "On track")
                                    </div>
                                    <a href="#" class="mt-4 inline-flex items-center gap-1 text-sm font-medium text-primary hover:underline">
                                        "View billing"
                                        icon(data: iconify_icon!("feather:arrow-right"), attrs: attributes! { class="size-3.5" })
                                    </a>
                                )
                            )
                        </aside>
                    </div>
                </main>
            </body>
        </html>
    }
}

#[component]
async fn resource_row(name: &str, detail: &str, usage: &str) -> Result {
    view! {
        <div class="flex items-center gap-3 p-4">
            <span class="relative flex size-9 shrink-0 items-center justify-center rounded-lg bg-primary/10 text-primary">
                icon(data: iconify_icon!("feather:server"), attrs: attributes! { class="size-4" })
                <span class="absolute -right-0.5 -bottom-0.5 size-2.5 rounded-full border-2 border-background bg-emerald-500"></span>
            </span>
            <div class="min-w-0 flex-1">
                <p class="font-mono text-sm font-semibold">(name)</p>
                <p class="truncate text-xs text-muted-foreground">(detail)</p>
            </div>
            <span class="hidden text-xs text-muted-foreground sm:block">(usage)</span>
            button(
                variant: ButtonVariant::Ghost,
                size: ButtonSize::Icon,
                attrs: attributes! { type="button" aria-label="Resource actions" },
                icon(data: iconify_icon!("feather:more-horizontal"), attrs: attributes! { class="size-4" })
            )
        </div>
    }
}

#[component]
async fn activity_item(title: &str, detail: &str, time: &str) -> Result {
    view! {
        <li class="relative flex gap-3 pl-1">
            <span class="mt-1 grid size-7 shrink-0 place-items-center rounded-full border border-border bg-background text-muted-foreground">
                icon(data: iconify_icon!("feather:activity"), attrs: attributes! { class="size-3.5" })
            </span>
            <div class="min-w-0 flex-1">
                <div class="flex items-start justify-between gap-3">
                    <p class="text-sm font-medium">(title)</p>
                    <time class="shrink-0 text-xs text-muted-foreground">(time)</time>
                </div>
                <p class="mt-0.5 text-xs text-muted-foreground">(detail)</p>
            </div>
        </li>
    }
}

#[component]
async fn metric(label: &str, value: &str, amount: f32) -> Result {
    view! {
        <div>
            <div class="mb-2 flex items-center justify-between text-sm">
                <span class="text-muted-foreground">(label)</span>
                <span class="font-medium">(value)</span>
            </div>
            progress(value: amount, attrs: attributes! { aria-label=(label) })
        </div>
    }
}

#[component]
async fn info_row(label: &str, value: &str) -> Result {
    view! {
        <div class="flex items-start justify-between gap-4">
            <dt class="text-muted-foreground">(label)</dt>
            <dd class="text-right font-medium">(value)</dd>
        </div>
    }
}
