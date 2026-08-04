use crate::views::common::{PageTitle, StatusBadge};
use crate::Route;
use dioxus::prelude::*;
use dioxus_icons::lucide::{
    Activity, Boxes, Database, GitBranch, Globe, Link2, Network, Search, Server, Shield, Terminal,
};
use std::collections::{HashMap, VecDeque};

#[derive(Clone, Copy, PartialEq)]
struct GraphNode {
    id: &'static str,
    label: &'static str,
    kind: &'static str,
    status: &'static str,
    detail: &'static str,
    metric: &'static str,
    project: &'static str,
    environment: &'static str,
    space: &'static str,
    project_slug: Option<&'static str>,
}

#[derive(Clone, Copy, PartialEq)]
struct DependencyEdge {
    from: &'static str,
    to: &'static str,
    relation: &'static str,
    source: &'static str,
}

#[derive(Clone, Copy, PartialEq)]
struct PositionedNode {
    node: GraphNode,
    x: f64,
    y: f64,
}

#[derive(Clone, Copy, PartialEq)]
struct PositionedEdge {
    from: &'static str,
    to: &'static str,
    from_x: f64,
    from_y: f64,
    mid_y: f64,
    to_x: f64,
    to_y: f64,
    relation: &'static str,
    source: &'static str,
}

struct GraphLayout {
    nodes: Vec<PositionedNode>,
    edges: Vec<PositionedEdge>,
}

