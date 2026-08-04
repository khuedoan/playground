use crate::Route;
use dioxus::prelude::*;
use dioxus_icons::lucide::{
    Bell, Boxes, GitBranch, LayoutDashboard, Link2, Menu, Plus, Search, ServerCog, Settings, Shield,
};

#[component]
pub fn AppShell() -> Element {
    let current = use_route::<Route>();

    rsx! {
        div { class: "drawer min-h-screen lg:drawer-open",
            input { id: "app-drawer", r#type: "checkbox", class: "drawer-toggle" }

            div { class: "drawer-content min-w-0 bg-base-200",
                header { class: "navbar sticky top-0 z-10 gap-2 border-b border-base-300 bg-base-100 px-4",
                    div { class: "navbar-start gap-2",
                        label {
                            r#for: "app-drawer",
                            class: "btn btn-square btn-ghost lg:hidden",
                            aria_label: "Open navigation",
                            Menu { size: 18 }
                        }
                        label { class: "input input-bordered hidden w-full max-w-md items-center gap-2 sm:flex",
                            Search { size: 16 }
                            input {
                                r#type: "search",
                                class: "grow",
                                placeholder: "Search projects, components, links",
                                aria_label: "Search"
                            }
                        }
                    }
                    div { class: "navbar-end gap-2",
                        button { class: "btn btn-square btn-ghost", aria_label: "Notifications",
                            Bell { size: 18 }
                        }
                        button { class: "btn btn-primary",
                            Plus { size: 18 }
                            span { class: "hidden sm:inline", "New project" }
                        }
                    }
                }
                main { class: "mx-auto w-full max-w-screen-2xl p-4 sm:p-7",
                    Outlet::<Route> {}
                }
            }

            div { class: "drawer-side z-20",
                label { r#for: "app-drawer", aria_label: "Close navigation", class: "drawer-overlay" }
                aside { class: "flex min-h-full w-72 flex-col border-r border-base-300 bg-base-100",
                    div { class: "flex items-center gap-3 p-5",
                        div { class: "avatar placeholder",
                            div { class: "w-10 rounded-lg bg-primary text-primary-content",
                                span { class: "font-bold", "N" }
                            }
                        }
                        div {
                            strong { class: "block", "Netamos" }
                            span { class: "text-sm text-base-content/60", "Networked PaaS" }
                        }
                    }

                    nav { class: "flex-1 px-3",
                        ul { class: "menu w-full gap-1",
                            li { class: "menu-title", "Tenant" }
                            NavItem { current: current.clone(), route: Route::Dashboard {}, label: "Overview", icon: "dashboard" }
                            NavItem { current: current.clone(), route: Route::Graph {}, label: "Topology", icon: "graph" }
                            NavItem { current: current.clone(), route: Route::Projects {}, label: "Projects", icon: "projects" }
                            NavItem { current: current.clone(), route: Route::PrivateLinks {}, label: "Private links", icon: "links" }
                            NavItem { current: current.clone(), route: Route::Spaces {}, label: "Spaces", icon: "spaces" }
                            NavItem { current: current.clone(), route: Route::Settings {}, label: "Settings", icon: "settings" }
                        }

                        div { class: "mt-6",
                            p { class: "px-3 pb-2 text-xs font-semibold text-base-content/50", "CURRENT SCOPE" }
                            div { class: "flex items-center justify-between rounded-box bg-base-200 p-3",
                                span { class: "flex items-center gap-2",
                                    Shield { size: 16 }
                                    "Acme Retail"
                                }
                                span { class: "badge badge-ghost badge-sm", "3 spaces" }
                            }
                        }
                    }

                    div { class: "m-3 flex items-center gap-3 rounded-box border border-base-300 p-3",
                        div { class: "avatar placeholder",
                            div { class: "w-9 rounded-full bg-neutral text-neutral-content",
                                span { class: "text-xs", "KD" }
                            }
                        }
                        div {
                            strong { class: "block text-sm", "Khue Doan" }
                            span { class: "text-xs text-base-content/60", "Platform owner" }
                        }
                    }
                }
            }
        }
    }
}

#[component]
fn NavItem(current: Route, route: Route, label: String, icon: String) -> Element {
    rsx! {
        li {
            Link { to: route.clone(), class: nav_class(&current, &route),
                if icon == "dashboard" {
                    LayoutDashboard { size: 17 }
                } else if icon == "graph" {
                    GitBranch { size: 17 }
                } else if icon == "projects" {
                    Boxes { size: 17 }
                } else if icon == "links" {
                    Link2 { size: 17 }
                } else if icon == "spaces" {
                    ServerCog { size: 17 }
                } else {
                    Settings { size: 17 }
                }
                "{label}"
            }
        }
    }
}

fn nav_class(current: &Route, route: &Route) -> &'static str {
    let is_active = current == route
        || matches!(
            (current, route),
            (Route::ProjectDetail { .. }, Route::Projects {})
        );

    if is_active {
        "menu-active"
    } else {
        ""
    }
}
