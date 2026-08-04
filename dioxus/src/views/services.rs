use crate::views::common::{EmptyState, PageTitle, StatusBadge, UsageMeter};
use crate::views::ProjectComponentGraph;
use crate::Route;
use dioxus::prelude::*;
use dioxus_icons::lucide::{
    ArrowLeft, Boxes, Braces, Cloud, Link2, Plus, ServerCog, Shield, Terminal,
};

#[derive(Clone, Copy, PartialEq)]
struct Project {
    slug: &'static str,
    name: &'static str,
    owner: &'static str,
    environments: &'static str,
    default_environment: &'static str,
    spaces: &'static str,
    primary_space: &'static str,
    components: &'static str,
    private_links: &'static str,
    status: &'static str,
    traffic: &'static str,
    saturation: f64,
    repo: &'static str,
}

const PROJECTS: [Project; 8] = [
    Project {
        slug: "checkout",
        name: "checkout",
        owner: "Commerce",
        environments: "prod, staging, dev",
        default_environment: "prod",
        spaces: "hosted-us-east, acme-pci-prod",
        primary_space: "acme-pci-prod",
        components: "9",
        private_links: "3",
        status: "Healthy",
        traffic: "7.2M req",
        saturation: 68.0,
        repo: "acme/checkout",
    },
    Project {
        slug: "identity",
        name: "identity",
        owner: "Platform",
        environments: "prod, staging",
        default_environment: "prod",
        spaces: "hosted-us-east",
        primary_space: "hosted-us-east",
        components: "6",
        private_links: "5",
        status: "Healthy",
        traffic: "9.8M req",
        saturation: 54.0,
        repo: "acme/identity",
    },
    Project {
        slug: "catalog",
        name: "catalog",
        owner: "Commerce",
        environments: "prod, staging",
        default_environment: "prod",
        spaces: "hosted-us-east",
        primary_space: "hosted-us-east",
        components: "7",
        private_links: "4",
        status: "Warning",
        traffic: "4.1M req",
        saturation: 81.0,
        repo: "acme/catalog",
    },
    Project {
        slug: "analytics",
        name: "analytics",
        owner: "Data",
        environments: "prod",
        default_environment: "prod",
        spaces: "acme-eu-core",
        primary_space: "acme-eu-core",
        components: "8",
        private_links: "2",
        status: "Syncing",
        traffic: "1.9M events",
        saturation: 44.0,
        repo: "acme/analytics",
    },
    Project {
        slug: "support",
        name: "support",
        owner: "Customer Ops",
        environments: "prod, staging",
        default_environment: "prod",
        spaces: "hosted-us-east",
        primary_space: "hosted-us-east",
        components: "5",
        private_links: "1",
        status: "Ready",
        traffic: "612K req",
        saturation: 36.0,
        repo: "acme/support",
    },
    Project {
        slug: "notifications",
        name: "notifications",
        owner: "Platform",
        environments: "prod, dev",
        default_environment: "prod",
        spaces: "hosted-us-east",
        primary_space: "hosted-us-east",
        components: "6",
        private_links: "2",
        status: "Healthy",
        traffic: "2.8M msg",
        saturation: 47.0,
        repo: "acme/notifications",
    },
    Project {
        slug: "billing",
        name: "billing",
        owner: "Finance Systems",
        environments: "prod, staging",
        default_environment: "prod",
        spaces: "acme-pci-prod",
        primary_space: "acme-pci-prod",
        components: "8",
        private_links: "3",
        status: "Healthy",
        traffic: "1.3M req",
        saturation: 59.0,
        repo: "acme/billing",
    },
    Project {
        slug: "observability",
        name: "observability",
        owner: "SRE",
        environments: "prod",
        default_environment: "prod",
        spaces: "hosted-us-east, acme-eu-core",
        primary_space: "hosted-us-east",
        components: "15",
        private_links: "0",
        status: "Healthy",
        traffic: "38K spans/s",
        saturation: 62.0,
        repo: "acme/observability",
    },
];

