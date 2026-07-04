use crate::components::badge::{Badge, BadgeVariant};
use crate::components::button::{Button, ButtonVariant};
use crate::components::card::{Card, CardAction, CardContent, CardHeader, CardTitle};
use crate::views::common::{PageTitle, StatusBadge, UsageMeter};
use dioxus::prelude::*;
use dioxus_icons::lucide::{Cloud, Network, ServerCog, Shield};

#[derive(Clone, Copy)]
struct Space {
    name: &'static str,
    kind: &'static str,
    region: &'static str,
    owner: &'static str,
    environments: &'static str,
    utilization: f64,
    status: &'static str,
}

const SPACES: [Space; 4] = [
    Space {
        name: "hosted-us-east",
        kind: "Shared",
        region: "us-east",
        owner: "Netamos hosted",
        environments: "12 envs",
        utilization: 63.0,
        status: "Available",
    },
    Space {
        name: "acme-pci-prod",
        kind: "Dedicated",
        region: "us-east",
        owner: "Acme Retail",
        environments: "4 envs",
        utilization: 72.0,
        status: "Active",
    },
    Space {
        name: "acme-eu-core",
        kind: "Dedicated",
        region: "eu-west",
        owner: "Acme Retail",
        environments: "3 envs",
        utilization: 41.0,
        status: "Syncing",
    },
    Space {
        name: "acme-dev-sandbox",
        kind: "Dedicated",
        region: "us-west",
        owner: "Acme Retail",
        environments: "0 envs",
        utilization: 8.0,
        status: "Pending",
    },
];

#[component]
pub fn Spaces() -> Element {
    rsx! {
        div { class: "page-header-with-action",
            PageTitle {
                title: "Spaces",
                subtitle: "Compute data planes available to this tenant."
            }
            div { class: "header-actions",
                Button { variant: ButtonVariant::Outline,
                    Shield { size: 16 }
                    "Access policy"
                }
            }
        }

        section { class: "billing-grid",
            Card { class: "plan-card",
                CardHeader {
                    CardTitle { "Tenant access" }
                    CardAction {
                        Badge { variant: BadgeVariant::Primary, "1:N" }
                    }
                }
                CardContent {
                    strong { "Acme Retail" }
                    p { "This tenant can deploy environments into 3 active spaces." }
                    Button { variant: ButtonVariant::Secondary, "Grant space access" }
                }
            }
            Card {
                CardHeader { CardTitle { "Communication rule" } }
                CardContent { class: "forecast-card",
                    Network { size: 28 }
                    strong { "Same space only" }
                    span { "Components in different spaces cannot talk directly." }
                }
            }
        }

        section { class: "usage-grid",
            SpaceSummaryCard {
                title: "Hosted default",
                amount: "hosted-us-east",
                icon: "cloud",
                value: 63.0,
                detail: "Shared PaaS data plane"
            }
            SpaceSummaryCard {
                title: "Enterprise owned",
                amount: "2 active",
                icon: "shield",
                value: 56.0,
                detail: "Tenant-controlled spaces"
            }
            SpaceSummaryCard {
                title: "Pending access",
                amount: "1 space",
                icon: "server",
                value: 8.0,
                detail: "Sandbox awaiting approval"
            }
        }

        Card {
            CardHeader {
                CardTitle { "Space inventory" }
            }
            CardContent {
                div { class: "data-table space-table",
                    div { class: "table-header",
                        span { "Space" }
                        span { "Kind" }
                        span { "Region" }
                        span { "Owner" }
                        span { "Environments" }
                        span { "Utilization" }
                        span { "Status" }
                    }
                    for space in SPACES {
                        div { class: "table-row",
                            div { class: "table-primary",
                                strong { "{space.name}" }
                                span { "Compute data plane" }
                            }
                            StatusBadge { status: space.kind.to_string() }
                            span { "{space.region}" }
                            span { "{space.owner}" }
                            span { "{space.environments}" }
                            UsageMeter { label: "Load", value: space.utilization, detail: "Current" }
                            StatusBadge { status: space.status.to_string() }
                        }
                    }
                }
            }
        }
    }
}

#[component]
fn SpaceSummaryCard(title: String, amount: String, icon: String, value: f64, detail: String) -> Element {
    rsx! {
        Card { class: "usage-cost-card",
            CardHeader {
                div { class: "metric-icon",
                    if icon == "cloud" {
                        Cloud { size: 18 }
                    } else if icon == "shield" {
                        Shield { size: 18 }
                    } else {
                        ServerCog { size: 18 }
                    }
                }
                CardTitle { "{title}" }
            }
            CardContent {
                strong { "{amount}" }
                UsageMeter { label: "Utilization", value, detail }
            }
        }
    }
}
