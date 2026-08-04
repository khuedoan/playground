use crate::views::common::PageTitle;
use dioxus::prelude::*;
use dioxus_icons::lucide::{Network, Shield, Users};

#[component]
pub fn Settings() -> Element {
    let mut require_target_allow = use_signal(|| true);
    let mut dedicated_space_opt_in = use_signal(|| true);
    let mut telemetry_edges = use_signal(|| true);

    rsx! {
        PageTitle { title: "Settings", subtitle: "Tenant controls." }

        section { class: "grid gap-5 lg:grid-cols-2",
            div { class: "card border border-base-300 bg-base-100",
                div { class: "card-body",
                    h2 { class: "card-title", Users { size: 18 } "Tenant" }
                    div { class: "grid gap-4 sm:grid-cols-2",
                        TextField { id: "tenant-name", label: "Tenant name", value: "Acme Retail" }
                        TextField { id: "tenant-slug", label: "Tenant slug", value: "acme-retail" }
                        TextField { id: "default-space", label: "Default space", value: "hosted-us-east" }
                        TextField { id: "default-project", label: "Default project", value: "checkout" }
                    }
                    div { class: "card-actions justify-end pt-3",
                        button { class: "btn btn-ghost", "Cancel" }
                        button { class: "btn btn-primary", "Save changes" }
                    }
                }
            }

            div { class: "card border border-base-300 bg-base-100",
                div { class: "card-body",
                    h2 { class: "card-title", Shield { size: 18 } "Network guardrails" }
                    div { class: "divide-y divide-base-300",
                        SwitchRow {
                            title: "Require target allow",
                            detail: "Private links need both source request and target approval.",
                            checked: require_target_allow(),
                            onchange: move |checked| require_target_allow.set(checked)
                        }
                        SwitchRow {
                            title: "Dedicated space opt-in",
                            detail: "Enterprise spaces must be explicitly granted to this tenant.",
                            checked: dedicated_space_opt_in(),
                            onchange: move |checked| dedicated_space_opt_in.set(checked)
                        }
                        SwitchRow {
                            title: "Infer dependency graph",
                            detail: "Use network telemetry and config references to create component edges.",
                            checked: telemetry_edges(),
                            onchange: move |checked| telemetry_edges.set(checked)
                        }
                    }
                }
            }

            div { class: "card border border-base-300 bg-base-100 lg:col-span-2",
                div { class: "card-body",
                    h2 { class: "card-title", Network { size: 18 } "Graph sources" }
                    div { class: "divide-y divide-base-300",
                        GraphSource { label: "Network telemetry", value: "mTLS flow records and DNS targets" }
                        GraphSource { label: "Vault references", value: "secret/data/project/env/component paths" }
                        GraphSource { label: "Private links", value: "project intent plus target allow state" }
                        GraphSource { label: "Space placement", value: "environment to compute space assignment" }
                    }
                }
            }
        }
    }
}

#[component]
fn TextField(id: String, label: String, value: String) -> Element {
    rsx! {
        label { class: "form-control gap-2",
            span { class: "label-text", "{label}" }
            input { id, class: "input input-bordered w-full", value, aria_label: label }
        }
    }
}

#[component]
fn SwitchRow(
    title: String,
    detail: String,
    checked: bool,
    onchange: EventHandler<bool>,
) -> Element {
    rsx! {
        label { class: "flex cursor-pointer items-center justify-between gap-4 py-4",
            span {
                strong { class: "block", "{title}" }
                span { class: "text-sm text-base-content/60", "{detail}" }
            }
            input {
                r#type: "checkbox",
                class: "toggle toggle-primary",
                checked,
                aria_label: title,
                onchange: move |event| onchange.call(event.checked())
            }
        }
    }
}

#[component]
fn GraphSource(label: String, value: String) -> Element {
    rsx! {
        div { class: "grid gap-2 py-3 sm:grid-cols-[12rem_1fr]",
            span { class: "text-sm text-base-content/60", "{label}" }
            strong { class: "text-sm", "{value}" }
        }
    }
}