#[component]
pub fn Projects() -> Element {
    let mut filter = use_signal(|| "all");

    rsx! {
        div { class: "mb-6 flex flex-col gap-3 sm:flex-row sm:items-start sm:justify-between",
            PageTitle { title: "Projects", subtitle: "Tenant-scoped application groups." }
            div { class: "flex gap-2",
                button { class: "btn btn-outline", Link2 { size: 16 } "Request link" }
                button { class: "btn btn-primary", Plus { size: 16 } "Create project" }
            }
        }

        section { class: "mb-5 grid gap-4 md:grid-cols-3",
            SummaryTile { icon: "boxes", title: "Projects", value: "8", detail: "19 environments" }
            SummaryTile { icon: "braces", title: "Components", value: "64", detail: "Across all projects" }
            SummaryTile { icon: "cloud", title: "Spaces used", value: "3", detail: "2 tenant-owned" }
        }

        div { class: "card border border-base-300 bg-base-100",
            div { class: "card-body",
                h2 { class: "card-title", "Project inventory" }
                div { role: "tablist", class: "tabs tabs-box w-fit",
                    for (value, label) in [("all", "All"), ("hosted-us-east", "Hosted space"), ("dedicated", "Dedicated space")] {
                        button {
                            role: "tab",
                            class: if filter() == value { "tab tab-active" } else { "tab" },
                            onclick: move |_| filter.set(value),
                            "{label}"
                        }
                    }
                }
                ProjectsTable { filter: filter().to_string() }
            }
        }
    }
}

#[component]
fn SummaryTile(icon: String, title: String, value: String, detail: String) -> Element {
    rsx! {
        div { class: "card border border-base-300 bg-base-100",
            div { class: "card-body flex-row items-center",
                span { class: "btn btn-square btn-ghost pointer-events-none",
                    if icon == "boxes" { Boxes { size: 18 } }
                    else if icon == "braces" { Braces { size: 18 } }
                    else { Cloud { size: 18 } }
                }
                div {
                    span { class: "text-sm text-base-content/60", "{title}" }
                    strong { class: "block text-2xl", "{value}" }
                    p { class: "text-sm text-base-content/60", "{detail}" }
                }
            }
        }
    }
}

#[component]
fn ProjectsTable(filter: String) -> Element {
    let visible = PROJECTS
        .iter()
        .filter(|project| matches_filter(project, &filter));

    rsx! {
        div { class: "overflow-x-auto",
            table { class: "table table-zebra",
                thead { tr {
                    th { "Project" } th { "Owner" } th { "Spaces" } th { "Links" } th { "Status" } th { "Load" }
                } }
                tbody {
                    for project in visible {
                        tr { class: "hover:bg-base-200",
                            td {
                                Link { to: Route::ProjectDetail { slug: project.slug.to_string() }, class: "link link-hover font-semibold", "{project.name}" }
                                span { class: "block text-xs text-base-content/60", "{project.environments}" }
                            }
                            td { "{project.owner}" }
                            td { "{project.spaces}" }
                            td { "{project.private_links}" }
                            td { class: "space-x-1",
                                StatusBadge { status: project.status }
                                span { class: "badge badge-outline badge-sm", "{project.components} components" }
                            }
                            td { UsageMeter { label: "Space", value: project.saturation, detail: "Placement load" } }
                        }
                    }
                }
            }
        }
    }
}

fn matches_filter(project: &&Project, filter: &str) -> bool {
    filter == "all"
        || project.spaces.contains(filter)
        || filter == "dedicated" && !project.spaces.contains("hosted-us-east")
}