const COMPONENT_NODES: [GraphNode; 27] = [
    GraphNode {
        id: "public-ingress",
        label: "public-ingress",
        kind: "network",
        status: "Active",
        detail: "edge router",
        metric: "7.2M req",
        project: "checkout",
        environment: "prod",
        space: "acme-pci-prod",
        project_slug: Some("checkout"),
    },
    GraphNode {
        id: "checkout-web",
        label: "checkout-web",
        kind: "app",
        status: "Healthy",
        detail: "web",
        metric: "6 replicas",
        project: "checkout",
        environment: "prod",
        space: "acme-pci-prod",
        project_slug: Some("checkout"),
    },
    GraphNode {
        id: "checkout-api",
        label: "checkout-api",
        kind: "app",
        status: "Healthy",
        detail: "rust api",
        metric: "18K rpm",
        project: "checkout",
        environment: "prod",
        space: "acme-pci-prod",
        project_slug: Some("checkout"),
    },
    GraphNode {
        id: "checkout-worker",
        label: "checkout-worker",
        kind: "worker",
        status: "Healthy",
        detail: "orders",
        metric: "9 jobs",
        project: "checkout",
        environment: "prod",
        space: "acme-pci-prod",
        project_slug: Some("checkout"),
    },
    GraphNode {
        id: "checkout-queue",
        label: "checkout-queue",
        kind: "queue",
        status: "Ready",
        detail: "redis stream",
        metric: "1.8K msg",
        project: "checkout",
        environment: "prod",
        space: "acme-pci-prod",
        project_slug: Some("checkout"),
    },
    GraphNode {
        id: "checkout-db",
        label: "checkout-db",
        kind: "database",
        status: "Healthy",
        detail: "postgres",
        metric: "42% disk",
        project: "checkout",
        environment: "prod",
        space: "acme-pci-prod",
        project_slug: Some("checkout"),
    },
    GraphNode {
        id: "checkout-vault",
        label: "checkout-vault",
        kind: "secret",
        status: "Healthy",
        detail: "secret/data/checkout/prod",
        metric: "14 refs",
        project: "checkout",
        environment: "prod",
        space: "acme-pci-prod",
        project_slug: Some("checkout"),
    },
    GraphNode {
        id: "identity-api",
        label: "identity-api",
        kind: "app",
        status: "Healthy",
        detail: "oidc",
        metric: "22K rpm",
        project: "identity",
        environment: "prod",
        space: "acme-pci-prod",
        project_slug: Some("identity"),
    },
    GraphNode {
        id: "identity-db",
        label: "identity-db",
        kind: "database",
        status: "Healthy",
        detail: "postgres",
        metric: "35% disk",
        project: "identity",
        environment: "prod",
        space: "acme-pci-prod",
        project_slug: Some("identity"),
    },
    GraphNode {
        id: "identity-vault",
        label: "identity-vault",
        kind: "secret",
        status: "Healthy",
        detail: "secret/data/identity/prod",
        metric: "9 refs",
        project: "identity",
        environment: "prod",
        space: "acme-pci-prod",
        project_slug: Some("identity"),
    },
    GraphNode {
        id: "billing-api",
        label: "billing-api",
        kind: "app",
        status: "Healthy",
        detail: "go api",
        metric: "4K rpm",
        project: "billing",
        environment: "prod",
        space: "acme-pci-prod",
        project_slug: Some("billing"),
    },
    GraphNode {
        id: "billing-worker",
        label: "billing-worker",
        kind: "worker",
        status: "Deploying",
        detail: "invoice jobs",
        metric: "4 replicas",
        project: "billing",
        environment: "prod",
        space: "acme-pci-prod",
        project_slug: Some("billing"),
    },
    GraphNode {
        id: "billing-db",
        label: "billing-db",
        kind: "database",
        status: "Healthy",
        detail: "postgres",
        metric: "63% disk",
        project: "billing",
        environment: "prod",
        space: "acme-pci-prod",
        project_slug: Some("billing"),
    },
    GraphNode {
        id: "billing-vault",
        label: "billing-vault",
        kind: "secret",
        status: "Healthy",
        detail: "secret/data/billing/prod",
        metric: "11 refs",
        project: "billing",
        environment: "prod",
        space: "acme-pci-prod",
        project_slug: Some("billing"),
    },
    GraphNode {
        id: "notifications-worker",
        label: "notifications-worker",
        kind: "worker",
        status: "Healthy",
        detail: "outbox",
        metric: "12K msg",
        project: "notifications",
        environment: "prod",
        space: "acme-pci-prod",
        project_slug: Some("notifications"),
    },
    GraphNode {
        id: "email-provider",
        label: "email-provider",
        kind: "network",
        status: "Active",
        detail: "external",
        metric: "99.9% sla",
        project: "notifications",
        environment: "prod",
        space: "acme-pci-prod",
        project_slug: Some("notifications"),
    },
    GraphNode {
        id: "catalog-api",
        label: "catalog-api",
        kind: "app",
        status: "Warning",
        detail: "read api",
        metric: "0 rpm",
        project: "catalog",
        environment: "prod",
        space: "hosted-us-east",
        project_slug: Some("catalog"),
    },
    GraphNode {
        id: "catalog-db",
        label: "catalog-db",
        kind: "database",
        status: "Healthy",
        detail: "postgres",
        metric: "71% disk",
        project: "catalog",
        environment: "prod",
        space: "hosted-us-east",
        project_slug: Some("catalog"),
    },
    GraphNode {
        id: "catalog-cache",
        label: "catalog-cache",
        kind: "database",
        status: "Healthy",
        detail: "redis",
        metric: "38% mem",
        project: "catalog",
        environment: "prod",
        space: "hosted-us-east",
        project_slug: Some("catalog"),
    },
    GraphNode {
        id: "catalog-vault",
        label: "catalog-vault",
        kind: "secret",
        status: "Healthy",
        detail: "secret/data/catalog/prod",
        metric: "7 refs",
        project: "catalog",
        environment: "prod",
        space: "hosted-us-east",
        project_slug: Some("catalog"),
    },
    GraphNode {
        id: "support-web",
        label: "support-web",
        kind: "app",
        status: "Ready",
        detail: "portal",
        metric: "612K req",
        project: "support",
        environment: "prod",
        space: "hosted-us-east",
        project_slug: Some("support"),
    },
    GraphNode {
        id: "support-api",
        label: "support-api",
        kind: "app",
        status: "Ready",
        detail: "tickets",
        metric: "1.1K rpm",
        project: "support",
        environment: "prod",
        space: "hosted-us-east",
        project_slug: Some("support"),
    },
    GraphNode {
        id: "support-vault",
        label: "support-vault",
        kind: "secret",
        status: "Healthy",
        detail: "secret/data/support/prod",
        metric: "5 refs",
        project: "support",
        environment: "prod",
        space: "hosted-us-east",
        project_slug: Some("support"),
    },
    GraphNode {
        id: "telemetry-agent",
        label: "telemetry-agent",
        kind: "worker",
        status: "Healthy",
        detail: "flow collector",
        metric: "38K spans/s",
        project: "observability",
        environment: "prod",
        space: "acme-pci-prod",
        project_slug: Some("observability"),
    },
    GraphNode {
        id: "audit-sink",
        label: "audit-sink",
        kind: "database",
        status: "Healthy",
        detail: "object store",
        metric: "4.2TB",
        project: "observability",
        environment: "prod",
        space: "acme-pci-prod",
        project_slug: Some("observability"),
    },
    GraphNode {
        id: "analytics-ingest",
        label: "analytics-ingest",
        kind: "worker",
        status: "Blocked",
        detail: "eu stream",
        metric: "space mismatch",
        project: "analytics",
        environment: "prod",
        space: "acme-eu-core",
        project_slug: Some("analytics"),
    },
    GraphNode {
        id: "warehouse",
        label: "warehouse",
        kind: "database",
        status: "Healthy",
        detail: "olap",
        metric: "19TB",
        project: "analytics",
        environment: "prod",
        space: "acme-eu-core",
        project_slug: Some("analytics"),
    },
];

