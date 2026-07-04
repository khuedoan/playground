use crate::components::badge::{Badge, BadgeVariant};
use crate::components::button::{Button, ButtonVariant};
use crate::components::card::{Card, CardContent, CardHeader, CardTitle};
use crate::components::tabs::{TabContent, TabList, TabTrigger, Tabs};
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
    rsx! {
        div { class: "page-header-with-action",
            PageTitle {
                title: "Projects",
                subtitle: "Tenant-scoped application groups."
            }
            div { class: "header-actions",
                Button { variant: ButtonVariant::Outline,
                    Link2 { size: 16 }
                    "Request link"
                }
                Button {
                    Plus { size: 16 }
                    "Create project"
                }
            }
        }

        section { class: "service-summary-grid",
            SummaryTile { icon: "boxes", title: "Projects", value: "8", detail: "19 environments" }
            SummaryTile { icon: "braces", title: "Components", value: "64", detail: "Across all projects" }
            SummaryTile { icon: "cloud", title: "Spaces used", value: "3", detail: "2 tenant-owned" }
        }

        Card {
            CardHeader {
                CardTitle { "Project inventory" }
            }
            CardContent {
                Tabs { default_value: "all".to_string(), horizontal: true,
                    TabList {
                        TabTrigger { value: "all".to_string(), index: 0usize, "All" }
                        TabTrigger { value: "hosted".to_string(), index: 1usize, "Hosted space" }
                        TabTrigger { value: "dedicated".to_string(), index: 2usize, "Dedicated space" }
                    }
                    TabContent { value: "all".to_string(), index: 0usize,
                        ProjectsTable { filter: "all".to_string() }
                    }
                    TabContent { value: "hosted".to_string(), index: 1usize,
                        ProjectsTable { filter: "hosted-us-east".to_string() }
                    }
                    TabContent { value: "dedicated".to_string(), index: 2usize,
                        ProjectsTable { filter: "dedicated".to_string() }
                    }
                }
            }
        }
    }
}

#[component]
fn SummaryTile(icon: String, title: String, value: String, detail: String) -> Element {
    rsx! {
        Card { class: "summary-tile",
            CardContent {
                div { class: "summary-icon",
                    if icon == "boxes" {
                        Boxes { size: 18 }
                    } else if icon == "braces" {
                        Braces { size: 18 }
                    } else {
                        Cloud { size: 18 }
                    }
                }
                div {
                    span { "{title}" }
                    strong { "{value}" }
                    p { "{detail}" }
                }
            }
        }
    }
}

#[component]
fn ProjectsTable(filter: String) -> Element {
    let visible_count = PROJECTS
        .iter()
        .filter(|project| {
            filter == "all"
                || project.spaces.contains(filter.as_str())
                || filter == "dedicated" && !project.spaces.contains("hosted-us-east")
        })
        .count();

    rsx! {
        if visible_count == 0 {
            EmptyState {
                title: "No projects found",
                detail: "Try another space filter."
            }
        } else {
            div { class: "data-table projects-table",
                div { class: "table-header",
                    span { "Project" }
                    span { "Owner" }
                    span { "Spaces" }
                    span { "Links" }
                    span { "Status" }
                    span { "Load" }
                }
                for project in PROJECTS {
                    if filter == "all"
                        || project.spaces.contains(filter.as_str())
                        || filter == "dedicated" && !project.spaces.contains("hosted-us-east")
                    {
                        Link {
                            to: Route::ProjectDetail { slug: project.slug.to_string() },
                            class: "table-row service-link-row",
                            div { class: "table-primary",
                                strong { "{project.name}" }
                                span { "{project.environments}" }
                            }
                            span { "{project.owner}" }
                            span { "{project.spaces}" }
                            span { "{project.private_links}" }
                            div { class: "status-cell",
                                StatusBadge { status: project.status.to_string() }
                                Badge { variant: BadgeVariant::Outline, "{project.components} components" }
                            }
                            UsageMeter {
                                label: "Space",
                                value: project.saturation,
                                detail: "Placement load"
                            }
                        }
                    }
                }
            }
        }
    }
}

