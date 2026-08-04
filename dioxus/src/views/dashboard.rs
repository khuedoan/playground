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
        PageTitle { title: "Overview", subtitle: "Acme Retail tenant." }

        section { class: "mb-5 grid gap-4 sm:grid-cols-2 xl:grid-cols-4",
            MetricCard { title: "Projects", value: "8", detail: "19 environments", icon: "projects" }
            MetricCard { title: "Components", value: "64", detail: "Telemetry graph active", icon: "components" }
            MetricCard { title: "Private links", value: "12", detail: "3 waiting for target allow", icon: "links" }
            MetricCard { title: "Spaces", value: "3", detail: "1 hosted, 2 tenant owned", icon: "spaces" }
        }

        section { class: "grid gap-5 xl:grid-cols-[3fr_2fr]",
            div { class: "card border border-base-300 bg-base-100 xl:row-span-2",
                div { class: "card-body",
                    div { class: "flex items-center justify-between gap-3",
                        h2 { class: "card-title", "Tenant hierarchy" }
                        button { class: "btn btn-outline btn-sm",
                            "Open projects"
                            ArrowUpRight { size: 14 }
                        }
                    }
                    div { class: "divide-y divide-base-300",
                        HierarchyRow { icon: "tenant", title: "Acme Retail", detail: "Tenant workspace with access to hosted-us-east, acme-pci-prod, acme-eu-core." }
                        HierarchyRow { icon: "project", title: "Projects", detail: "Checkout, identity, catalog, analytics, support, notifications, billing, observability." }
                        HierarchyRow { icon: "environment", title: "Environments", detail: "Each project carries prod, staging, or dev configuration and placement." }
                        HierarchyRow { icon: "component", title: "Components", detail: "Apps, workers, stores, queues, secrets, and external endpoints." }
                    }
                }
            }

            div { class: "card border border-base-300 bg-base-100",
                div { class: "card-body",
                    h2 { class: "card-title", "Project health" }
                    div { class: "divide-y divide-base-300",
                        for project in PROJECTS { ProjectHealthRow { project } }
                    }
                }
            }

            div { class: "card border border-base-300 bg-base-100",
                div { class: "card-body",
                    h2 { class: "card-title", "Private link handshakes" }
                    div { class: "divide-y divide-base-300",
                        for link in LINKS { LinkRow { link } }
                    }
                }
            }

            div { class: "card border border-base-300 bg-base-100 xl:col-span-2",
                div { class: "card-body",
                    h2 { class: "card-title", "Space placement" }
                    div { class: "grid gap-3 md:grid-cols-3",
                        for space in SPACES { SpacePill { space } }
                    }
                }
            }
        }
    }
}

#[component]
fn MetricCard(title: String, value: String, detail: String, icon: String) -> Element {
    rsx! {
        div { class: "card border border-base-300 bg-base-100",
            div { class: "card-body gap-3",
                div { class: "flex items-center justify-between",
                    span { class: "btn btn-square btn-ghost pointer-events-none",
                        if icon == "projects" { Boxes { size: 18 } }
                        else if icon == "components" { Network { size: 18 } }
                        else if icon == "links" { Link2 { size: 18 } }
                        else { ServerCog { size: 18 } }
                    }
                    span { class: "badge badge-ghost badge-sm", "Live" }
                }
                div { class: "stat p-0",
                    div { class: "stat-title", "{title}" }
                    div { class: "stat-value text-3xl", "{value}" }
                    div { class: "stat-desc", "{detail}" }
                }
            }
        }
    }
}

#[component]
fn HierarchyRow(icon: String, title: String, detail: String) -> Element {
    rsx! {
        div { class: "flex gap-3 py-4",
            span { class: "mt-1 text-base-content/60",
                if icon == "tenant" { Shield { size: 18 } }
                else if icon == "project" { Boxes { size: 18 } }
                else if icon == "environment" { GitBranch { size: 18 } }
                else { Braces { size: 18 } }
            }
            div {
                strong { class: "block", "{title}" }
                span { class: "text-sm text-base-content/60", "{detail}" }
            }
        }
    }
}

#[component]
fn ProjectHealthRow(project: ProjectSignal) -> Element {
    rsx! {
        div { class: "grid gap-3 py-4 sm:grid-cols-[1fr_auto]",
            div {
                strong { class: "block", "{project.name}" }
                span { class: "text-sm text-base-content/60", "{project.environments} in {project.space}" }
            }
            StatusBadge { status: project.status }
            div { class: "sm:col-span-2",
                UsageMeter { label: "Space load", value: project.utilization, detail: "Current placement" }
            }
        }
    }
}

#[component]
fn LinkRow(link: LinkSignal) -> Element {
    rsx! {
        div { class: "flex items-start justify-between gap-3 py-4",
            div {
                strong { class: "block", "{link.source} -> {link.target}" }
                span { class: "text-sm text-base-content/60", "{link.detail}" }
            }
            StatusBadge { status: link.status }
        }
    }
}

#[component]
fn SpacePill(space: SpaceSignal) -> Element {
    rsx! {
        div { class: "flex gap-3 rounded-box border border-base-300 p-4",
            Cloud { size: 18 }
            div {
                strong { class: "block", "{space.name}" }
                span { class: "text-sm text-base-content/60", "{space.kind} - {space.detail}" }
            }
        }
    }
}