const COMPONENT_EDGES: [DependencyEdge; 42] = [
    DependencyEdge {
        from: "public-ingress",
        to: "checkout-web",
        relation: "routes",
        source: "telemetry",
    },
    DependencyEdge {
        from: "checkout-web",
        to: "checkout-api",
        relation: "calls",
        source: "telemetry",
    },
    DependencyEdge {
        from: "checkout-web",
        to: "checkout-vault",
        relation: "reads",
        source: "vault ref",
    },
    DependencyEdge {
        from: "checkout-api",
        to: "checkout-db",
        relation: "writes",
        source: "telemetry",
    },
    DependencyEdge {
        from: "checkout-api",
        to: "checkout-queue",
        relation: "publishes",
        source: "telemetry",
    },
    DependencyEdge {
        from: "checkout-api",
        to: "checkout-vault",
        relation: "reads",
        source: "vault ref",
    },
    DependencyEdge {
        from: "checkout-api",
        to: "identity-api",
        relation: "validates",
        source: "private link",
    },
    DependencyEdge {
        from: "checkout-api",
        to: "billing-api",
        relation: "charges",
        source: "private link",
    },
    DependencyEdge {
        from: "checkout-api",
        to: "catalog-api",
        relation: "blocked",
        source: "private link",
    },
    DependencyEdge {
        from: "checkout-worker",
        to: "checkout-queue",
        relation: "consumes",
        source: "telemetry",
    },
    DependencyEdge {
        from: "checkout-worker",
        to: "checkout-db",
        relation: "writes",
        source: "telemetry",
    },
    DependencyEdge {
        from: "checkout-worker",
        to: "billing-worker",
        relation: "triggers",
        source: "private link",
    },
    DependencyEdge {
        from: "checkout-worker",
        to: "checkout-vault",
        relation: "reads",
        source: "vault ref",
    },
    DependencyEdge {
        from: "identity-api",
        to: "identity-db",
        relation: "reads",
        source: "telemetry",
    },
    DependencyEdge {
        from: "identity-api",
        to: "identity-vault",
        relation: "reads",
        source: "vault ref",
    },
    DependencyEdge {
        from: "identity-api",
        to: "telemetry-agent",
        relation: "emits",
        source: "telemetry",
    },
    DependencyEdge {
        from: "billing-api",
        to: "billing-db",
        relation: "writes",
        source: "telemetry",
    },
    DependencyEdge {
        from: "billing-api",
        to: "billing-vault",
        relation: "reads",
        source: "vault ref",
    },
    DependencyEdge {
        from: "billing-api",
        to: "notifications-worker",
        relation: "sends",
        source: "private link",
    },
    DependencyEdge {
        from: "billing-worker",
        to: "billing-db",
        relation: "writes",
        source: "telemetry",
    },
    DependencyEdge {
        from: "billing-worker",
        to: "billing-vault",
        relation: "reads",
        source: "vault ref",
    },
    DependencyEdge {
        from: "billing-worker",
        to: "notifications-worker",
        relation: "events",
        source: "telemetry",
    },
    DependencyEdge {
        from: "notifications-worker",
        to: "email-provider",
        relation: "sends",
        source: "telemetry",
    },
    DependencyEdge {
        from: "notifications-worker",
        to: "telemetry-agent",
        relation: "emits",
        source: "telemetry",
    },
    DependencyEdge {
        from: "catalog-api",
        to: "catalog-db",
        relation: "reads",
        source: "telemetry",
    },
    DependencyEdge {
        from: "catalog-api",
        to: "catalog-cache",
        relation: "caches",
        source: "telemetry",
    },
    DependencyEdge {
        from: "catalog-api",
        to: "catalog-vault",
        relation: "reads",
        source: "vault ref",
    },
    DependencyEdge {
        from: "catalog-cache",
        to: "catalog-db",
        relation: "refreshes",
        source: "telemetry",
    },
    DependencyEdge {
        from: "support-web",
        to: "support-api",
        relation: "calls",
        source: "telemetry",
    },
    DependencyEdge {
        from: "support-api",
        to: "support-vault",
        relation: "reads",
        source: "vault ref",
    },
    DependencyEdge {
        from: "support-api",
        to: "identity-api",
        relation: "requested",
        source: "private link",
    },
    DependencyEdge {
        from: "telemetry-agent",
        to: "audit-sink",
        relation: "archives",
        source: "telemetry",
    },
    DependencyEdge {
        from: "checkout-api",
        to: "telemetry-agent",
        relation: "emits",
        source: "telemetry",
    },
    DependencyEdge {
        from: "checkout-web",
        to: "telemetry-agent",
        relation: "emits",
        source: "telemetry",
    },
    DependencyEdge {
        from: "billing-api",
        to: "telemetry-agent",
        relation: "emits",
        source: "telemetry",
    },
    DependencyEdge {
        from: "checkout-db",
        to: "telemetry-agent",
        relation: "metrics",
        source: "telemetry",
    },
    DependencyEdge {
        from: "billing-db",
        to: "telemetry-agent",
        relation: "metrics",
        source: "telemetry",
    },
    DependencyEdge {
        from: "identity-db",
        to: "telemetry-agent",
        relation: "metrics",
        source: "telemetry",
    },
    DependencyEdge {
        from: "analytics-ingest",
        to: "warehouse",
        relation: "writes",
        source: "telemetry",
    },
    DependencyEdge {
        from: "checkout-queue",
        to: "analytics-ingest",
        relation: "blocked",
        source: "space rule",
    },
    DependencyEdge {
        from: "audit-sink",
        to: "analytics-ingest",
        relation: "exports",
        source: "private link",
    },
    DependencyEdge {
        from: "warehouse",
        to: "telemetry-agent",
        relation: "metrics",
        source: "telemetry",
    },
];

