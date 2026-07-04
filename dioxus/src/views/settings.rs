use crate::components::button::{Button, ButtonVariant};
use crate::components::card::{Card, CardContent, CardHeader, CardTitle};
use crate::components::input::Input;
use crate::components::label::Label;
use crate::components::switch::Switch;
use crate::views::common::PageTitle;
use dioxus::prelude::*;
use dioxus_icons::lucide::{Network, Shield, Users};

#[component]
pub fn Settings() -> Element {
    let mut require_target_allow = use_signal(|| true);
    let mut dedicated_space_opt_in = use_signal(|| true);
    let mut telemetry_edges = use_signal(|| true);

    rsx! {
        PageTitle {
            title: "Settings",
            subtitle: "Tenant controls."
        }

        section { class: "settings-grid",
            Card {
                CardHeader {
                    CardTitle {
                        Users { size: 18 }
                        "Tenant"
                    }
                }
                CardContent {
                    div { class: "form-grid",
                        div { class: "field",
                            Label { html_for: "tenant-name", "Tenant name" }
                            Input {
                                id: "tenant-name",
                                value: "Acme Retail",
                                aria_label: "Tenant name"
                            }
                        }
                        div { class: "field",
                            Label { html_for: "tenant-slug", "Tenant slug" }
                            Input {
                                id: "tenant-slug",
                                value: "acme-retail",
                                aria_label: "Tenant slug"
                            }
                        }
                        div { class: "field",
                            Label { html_for: "default-space", "Default space" }
                            Input {
                                id: "default-space",
                                value: "hosted-us-east",
                                aria_label: "Default space"
                            }
                        }
                        div { class: "field",
                            Label { html_for: "default-project", "Default project" }
                            Input {
                                id: "default-project",
                                value: "checkout",
                                aria_label: "Default project"
                            }
                        }
                    }
                    div { class: "form-actions",
                        Button { variant: ButtonVariant::Outline, "Cancel" }
                        Button { "Save changes" }
                    }
                }
            }

            Card {
                CardHeader {
                    CardTitle {
                        Shield { size: 18 }
                        "Network guardrails"
                    }
                }
                CardContent {
                    div { class: "switch-list",
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

            Card {
                CardHeader {
                    CardTitle {
                        Network { size: 18 }
                        "Graph sources"
                    }
                }
                CardContent {
                    div { class: "detail-list",
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
fn SwitchRow(
    title: String,
    detail: String,
    checked: bool,
    onchange: EventHandler<bool>,
) -> Element {
    rsx! {
        div { class: "switch-row",
            div {
                strong { "{title}" }
                span { "{detail}" }
            }
            Switch {
                checked,
                aria_label: title,
                on_checked_change: move |value| onchange.call(value)
            }
        }
    }
}

#[component]
fn GraphSource(label: String, value: String) -> Element {
    rsx! {
        div { class: "detail-item",
            span { "{label}" }
            strong { "{value}" }
        }
    }
}
