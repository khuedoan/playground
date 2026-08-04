use crate::views::common::{PageTitle, StatusBadge};
use dioxus::prelude::*;
use dioxus_icons::lucide::{GitBranch, Link2, Network, Shield, Terminal};

#[derive(Clone, Copy)]
struct PrivateLink {
    source: &'static str,
    target: &'static str,
    source_space: &'static str,
    target_space: &'static str,
    source_request: &'static str,
    target_allow: &'static str,
    observed: &'static str,
    status: &'static str,
}

const PRIVATE_LINKS: [PrivateLink; 8] = [
    PrivateLink {
        source: "checkout",
        target: "identity",
        source_space: "acme-pci-prod",
        target_space: "acme-pci-prod",
        source_request: "Requested",
        target_allow: "Allowed",
        observed: "18K rpm",
        status: "Linked",
    },
    PrivateLink {
        source: "checkout",
        target: "billing",
        source_space: "acme-pci-prod",
        target_space: "acme-pci-prod",
        source_request: "Requested",
        target_allow: "Allowed",
        observed: "4K rpm",
        status: "Linked",
    },
    PrivateLink {
        source: "checkout",
        target: "catalog",
        source_space: "hosted-us-east",
        target_space: "hosted-us-east",
        source_request: "Requested",
        target_allow: "Needs allow",
        observed: "0 rpm",
        status: "Needs allow",
    },
    PrivateLink {
        source: "support",
        target: "identity",
        source_space: "hosted-us-east",
        target_space: "hosted-us-east",
        source_request: "Requested",
        target_allow: "Pending",
        observed: "0 rpm",
        status: "Requested",
    },
    PrivateLink {
        source: "notifications",
        target: "identity",
        source_space: "hosted-us-east",
        target_space: "hosted-us-east",
        source_request: "Requested",
        target_allow: "Allowed",
        observed: "11K rpm",
        status: "Linked",
    },
    PrivateLink {
        source: "analytics",
        target: "catalog",
        source_space: "acme-eu-core",
        target_space: "hosted-us-east",
        source_request: "Requested",
        target_allow: "Allowed",
        observed: "0 rpm",
        status: "Blocked",
    },
    PrivateLink {
        source: "billing",
        target: "notifications",
        source_space: "acme-pci-prod",
        target_space: "acme-pci-prod",
        source_request: "Requested",
        target_allow: "Allowed",
        observed: "2K rpm",
        status: "Linked",
    },
    PrivateLink {
        source: "catalog",
        target: "observability",
        source_space: "hosted-us-east",
        target_space: "hosted-us-east",
        source_request: "Requested",
        target_allow: "Drift",
        observed: "312 rpm",
        status: "Drift",
    },
];

#[component]
pub fn PrivateLinks() -> Element {
    rsx! {
        div { class: "mb-6 flex flex-col gap-3 sm:flex-row sm:items-start sm:justify-between",
            PageTitle { title: "Private links", subtitle: "Project-to-project network intent." }
            div { class: "flex gap-2",
                button { class: "btn btn-outline", Terminal { size: 16 } "Audit log" }
                button { class: "btn btn-primary", Link2 { size: 16 } "Request link" }
            }
        }

        section { class: "mb-5 grid gap-4 lg:grid-cols-2",
            InfoCard {
                title: "Two-way configuration",
                detail: "A link is usable only after both project configs agree.",
                icon: "shield",
                badge: "Active"
            }
            InfoCard {
                title: "Space boundary",
                detail: "Different compute spaces block component traffic even when project intent exists.",
                icon: "network"
            }
        }

        div { class: "card mb-5 border border-base-300 bg-base-100",
            div { class: "card-body",
                h2 { class: "card-title", "Link inventory" }
                div { class: "overflow-x-auto",
                    table { class: "table table-zebra",
                        thead { tr {
                            th { "Projects" } th { "Spaces" } th { "Source" } th { "Target" } th { "Observed" } th { "Status" }
                        } }
                        tbody {
                            for link in PRIVATE_LINKS {
                                tr {
                                    td {
                                        strong { class: "block", "{link.source} -> {link.target}" }
                                        span { class: "text-xs text-base-content/60", "Project graph edge" }
                                    }
                                    td { "{link.source_space} -> {link.target_space}" }
                                    td { StatusBadge { status: link.source_request } }
                                    td { StatusBadge { status: link.target_allow } }
                                    td { "{link.observed}" }
                                    td { StatusBadge { status: link.status } }
                                }
                            }
                        }
                    }
                }
            }
        }

        div { class: "card border border-base-300 bg-base-100",
            div { class: "card-body",
                h2 { class: "card-title", GitBranch { size: 18 } "How links affect topology" }
                div { class: "divide-y divide-base-300",
                    ExplainRow { title: "Project graph", detail: "Edges come from private-link intent and observed traffic between projects." }
                    ExplainRow { title: "Component graph", detail: "Edges come from network telemetry and configuration references inside the selected environment." }
                    ExplainRow { title: "Enforcement", detail: "Project-level links do not override space isolation." }
                }
            }
        }
    }
}

#[component]
fn InfoCard(title: String, detail: String, icon: String, badge: Option<String>) -> Element {
    rsx! {
        div { class: "card border border-base-300 bg-base-100",
            div { class: "card-body",
                div { class: "flex items-center gap-3",
                    if icon == "shield" { Shield { size: 28 } } else { Network { size: 28 } }
                    div { class: "flex-1",
                        h2 { class: "card-title", "{title}" }
                        p { class: "text-sm text-base-content/60", "{detail}" }
                    }
                    if let Some(badge) = badge { StatusBadge { status: badge } }
                }
            }
        }
    }
}

#[component]
fn ExplainRow(title: String, detail: String) -> Element {
    rsx! {
        div { class: "flex items-center justify-between gap-3 py-4",
            div {
                strong { class: "block", "{title}" }
                span { class: "text-sm text-base-content/60", "{detail}" }
            }
            Link2 { size: 18 }
        }
    }
}