const PROJECT_NODES: [GraphNode; 8] = [
    GraphNode {
        id: "checkout",
        label: "checkout",
        kind: "project",
        status: "Healthy",
        detail: "commerce",
        metric: "9 components",
        project: "checkout",
        environment: "3 envs",
        space: "acme-pci-prod",
        project_slug: Some("checkout"),
    },
    GraphNode {
        id: "identity",
        label: "identity",
        kind: "project",
        status: "Healthy",
        detail: "auth",
        metric: "6 components",
        project: "identity",
        environment: "2 envs",
        space: "acme-pci-prod",
        project_slug: Some("identity"),
    },
    GraphNode {
        id: "billing",
        label: "billing",
        kind: "project",
        status: "Healthy",
        detail: "finance",
        metric: "8 components",
        project: "billing",
        environment: "2 envs",
        space: "acme-pci-prod",
        project_slug: Some("billing"),
    },
    GraphNode {
        id: "notifications",
        label: "notifications",
        kind: "project",
        status: "Healthy",
        detail: "messaging",
        metric: "6 components",
        project: "notifications",
        environment: "2 envs",
        space: "acme-pci-prod",
        project_slug: Some("notifications"),
    },
    GraphNode {
        id: "catalog",
        label: "catalog",
        kind: "project",
        status: "Warning",
        detail: "commerce",
        metric: "7 components",
        project: "catalog",
        environment: "2 envs",
        space: "hosted-us-east",
        project_slug: Some("catalog"),
    },
    GraphNode {
        id: "support",
        label: "support",
        kind: "project",
        status: "Ready",
        detail: "customer ops",
        metric: "5 components",
        project: "support",
        environment: "2 envs",
        space: "hosted-us-east",
        project_slug: Some("support"),
    },
    GraphNode {
        id: "analytics",
        label: "analytics",
        kind: "project",
        status: "Blocked",
        detail: "eu data",
        metric: "8 components",
        project: "analytics",
        environment: "1 env",
        space: "acme-eu-core",
        project_slug: Some("analytics"),
    },
    GraphNode {
        id: "observability",
        label: "observability",
        kind: "project",
        status: "Healthy",
        detail: "sre",
        metric: "15 components",
        project: "observability",
        environment: "1 env",
        space: "hosted-us-east",
        project_slug: Some("observability"),
    },
];

