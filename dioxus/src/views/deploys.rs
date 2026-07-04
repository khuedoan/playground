use crate::components::button::{Button, ButtonVariant};
use crate::components::card::{Card, CardContent, CardHeader, CardTitle};
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
        div { class: "page-header-with-action",
            PageTitle {
                title: "Private links",
                subtitle: "Project-to-project network intent."
            }
            div { class: "header-actions",
                Button { variant: ButtonVariant::Outline,
                    Terminal { size: 16 }
                    "Audit log"
                }
                Button {
                    Link2 { size: 16 }
                    "Request link"
                }
            }
        }

        section { class: "deploy-overview",
            Card {
                CardHeader { CardTitle { "Two-way configuration" } }
                CardContent {
                    div { class: "rollout-card",
                        Shield { size: 28 }
                        div {
                            strong { "Source request plus target allow" }
                            span { "A link is usable only after both project configs agree." }
                        }
                        StatusBadge { status: "Active".to_string() }
                    }
                }
            }
            Card {
                CardHeader { CardTitle { "Space boundary" } }
                CardContent {
                    div { class: "source-card",
                        Network { size: 28 }
                        div {
                            strong { "Same-space traffic only" }
                            span { "Different compute spaces block component traffic even when project intent exists." }
                        }
                    }
                }
            }
        }

        Card {
            CardHeader {
                CardTitle { "Link inventory" }
            }
            CardContent {
                div { class: "data-table private-link-table",
                    div { class: "table-header",
                        span { "Projects" }
                        span { "Spaces" }
                        span { "Source" }
                        span { "Target" }
                        span { "Observed" }
                        span { "Status" }
                    }
                    for link in PRIVATE_LINKS {
                        div { class: "table-row",
                            div { class: "table-primary",
                                strong { "{link.source} -> {link.target}" }
                                span { "Project graph edge" }
                            }
                            span { "{link.source_space} -> {link.target_space}" }
                            StatusBadge { status: link.source_request.to_string() }
                            StatusBadge { status: link.target_allow.to_string() }
                            span { "{link.observed}" }
                            StatusBadge { status: link.status.to_string() }
                        }
                    }
                }
            }
        }

        Card {
            CardHeader {
                CardTitle {
                    GitBranch { size: 18 }
                    "How links affect topology"
                }
            }
            CardContent {
                div { class: "compact-list",
                    ExplainRow {
                        title: "Project graph",
                        detail: "Edges come from private-link intent and observed traffic between projects."
                    }
                    ExplainRow {
                        title: "Component graph",
                        detail: "Edges come from network telemetry and configuration references inside the selected environment."
                    }
                    ExplainRow {
                        title: "Enforcement",
                        detail: "Project-level links do not override space isolation."
                    }
                }
            }
        }
    }
}

#[component]
fn ExplainRow(title: String, detail: String) -> Element {
    rsx! {
        div { class: "compact-row",
            div {
                strong { "{title}" }
                span { "{detail}" }
            }
            Link2 { size: 18 }
        }
    }
}
