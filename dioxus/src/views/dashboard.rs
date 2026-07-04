use crate::components::badge::{Badge, BadgeVariant};
use crate::components::button::{Button, ButtonVariant};
use crate::components::card::{Card, CardAction, CardContent, CardHeader, CardTitle};
use crate::views::common::{PageTitle, StatusBadge, UsageMeter};
use dioxus::prelude::*;
use dioxus_icons::lucide::{
    ArrowUpRight, Boxes, Braces, Cloud, GitBranch, Link2, Network, ServerCog, Shield,
};

#[derive(Clone, Copy, PartialEq)]
struct ProjectSignal {
    name: &'static str,
    environments: &'static str,
    space: &'static str,
    status: &'static str,
    utilization: f64,
}

#[derive(Clone, Copy, PartialEq)]
struct LinkSignal {
    source: &'static str,
    target: &'static str,
    status: &'static str,
    detail: &'static str,
}

#[derive(Clone, Copy, PartialEq)]
struct SpaceSignal {
    name: &'static str,
    kind: &'static str,
    detail: &'static str,
}

const PROJECTS: [ProjectSignal; 4] = [
    ProjectSignal {
        name: "checkout",
        environments: "prod, staging, dev",
        space: "acme-pci-prod",
        status: "Healthy",
        utilization: 68.0,
    },
    ProjectSignal {
        name: "identity",
        environments: "prod, staging",
        space: "hosted-us-east",
        status: "Healthy",
        utilization: 52.0,
    },
    ProjectSignal {
        name: "catalog",
        environments: "prod, staging",
        space: "hosted-us-east",
        status: "Warning",
        utilization: 81.0,
    },
    ProjectSignal {
        name: "analytics",
        environments: "prod",
        space: "acme-eu-core",
        status: "Syncing",
        utilization: 44.0,
    },
];

const LINKS: [LinkSignal; 4] = [
    LinkSignal {
        source: "checkout",
        target: "identity",
        status: "Linked",
        detail: "Both sides allowed. Same space path active.",
    },
    LinkSignal {
        source: "checkout",
        target: "catalog",
        status: "Needs allow",
        detail: "Source requested. Target allow rule missing.",
    },
    LinkSignal {
        source: "analytics",
        target: "catalog",
        status: "Blocked",
        detail: "Different spaces. No component traffic permitted.",
    },
    LinkSignal {
        source: "support",
        target: "identity",
        status: "Requested",
        detail: "Target review pending.",
    },
];

const SPACES: [SpaceSignal; 3] = [
    SpaceSignal {
        name: "hosted-us-east",
        kind: "Shared",
        detail: "Default hosted data plane.",
    },
    SpaceSignal {
        name: "acme-pci-prod",
        kind: "Dedicated",
        detail: "Tenant-owned PCI workloads.",
    },
    SpaceSignal {
        name: "acme-eu-core",
        kind: "Dedicated",
        detail: "EU data residency boundary.",
    },
];

#[component]
pub fn Dashboard() -> Element {
    rsx! {
        PageTitle {
            title: "Overview",
            subtitle: "Acme Retail tenant."
        }

        section { class: "metric-grid",
            MetricCard {
                title: "Projects",
                value: "8",
                detail: "19 environments",
                Boxes { size: 18 }
            }
            MetricCard {
                title: "Components",
                value: "64",
                detail: "Telemetry graph active",
                Network { size: 18 }
            }
            MetricCard {
                title: "Private links",
                value: "12",
                detail: "3 waiting for target allow",
                Link2 { size: 18 }
            }
            MetricCard {
                title: "Spaces",
                value: "3",
                detail: "1 hosted, 2 tenant owned",
                ServerCog { size: 18 }
            }
        }

        section { class: "dashboard-grid",
            Card { class: "wide-card",
                CardHeader {
                    CardTitle { "Tenant hierarchy" }
                    CardAction {
                        Button { variant: ButtonVariant::Outline,
                            "Open projects"
                            ArrowUpRight { size: 14 }
                        }
                    }
                }
                CardContent {
                    div { class: "hierarchy-list",
                        HierarchyRow {
                            icon: "tenant",
                            title: "Acme Retail",
                            detail: "Tenant workspace with access to hosted-us-east, acme-pci-prod, acme-eu-core."
                        }
                        HierarchyRow {
                            icon: "project",
                            title: "Projects",
                            detail: "Checkout, identity, catalog, analytics, support, notifications, billing, observability."
                        }
                        HierarchyRow {
                            icon: "environment",
                            title: "Environments",
                            detail: "Each project carries prod, staging, or dev configuration and placement."
                        }
                        HierarchyRow {
                            icon: "component",
                            title: "Components",
                            detail: "Apps, workers, stores, queues, secrets, and external endpoints."
                        }
                    }
                }
            }

            Card {
                CardHeader {
                    CardTitle { "Project health" }
                }
                CardContent {
                    div { class: "service-list",
                        for project in PROJECTS {
                            ProjectHealthRow { project }
                        }
                    }
                }
            }

            Card {
                CardHeader {
                    CardTitle { "Private link handshakes" }
                }
                CardContent {
                    div { class: "compact-list",
                        for link in LINKS {
                            LinkRow { link }
                        }
                    }
                }
            }

            Card {
                CardHeader {
                    CardTitle { "Space placement" }
                }
                CardContent {
                    div { class: "region-map",
                        for space in SPACES {
                            SpacePill { space }
                        }
                    }
                }
            }
        }
    }
}

#[component]
fn MetricCard(title: String, value: String, detail: String, children: Element) -> Element {
    rsx! {
        Card { class: "metric-card",
            CardHeader {
                div { class: "metric-icon", {children} }
                Badge { variant: BadgeVariant::Secondary, "Live" }
            }
            CardContent {
                p { "{title}" }
                strong { "{value}" }
                span { "{detail}" }
            }
        }
    }
}

#[component]
fn HierarchyRow(icon: String, title: String, detail: String) -> Element {
    rsx! {
        div { class: "compact-row",
            div { class: "row-with-icon",
                if icon == "tenant" {
                    Shield { size: 18 }
                } else if icon == "project" {
                    Boxes { size: 18 }
                } else if icon == "environment" {
                    GitBranch { size: 18 }
                } else {
                    Braces { size: 18 }
                }
                div {
                    strong { "{title}" }
                    span { "{detail}" }
                }
            }
        }
    }
}

#[component]
fn ProjectHealthRow(project: ProjectSignal) -> Element {
    rsx! {
        div { class: "service-row",
            div { class: "service-identity",
                strong { "{project.name}" }
                span { "{project.environments} in {project.space}" }
            }
            StatusBadge { status: project.status.to_string() }
            UsageMeter {
                label: "Space load",
                value: project.utilization,
                detail: "Current placement"
            }
        }
    }
}

#[component]
fn LinkRow(link: LinkSignal) -> Element {
    rsx! {
        div { class: "compact-row",
            div {
                strong { "{link.source} -> {link.target}" }
                span { "{link.detail}" }
            }
            StatusBadge { status: link.status.to_string() }
        }
    }
}

#[component]
fn SpacePill(space: SpaceSignal) -> Element {
    rsx! {
        div { class: "region-pill",
            div { class: "region-icon",
                Cloud { size: 18 }
            }
            div {
                strong { "{space.name}" }
                span { "{space.kind} - {space.detail}" }
            }
        }
    }
}