const PROJECT_EDGES: [DependencyEdge; 11] = [
    DependencyEdge {
        from: "checkout",
        to: "identity",
        relation: "auth",
        source: "linked",
    },
    DependencyEdge {
        from: "checkout",
        to: "billing",
        relation: "charges",
        source: "linked",
    },
    DependencyEdge {
        from: "billing",
        to: "notifications",
        relation: "events",
        source: "linked",
    },
    DependencyEdge {
        from: "notifications",
        to: "identity",
        relation: "templates",
        source: "linked",
    },
    DependencyEdge {
        from: "support",
        to: "identity",
        relation: "requested",
        source: "target pending",
    },
    DependencyEdge {
        from: "checkout",
        to: "catalog",
        relation: "requested",
        source: "needs allow",
    },
    DependencyEdge {
        from: "catalog",
        to: "observability",
        relation: "metrics",
        source: "drift",
    },
    DependencyEdge {
        from: "checkout",
        to: "observability",
        relation: "metrics",
        source: "telemetry",
    },
    DependencyEdge {
        from: "billing",
        to: "observability",
        relation: "metrics",
        source: "telemetry",
    },
    DependencyEdge {
        from: "analytics",
        to: "catalog",
        relation: "blocked",
        source: "space mismatch",
    },
    DependencyEdge {
        from: "observability",
        to: "analytics",
        relation: "exports",
        source: "space mismatch",
    },
];

#[component]
pub fn Graph() -> Element {
    let selected_id = use_signal(|| None::<String>);
    let nodes: &[GraphNode] = &PROJECT_NODES;
    let edges: &[DependencyEdge] = &PROJECT_EDGES;

    rsx! {
        div { class: "mb-6 flex flex-col gap-3 sm:flex-row sm:items-start sm:justify-between",
            PageTitle { title: "Topology", subtitle: "Project private-link graph." }
            div { class: "flex gap-2",
                button { class: "btn btn-outline", Search { size: 16 } "Find" }
                button { class: "btn btn-primary", Link2 { size: 16 } "Request link" }
            }
        }
        GraphPanel {
            nodes: nodes.to_vec(),
            edges: edges.to_vec(),
            title: "Project private-link graph",
            scope: "Acme Retail",
            selected_id,
            project_view: true,
            show_project_link: true
        }
    }
}

#[component]
pub fn ProjectComponentGraph(project_slug: String, project_name: String) -> Element {
    let nodes: Vec<GraphNode> = COMPONENT_NODES
        .iter()
        .copied()
        .filter(|node| node.project_slug == Some(project_slug.as_str()))
        .collect();

    if nodes.is_empty() {
        return rsx! {
            div { class: "card mb-5 border border-base-300 bg-base-100",
                div { class: "card-body items-center py-10 text-center",
                    h2 { class: "card-title", "No components found" }
                    p { class: "text-base-content/60", "No component telemetry has been collected for this project." }
                }
            }
        };
    }

    let edges: Vec<DependencyEdge> = COMPONENT_EDGES
        .iter()
        .copied()
        .filter(|edge| {
            nodes.iter().any(|node| node.id == edge.from)
                && nodes.iter().any(|node| node.id == edge.to)
        })
        .collect();
    let selected_id = use_signal(|| None::<String>);

    rsx! {
        GraphPanel {
            nodes,
            edges,
            title: "Component topology",
            scope: format!("{project_name} prod"),
            selected_id,
            project_view: false,
            show_project_link: false
        }
    }
}