#[component]
pub fn ProjectDetail(slug: String) -> Element {
    let Some(project) = PROJECTS
        .iter()
        .copied()
        .find(|project| project.slug == slug)
    else {
        return rsx! {
            PageTitle { title: "Project not found", subtitle: "" }
            EmptyState { title: "No matching project", detail: slug }
        };
    };

    rsx! {
        div { class: "mb-6 flex flex-col gap-3 sm:flex-row sm:items-start sm:justify-between",
            div {
                Link { to: Route::Projects {}, class: "link link-hover mb-3 inline-flex items-center gap-2 text-sm",
                    ArrowLeft { size: 16 } "Projects"
                }
                PageTitle {
                    title: project.name,
                    subtitle: format!("{} - {} - {}", project.owner, project.default_environment, project.primary_space)
                }
            }
            div { class: "flex gap-2",
                button { class: "btn btn-outline", Terminal { size: 16 } "Logs" }
                button { class: "btn btn-primary", Link2 { size: 16 } "Add private link" }
            }
        }

        section { class: "mb-5 grid gap-4 md:grid-cols-3",
            DetailSummary { title: "Status", value: project.traffic, detail: "current environment traffic", status: project.status }
            DetailSummary { title: "Components", value: project.components, detail: "apps, stores, workers, secrets", icon: "components" }
            DetailSummary { title: "Spaces", value: project.primary_space, detail: project.spaces, icon: "spaces" }
        }

        ProjectComponentGraph { project_slug: project.slug, project_name: project.name }

        section { class: "grid gap-5 lg:grid-cols-2",
            DetailCard { title: "Project model",
                DetailItem { label: "Tenant", value: "Acme Retail" }
                DetailItem { label: "Owner", value: project.owner }
                DetailItem { label: "Repository", value: project.repo }
                DetailItem { label: "Environments", value: project.environments }
                DetailItem { label: "Private links", value: project.private_links }
            }
            DetailCard { title: "Environment placement",
                DetailItem { label: "prod", value: project.primary_space }
                DetailItem { label: "staging", value: "hosted-us-east" }
                DetailItem { label: "dev", value: "hosted-us-east" }
                DetailItem { label: "Rule", value: "Components can talk only inside the same space" }
            }
            DetailCard { title: "Dependency sources",
                DependencySource { source: "Network telemetry", detail: "mTLS flow records generate runtime edges." }
                DependencySource { source: "Vault references", detail: "Secret paths generate config edges." }
                DependencySource { source: "Private links", detail: "Project edges require source request and target allow." }
            }
            DetailCard { title: "Access guardrails",
                UsageMeter { label: "Space utilization", value: project.saturation, detail: "Primary space" }
                DetailItem { label: "Default environment", value: project.default_environment }
                DetailItem { label: "Ingress", value: "Private by default" }
                DetailItem { label: "Config", value: "Vault path references tracked" }
            }
        }
    }
}

#[component]
fn DetailSummary(
    title: String,
    value: String,
    detail: String,
    status: Option<String>,
    icon: Option<String>,
) -> Element {
    rsx! {
        div { class: "card border border-base-300 bg-base-100",
            div { class: "card-body",
                h2 { class: "card-title", "{title}" }
                if let Some(status) = status { StatusBadge { status } }
                if let Some(icon) = icon {
                    if icon == "components" { Braces { size: 22 } } else { ServerCog { size: 22 } }
                }
                strong { class: "text-2xl", "{value}" }
                span { class: "text-sm text-base-content/60", "{detail}" }
            }
        }
    }
}

#[component]
fn DetailCard(title: String, children: Element) -> Element {
    rsx! {
        div { class: "card border border-base-300 bg-base-100",
            div { class: "card-body",
                h2 { class: "card-title", "{title}" }
                div { class: "divide-y divide-base-300", {children} }
            }
        }
    }
}

#[component]
fn DetailItem(label: String, value: String) -> Element {
    rsx! {
        div { class: "grid gap-2 py-3 sm:grid-cols-[10rem_1fr]",
            span { class: "text-sm text-base-content/60", "{label}" }
            strong { class: "text-sm", "{value}" }
        }
    }
}

#[component]
fn DependencySource(source: String, detail: String) -> Element {
    rsx! {
        div { class: "flex items-center justify-between gap-3 py-3",
            div {
                strong { class: "block", "{source}" }
                span { class: "text-sm text-base-content/60", "{detail}" }
            }
            Shield { size: 18 }
        }
    }
}