#[component]
pub fn ProjectDetail(slug: String) -> Element {
    let Some(project) = project_by_slug(&slug) else {
        return rsx! {
            PageTitle {
                title: "Project not found",
                subtitle: String::new()
            }
            EmptyState {
                title: "No matching project",
                detail: slug
            }
        };
    };

    rsx! {
        div { class: "page-header-with-action",
            div { class: "service-title-block",
                Link { to: Route::Projects {}, class: "back-link",
                    ArrowLeft { size: 16 }
                    "Projects"
                }
                PageTitle {
                    title: project.name.to_string(),
                    subtitle: format!("{} - {} - {}", project.owner, project.default_environment, project.primary_space)
                }
            }
            div { class: "header-actions",
                Button { variant: ButtonVariant::Outline,
                    Terminal { size: 16 }
                    "Logs"
                }
                Button {
                    Link2 { size: 16 }
                    "Add private link"
                }
            }
        }

        section { class: "service-detail-grid",
            Card { class: "service-summary-card",
                CardHeader {
                    CardTitle { "Status" }
                }
                CardContent { class: "service-status-card",
                    StatusBadge { status: project.status.to_string() }
                    strong { "{project.traffic}" }
                    span { "current environment traffic" }
                }
            }
            Card { class: "service-summary-card",
                CardHeader {
                    CardTitle { "Components" }
                }
                CardContent { class: "service-icon-copy",
                    Braces { size: 22 }
                    div {
                        strong { "{project.components}" }
                        span { "apps, stores, workers, secrets" }
                    }
                }
            }
            Card { class: "service-summary-card",
                CardHeader {
                    CardTitle { "Spaces" }
                }
                CardContent { class: "service-icon-copy",
                    ServerCog { size: 22 }
                    div {
                        strong { "{project.primary_space}" }
                        span { "{project.spaces}" }
                    }
                }
            }
        }

        ProjectComponentGraph {
            project_slug: project.slug.to_string(),
            project_name: project.name.to_string()
        }

        section { class: "service-detail-grid two-column",
            Card {
                CardHeader {
                    CardTitle { "Project model" }
                }
                CardContent {
                    div { class: "detail-list",
                        DetailItem { label: "Tenant", value: "Acme Retail" }
                        DetailItem { label: "Owner", value: project.owner }
                        DetailItem { label: "Repository", value: project.repo }
                        DetailItem { label: "Environments", value: project.environments }
                        DetailItem { label: "Private links", value: project.private_links }
                    }
                }
            }
            Card {
                CardHeader {
                    CardTitle { "Environment placement" }
                }
                CardContent {
                    div { class: "detail-list",
                        DetailItem { label: "prod", value: project.primary_space }
                        DetailItem { label: "staging", value: "hosted-us-east" }
                        DetailItem { label: "dev", value: "hosted-us-east" }
                        DetailItem { label: "Rule", value: "Components can talk only inside the same space" }
                    }
                }
            }
            Card {
                CardHeader {
                    CardTitle { "Dependency sources" }
                }
                CardContent {
                    div { class: "compact-list",
                        DependencySource { source: "Network telemetry", detail: "mTLS flow records generate runtime edges." }
                        DependencySource { source: "Vault references", detail: "Secret paths generate config edges." }
                        DependencySource { source: "Private links", detail: "Project edges require source request and target allow." }
                    }
                }
            }
            Card {
                CardHeader {
                    CardTitle { "Access guardrails" }
                }
                CardContent { class: "stacked-content",
                    UsageMeter { label: "Space utilization", value: project.saturation, detail: "Primary space" }
                    DetailItem { label: "Default environment", value: project.default_environment }
                    DetailItem { label: "Ingress", value: "Private by default" }
                    DetailItem { label: "Config", value: "Vault path references tracked" }
                }
            }
        }
    }
}

fn project_by_slug(slug: &str) -> Option<Project> {
    PROJECTS
        .iter()
        .copied()
        .find(|project| project.slug == slug)
}

#[component]
fn DetailItem(label: String, value: String) -> Element {
    rsx! {
        div { class: "detail-item",
            span { "{label}" }
            strong { "{value}" }
        }
    }
}

#[component]
fn DependencySource(source: String, detail: String) -> Element {
    rsx! {
        div { class: "compact-row",
            div {
                strong { "{source}" }
                span { "{detail}" }
            }
            Shield { size: 18 }
        }
    }
}