#[component]
fn GraphPanel(
    nodes: Vec<GraphNode>,
    edges: Vec<DependencyEdge>,
    title: String,
    scope: String,
    mut selected_id: Signal<Option<String>>,
    project_view: bool,
    show_project_link: bool,
) -> Element {
    let active_id = selected_id();
    let focused_id = active_id.as_deref();
    let selected = focused_id.and_then(|id| graph_node_by_id(&nodes, id));
    let layout = layout_graph(&nodes, &edges);
    let resource_count = nodes.len();
    let dependency_count = edges.len();

    rsx! {
        section { class: "mb-5 grid gap-5 xl:grid-cols-[minmax(0,1fr)_18rem]",
            div { class: "card min-w-0 border border-base-300 bg-base-100",
                div { class: "card-body",
                    div { class: "mb-2 flex flex-wrap items-start justify-between gap-3",
                        div {
                            h2 { class: "card-title capitalize", "{title}" }
                            p { class: "text-sm text-base-content/60", "{resource_count} resources, {dependency_count} dependencies" }
                        }
                        span { class: "badge badge-outline", "{scope}" }
                    }
                    div {
                        class: "relative min-h-[36rem] overflow-hidden rounded-box border border-base-300 bg-base-200",
                        onclick: move |_| selected_id.set(None),
                        svg {
                            class: "absolute inset-0 h-full w-full text-base-content",
                            view_box: "0 0 100 100",
                            preserve_aspect_ratio: "none",
                            for edge in layout.edges {
                                polyline {
                                    class: if focused_id.is_some_and(|id| edge.from == id || edge.to == id) {
                                        "fill-none stroke-current opacity-70"
                                    } else {
                                        "fill-none stroke-current opacity-15"
                                    },
                                    stroke_width: "0.35",
                                    vector_effect: "non-scaling-stroke",
                                    "data-relation": edge.relation,
                                    "data-source": edge.source,
                                    points: "{edge.from_x},{edge.from_y} {edge.from_x},{edge.mid_y} {edge.to_x},{edge.mid_y} {edge.to_x},{edge.to_y}"
                                }
                            }
                        }
                        for positioned in layout.nodes {
                            GraphNodeView {
                                positioned,
                                selected: focused_id == Some(positioned.node.id),
                                related: focused_id
                                    .map(|id| is_related_node(id, positioned.node.id, &edges))
                                    .unwrap_or(false),
                                focused: focused_id.is_some(),
                                selected_id
                            }
                        }
                    }
                }
            }

            if let Some(node) = selected {
                GraphInspector {
                    node,
                    incoming: edges.iter().filter(|edge| edge.to == node.id).count(),
                    outgoing: edges.iter().filter(|edge| edge.from == node.id).count(),
                    project_view,
                    show_project_link
                }
            } else {
                GraphOverviewInspector { title, scope, resource_count, dependency_count }
            }
        }
    }
}

#[component]
fn GraphNodeView(
    positioned: PositionedNode,
    selected: bool,
    related: bool,
    focused: bool,
    mut selected_id: Signal<Option<String>>,
) -> Element {
    let node = positioned.node;
    let style = format!("left: {}%; top: {}%;", positioned.x, positioned.y);
    let class = if selected {
        "btn absolute h-auto min-h-20 w-44 -translate-x-1/2 -translate-y-1/2 justify-start border-primary bg-base-100 p-3 text-left normal-case outline outline-2 outline-primary"
    } else if related {
        "btn absolute h-auto min-h-20 w-44 -translate-x-1/2 -translate-y-1/2 justify-start border-primary bg-base-100 p-3 text-left normal-case"
    } else if focused {
        "btn absolute h-auto min-h-20 w-44 -translate-x-1/2 -translate-y-1/2 justify-start border-base-300 bg-base-100 p-3 text-left normal-case opacity-35"
    } else {
        "btn absolute h-auto min-h-20 w-44 -translate-x-1/2 -translate-y-1/2 justify-start border-base-300 bg-base-100 p-3 text-left normal-case"
    };

    rsx! {
        button {
            class,
            style,
            onclick: move |event| {
                event.stop_propagation();
                if selected_id().as_deref() == Some(node.id) {
                    selected_id.set(None);
                } else {
                    selected_id.set(Some(node.id.to_string()));
                }
            },
            div { class: "flex min-w-0 items-center gap-3",
                span { class: "shrink-0", NodeIcon { kind: node.kind } }
                span { class: "min-w-0",
                    span { class: "flex items-center gap-2",
                        strong { class: "truncate", "{node.label}" }
                        StatusDot { status: node.status }
                    }
                    span { class: "block truncate text-xs font-normal text-base-content/60", "{node.detail}" }
                    span { class: "block truncate text-xs font-normal text-base-content/60", "{node.metric}" }
                }
            }
        }
    }
}

#[component]
fn NodeIcon(kind: String) -> Element {
    rsx! {
        if kind == "network" { Globe { size: 18 } }
        else if kind == "database" { Database { size: 18 } }
        else if kind == "worker" { Activity { size: 18 } }
        else if kind == "secret" { Shield { size: 18 } }
        else if kind == "queue" { GitBranch { size: 18 } }
        else if kind == "project" { Boxes { size: 18 } }
        else { Server { size: 18 } }
    }
}

