use crate::components::badge::{Badge, BadgeVariant};
use crate::components::card::{Card, CardContent};
use crate::components::progress::Progress;
use dioxus::prelude::*;

#[component]
pub fn PageTitle(title: String, subtitle: String) -> Element {
    rsx! {
        div { class: "page-title",
            div {
                h1 { "{title}" }
            }
            if !subtitle.is_empty() {
                p { "{subtitle}" }
            }
        }
    }
}

#[component]
pub fn StatusBadge(status: String) -> Element {
    let variant = match status.as_str() {
        "Healthy" | "Ready" | "Paid" | "Active" | "Passed" | "Allowed" | "Linked"
        | "Available" | "Dedicated" => BadgeVariant::Primary,
        "Deploying" | "Pending" | "Preview" | "Requested" | "Shared" | "Syncing" => {
            BadgeVariant::Secondary
        }
        "Warning" | "Failed" | "Overdue" | "Blocked" | "Needs allow" | "Drift" => {
            BadgeVariant::Destructive
        }
        _ => BadgeVariant::Outline,
    };

    rsx! {
        Badge { variant, "{status}" }
    }
}

#[component]
pub fn UsageMeter(label: String, value: f64, detail: String) -> Element {
    rsx! {
        div { class: "usage-meter",
            div { class: "usage-meter-header",
                span { "{label}" }
                strong { "{value:.0}%" }
            }
            Progress { value }
            p { "{detail}" }
        }
    }
}

#[component]
pub fn EmptyState(title: String, detail: String) -> Element {
    rsx! {
        Card { class: "empty-state",
            CardContent {
                h3 { "{title}" }
                p { "{detail}" }
            }
        }
    }
}
