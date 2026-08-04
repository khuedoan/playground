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
        div { class: "mb-6 flex flex-col gap-3 sm:flex-row sm:items-start sm:justify-between",
            PageTitle { title: "Spaces", subtitle: "Compute data planes available to this tenant." }
            button { class: "btn btn-outline", Shield { size: 16 } "Access policy" }
        }

        section { class: "mb-5 grid gap-4 lg:grid-cols-[3fr_2fr]",
            div { class: "card border border-base-300 bg-base-100",
                div { class: "card-body",
                    div { class: "flex items-center justify-between",
                        h2 { class: "card-title", "Tenant access" }
                        span { class: "badge badge-primary", "1:N" }
                    }
                    strong { class: "text-2xl", "Acme Retail" }
                    p { class: "text-base-content/60", "This tenant can deploy environments into 3 active spaces." }
                    div { class: "card-actions", button { class: "btn btn-ghost", "Grant space access" } }
                }
            }
            div { class: "card border border-base-300 bg-base-100",
                div { class: "card-body",
                    h2 { class: "card-title", "Communication rule" }
                    Network { size: 28 }
                    strong { class: "text-2xl", "Same space only" }
                    p { class: "text-base-content/60", "Components in different spaces cannot talk directly." }
                }
            }
        }

        section { class: "mb-5 grid gap-4 md:grid-cols-3",
            SpaceSummaryCard { title: "Hosted default", amount: "hosted-us-east", icon: "cloud", value: 63.0, detail: "Shared PaaS data plane" }
            SpaceSummaryCard { title: "Enterprise owned", amount: "2 active", icon: "shield", value: 56.0, detail: "Tenant-controlled spaces" }
            SpaceSummaryCard { title: "Pending access", amount: "1 space", icon: "server", value: 8.0, detail: "Sandbox awaiting approval" }
        }

        div { class: "card border border-base-300 bg-base-100",
            div { class: "card-body",
                h2 { class: "card-title", "Space inventory" }
                div { class: "overflow-x-auto",
                    table { class: "table table-zebra",
                        thead { tr {
                            th { "Space" } th { "Kind" } th { "Region" } th { "Owner" } th { "Environments" } th { "Utilization" } th { "Status" }
                        } }
                        tbody {
                            for space in SPACES {
                                tr {
                                    td {
                                        strong { class: "block", "{space.name}" }
                                        span { class: "text-xs text-base-content/60", "Compute data plane" }
                                    }
                                    td { StatusBadge { status: space.kind } }
                                    td { "{space.region}" }
                                    td { "{space.owner}" }
                                    td { "{space.environments}" }
                                    td { UsageMeter { label: "Load", value: space.utilization, detail: "Current" } }
                                    td { StatusBadge { status: space.status } }
                                }
                            }
                        }
                    }
                }
            }
        }
    }
}

#[component]
fn SpaceSummaryCard(
    title: String,
    amount: String,
    icon: String,
    value: f64,
    detail: String,
) -> Element {
    rsx! {
        div { class: "card border border-base-300 bg-base-100",
            div { class: "card-body",
                div { class: "flex items-center gap-3",
                    if icon == "cloud" { Cloud { size: 18 } }
                    else if icon == "shield" { Shield { size: 18 } }
                    else { ServerCog { size: 18 } }
                    h2 { class: "card-title", "{title}" }
                }
                strong { class: "text-2xl", "{amount}" }
                UsageMeter { label: "Utilization", value, detail }
            }
        }
    }
}