#[component]
fn StatusDot(status: String) -> Element {
    let class = match status.as_str() {
        "Warning" | "Blocked" | "Needs allow" | "Drift" => "status status-error",
        "Deploying" | "Pending" | "Syncing" => "status status-warning",
        _ => "status status-success",
    };
    rsx! { span { class } }
}

#[component]
fn GraphInspector(
    node: GraphNode,
    incoming: usize,
    outgoing: usize,
    project_view: bool,
    show_project_link: bool,
) -> Element {
    rsx! {
        div { class: "card h-fit border border-base-300 bg-base-100",
            div { class: "card-body",
                h2 { class: "card-title", "{node.label}" }
                StatusBadge { status: node.status }
                strong { class: "text-3xl", "{node.metric}" }
                span { class: "text-sm text-base-content/60", "{node.kind}" }
                div { class: "divide-y divide-base-300",
                    GraphDetail { label: "Kind", value: node.kind }
                    GraphDetail { label: "Project", value: node.project }
                    GraphDetail { label: "Environment", value: node.environment }
                    GraphDetail { label: "Space", value: node.space }
                    GraphDetail { label: "Incoming", value: incoming.to_string() }
                    GraphDetail { label: "Outgoing", value: outgoing.to_string() }
                }
                div { class: "grid grid-cols-2 gap-2",
                    if show_project_link {
                        if let Some(slug) = node.project_slug {
                            Link { to: Route::ProjectDetail { slug: slug.to_string() }, class: "btn btn-primary col-span-2",
                                if project_view { "Open project" } else { "Open owning project" }
                            }
                        }
                    }
                    button { class: "btn btn-outline", Network { size: 16 } "Flows" }
                    button { class: "btn btn-primary", Terminal { size: 16 } "Config" }
                }
            }
        }
    }
}

#[component]
fn GraphOverviewInspector(
    title: String,
    scope: String,
    resource_count: usize,
    dependency_count: usize,
) -> Element {
    rsx! {
        div { class: "card h-fit border border-base-300 bg-base-100",
            div { class: "card-body",
                h2 { class: "card-title", "{title}" }
                StatusBadge { status: "Ready" }
                strong { class: "text-3xl", "{resource_count}" }
                span { class: "text-sm text-base-content/60", "resources" }
                div { class: "divide-y divide-base-300",
                    GraphDetail { label: "Scope", value: scope }
                    GraphDetail { label: "Dependencies", value: dependency_count.to_string() }
                    GraphDetail { label: "Selection", value: "None" }
                }
            }
        }
    }
}

#[component]
fn GraphDetail(label: String, value: String) -> Element {
    rsx! {
        div { class: "grid gap-1 py-3",
            span { class: "text-xs text-base-content/60", "{label}" }
            strong { class: "text-sm", "{value}" }
        }
    }
}

fn graph_node_by_id(nodes: &[GraphNode], id: &str) -> Option<GraphNode> {
    nodes.iter().copied().find(|node| node.id == id)
}

fn is_related_node(selected: &str, node: &str, edges: &[DependencyEdge]) -> bool {
    edges.iter().any(|edge| {
        edge.from == selected && edge.to == node || edge.to == selected && edge.from == node
    })
}

