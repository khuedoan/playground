use crate::components::avatar::{Avatar, AvatarFallback, AvatarImageSize};
use crate::components::badge::{Badge, BadgeVariant};
use crate::components::button::{Button, ButtonVariant};
use crate::components::input::Input;
use crate::components::sidebar::{
    Sidebar, SidebarCollapsible, SidebarContent, SidebarFooter, SidebarGroup, SidebarGroupContent,
    SidebarGroupLabel, SidebarHeader, SidebarInset, SidebarMenu, SidebarMenuItem, SidebarProvider,
};
use crate::Route;
use dioxus::prelude::*;
use dioxus_icons::lucide::{
    Bell, Boxes, GitBranch, LayoutDashboard, Link2, Plus, Search, ServerCog, Settings, Shield,
};

#[component]
pub fn AppShell() -> Element {
    let current = use_route::<Route>();

    rsx! {
        SidebarProvider { class: "app-shell",
            Sidebar {
                collapsible: SidebarCollapsible::None,
                SidebarHeader {
                    div { class: "brand-row",
                        div { class: "brand-mark", "N" }
                        div { class: "brand-copy",
                            strong { "Netamos" }
                            span { "Networked PaaS" }
                        }
                    }
                }
                SidebarContent {
                    SidebarGroup {
                        SidebarGroupLabel { "Tenant" }
                        SidebarGroupContent {
                            SidebarMenu {
                                SidebarMenuItem {
                                    Link {
                                        to: Route::Dashboard {},
                                        class: nav_class(&current, &Route::Dashboard {}),
                                        LayoutDashboard { size: 16 }
                                        span { "Overview" }
                                    }
                                }
                                SidebarMenuItem {
                                    Link {
                                        to: Route::Graph {},
                                        class: nav_class(&current, &Route::Graph {}),
                                        GitBranch { size: 16 }
                                        span { "Topology" }
                                    }
                                }
                                SidebarMenuItem {
                                    Link {
                                        to: Route::Projects {},
                                        class: nav_class(&current, &Route::Projects {}),
                                        Boxes { size: 16 }
                                        span { "Projects" }
                                    }
                                }
                                SidebarMenuItem {
                                    Link {
                                        to: Route::PrivateLinks {},
                                        class: nav_class(&current, &Route::PrivateLinks {}),
                                        Link2 { size: 16 }
                                        span { "Private links" }
                                    }
                                }
                                SidebarMenuItem {
                                    Link {
                                        to: Route::Spaces {},
                                        class: nav_class(&current, &Route::Spaces {}),
                                        ServerCog { size: 16 }
                                        span { "Spaces" }
                                    }
                                }
                                SidebarMenuItem {
                                    Link {
                                        to: Route::Settings {},
                                        class: nav_class(&current, &Route::Settings {}),
                                        Settings { size: 16 }
                                        span { "Settings" }
                                    }
                                }
                            }
                        }
                    }
                    SidebarGroup {
                        SidebarGroupLabel { "Current scope" }
                        SidebarGroupContent {
                            div { class: "sidebar-environment",
                                div {
                                    Shield { size: 16 }
                                    span { "Acme Retail" }
                                }
                                Badge { variant: BadgeVariant::Secondary, "3 spaces" }
                            }
                        }
                    }
                }
                SidebarFooter {
                    div { class: "account-card",
                        Avatar { size: AvatarImageSize::Medium,
                            AvatarFallback { "KD" }
                        }
                        div {
                            strong { "Khue Doan" }
                            span { "Platform owner" }
                        }
                    }
                }
            }

            SidebarInset { class: "app-main",
                header { class: "topbar",
                    div { class: "topbar-left",
                        div { class: "search-control",
                            Search { size: 16 }
                            Input {
                                r#type: "search",
                                placeholder: "Search projects, components, links",
                                aria_label: "Search"
                            }
                        }
                    }
                    div { class: "topbar-actions",
                        Button { variant: ButtonVariant::Ghost, size: crate::components::button::ButtonSize::Icon,
                            Bell { size: 16 }
                            span { class: "sr-only", "Notifications" }
                        }
                        Button {
                            Plus { size: 16 }
                            "New project"
                        }
                    }
                }
                div { class: "page-frame",
                    Outlet::<Route> {}
                }
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
        "app-nav-link active"
    } else {
        "app-nav-link"
    }
}
