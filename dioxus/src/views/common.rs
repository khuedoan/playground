use dioxus::prelude::*;

#[component]
pub fn PageTitle(title: String, subtitle: String) -> Element {
    rsx! {
        div { class: "mb-6 flex flex-col gap-2 sm:flex-row sm:items-end sm:justify-between",
            h1 { class: "text-3xl font-bold", "{title}" }
            if !subtitle.is_empty() {
                p { class: "text-base-content/60", "{subtitle}" }
            }
        }
    }
}

#[component]
pub fn StatusBadge(status: String) -> Element {
    let class = match status.as_str() {
        "Healthy" | "Ready" | "Paid" | "Active" | "Passed" | "Allowed" | "Linked" | "Available"
        | "Dedicated" => "badge badge-success badge-sm",
        "Deploying" | "Pending" | "Preview" | "Requested" | "Shared" | "Syncing" => {
            "badge badge-warning badge-sm"
        }
        "Warning" | "Failed" | "Overdue" | "Blocked" | "Needs allow" | "Drift" => {
            "badge badge-error badge-sm"
        }
        _ => "badge badge-ghost badge-sm",
    };

    rsx! { span { class, "{status}" } }
}

#[component]
pub fn UsageMeter(label: String, value: f64, detail: String) -> Element {
    rsx! {
        div { class: "grid min-w-32 gap-1 text-sm",
            div { class: "flex justify-between gap-3",
                span { class: "text-base-content/60", "{label}" }
                strong { "{value:.0}%" }
            }
            progress { class: "progress progress-primary w-full", value, max: 100 }
            p { class: "text-xs text-base-content/60", "{detail}" }
        }
    }
}

#[component]
pub fn EmptyState(title: String, detail: String) -> Element {
    rsx! {
        div { class: "card border border-base-300 bg-base-100",
            div { class: "card-body items-center py-10 text-center",
                h3 { class: "card-title", "{title}" }
                p { class: "text-base-content/60", "{detail}" }
            }
        }
    }
}