fn layout_graph(nodes: &[GraphNode], dependencies: &[DependencyEdge]) -> GraphLayout {
    const MAX_LAYER_COLUMNS: usize = 4;

    let mut index_by_id = HashMap::new();
    for (index, node) in nodes.iter().enumerate() {
        index_by_id.insert(node.id, index);
    }

    let node_count = nodes.len();
    let mut outgoing = vec![Vec::new(); node_count];
    let mut incoming = vec![Vec::new(); node_count];
    let mut remaining_incoming = vec![0usize; node_count];

    for edge in dependencies {
        let Some(&from) = index_by_id.get(edge.from) else {
            continue;
        };
        let Some(&to) = index_by_id.get(edge.to) else {
            continue;
        };

        outgoing[from].push(to);
        incoming[to].push(from);
        remaining_incoming[to] += 1;
    }

    let mut layers = vec![None; node_count];
    let mut queue = VecDeque::new();
    for index in 0..node_count {
        if remaining_incoming[index] == 0 {
            layers[index] = Some(0);
            queue.push_back(index);
        }
    }

    if queue.is_empty() && node_count > 0 {
        layers[0] = Some(0);
        queue.push_back(0);
    }

    while let Some(index) = queue.pop_front() {
        let next_layer = layers[index].unwrap_or(0) + 1;
        for &target in &outgoing[index] {
            layers[target] =
                Some(layers[target].map_or(next_layer, |layer: usize| layer.max(next_layer)));
            remaining_incoming[target] = remaining_incoming[target].saturating_sub(1);
            if remaining_incoming[target] == 0 {
                queue.push_back(target);
            }
        }
    }

    while layers.iter().any(Option::is_none) {
        let mut changed = false;
        for index in 0..node_count {
            if layers[index].is_some() {
                continue;
            }

            let incoming_layer = incoming[index]
                .iter()
                .filter_map(|source| layers[*source])
                .max();

            if let Some(layer) = incoming_layer {
                layers[index] = Some(layer + 1);
                changed = true;
            }
        }

        if !changed {
            let next_layer = layers.iter().filter_map(|layer| *layer).max().unwrap_or(0) + 1;
            if let Some(index) = layers.iter().position(Option::is_none) {
                layers[index] = Some(next_layer);
            }
        }
    }

    let mut raw_layers: Vec<usize> = layers.into_iter().map(|layer| layer.unwrap_or(0)).collect();
    let mut unique_layers = raw_layers.clone();
    unique_layers.sort_unstable();
    unique_layers.dedup();

    for layer in &mut raw_layers {
        *layer = unique_layers
            .iter()
            .position(|candidate| candidate == layer)
            .unwrap_or(0);
    }

    let mut layer_counts = vec![0usize; unique_layers.len().max(1)];
    for layer in &raw_layers {
        layer_counts[*layer] += 1;
    }

    let mut layer_start_rows = vec![0usize; layer_counts.len()];
    let mut total_rows = 0usize;
    for (layer, count) in layer_counts.iter().enumerate() {
        layer_start_rows[layer] = total_rows;
        total_rows += (*count).max(1).div_ceil(MAX_LAYER_COLUMNS);
    }

    let mut layer_offsets = vec![0usize; layer_counts.len()];
    let mut positions = vec![(50.0, 50.0); node_count];
    let mut positioned_nodes = Vec::with_capacity(node_count);

    for (index, node) in nodes.iter().copied().enumerate() {
        let layer = raw_layers[index];
        let order = layer_offsets[layer];
        layer_offsets[layer] += 1;

        let row_in_layer = order / MAX_LAYER_COLUMNS;
        let column = order % MAX_LAYER_COLUMNS;
        let remaining = layer_counts[layer].saturating_sub(row_in_layer * MAX_LAYER_COLUMNS);
        let columns_in_row = remaining.clamp(1, MAX_LAYER_COLUMNS);
        let visual_row = layer_start_rows[layer] + row_in_layer;

        let x = match columns_in_row {
            0 | 1 => 50.0,
            count => 18.0 + column as f64 * (64.0 / (count - 1) as f64),
        };
        let y = if total_rows <= 1 {
            50.0
        } else {
            10.0 + visual_row as f64 * (80.0 / (total_rows - 1) as f64)
        };

        positions[index] = (x, y);
        positioned_nodes.push(PositionedNode { node, x, y });
    }

    positioned_nodes.sort_by(|left, right| {
        left.y
            .total_cmp(&right.y)
            .then(left.x.total_cmp(&right.x))
            .then(left.node.label.cmp(right.node.label))
    });

    let positioned_edges = dependencies
        .iter()
        .filter_map(|edge| {
            let from = *index_by_id.get(edge.from)?;
            let to = *index_by_id.get(edge.to)?;
            let (from_x, from_y) = positions[from];
            let (to_x, to_y) = positions[to];
            let mid_y = if to_y >= from_y {
                (from_y + to_y) / 2.0
            } else {
                (from_y + 6.0).min(92.0)
            };

            Some(PositionedEdge {
                from: edge.from,
                to: edge.to,
                from_x,
                from_y,
                mid_y,
                to_x,
                to_y,
                relation: edge.relation,
                source: edge.source,
            })
        })
        .collect();

    GraphLayout {
        nodes: positioned_nodes,
        edges: positioned_edges,
    }
}
