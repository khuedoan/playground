use crate::{
    components::dropdown_menu::{
        dropdown_menu, dropdown_menu_content, dropdown_menu_item, dropdown_menu_label,
        dropdown_menu_link, dropdown_menu_separator, dropdown_menu_trigger,
    },
    mock,
};
use topcoat::{
    Result,
    asset::asset,
    context::Cx,
    font::fontsource::fontsource_font,
    icon::{icon, iconify::iconify_icon},
    router::{RouterErrorExt, Slot, layout, path_param, query_params, redirect_permanent, uri},
    tailwind,
    view::{attributes, view},
};

#[path_param]
struct Tenant(str);

#[query_params(error = redirect("?"))]
struct LayoutQuery {
    action: Option<String>,
    display_name: Option<String>,
}

#[layout("/")]
pub async fn root_layout(slot: Slot<'_>) -> Result {
    view! {
        <!DOCTYPE html>
        <html lang="en">
            <head>
                <meta charset="utf-8">
                <meta name="viewport" content="width=device-width, initial-scale=1">
                <title>"Netamos"</title>
                topcoat::font::link(
                    font: fontsource_font!(
                        GEIST,
                        weight: [400, 500, 600],
                        style: Normal,
                        host: Asset,
                    )
                )
                <link rel="stylesheet" href=(tailwind::stylesheet!())>
                topcoat::dev::script()
                <script defer="" src=(asset!("assets/interactions.js"))></script>
            </head>
            <body>(slot.await?)</body>
        </html>
    }
}

#[layout("/tenants/{tenant}")]
pub async fn app_layout(cx: &Cx, slot: Slot<'_>) -> Result {
    let current_path = uri(cx).path();
    let query = query_params::<LayoutQuery>(cx)?;
    let tenant_slug = path_param::<Tenant>(cx);
    if tenant_slug == "default" {
        return Err(redirect_permanent("/tenants/khuedoan").into());
    }
    let tenant = mock::tenant(tenant_slug).ok_or_not_found()?;
    let tenant_root = format!("/tenants/{}", tenant.slug);
    let changes_path = format!("{tenant_root}/changes");
    let usage_path = format!("{tenant_root}/usage");
    let settings_path = format!("{tenant_root}/settings");
    let tenant_settings_active = current_path == settings_path;
    let changes_active = current_path == changes_path;
    let usage_active = current_path == usage_path;
    let projects_active = !tenant_settings_active && !changes_active && !usage_active;
    let projects_current = current_path == tenant_root;
    let stored_tenant_name = tenant.name;
    let tenant_name = if query.action.as_deref() == Some("save-tenant") {
        query.display_name.as_deref().unwrap_or(stored_tenant_name)
    } else {
        stored_tenant_name
    };
    let tenant_initial = tenant_name.chars().next().unwrap_or('T');
    let project_count = tenant.projects.len();

    view! {
        <div class="min-h-screen bg-subtle lg:grid lg:grid-cols-[13.5rem_minmax(0,1fr)]">
            <aside class="hidden border-r border-sidebar-border bg-sidebar text-sidebar-foreground lg:flex lg:h-screen lg:flex-col lg:sticky lg:top-0">
                <div class="flex h-14 shrink-0 items-center border-b border-sidebar-border px-4">
                    <a
                        href=(tenant_root.clone())
                        aria-label="Netamos home"
                        class="flex items-center gap-2.5 font-semibold tracking-tight"
                    >
                        <span class="grid size-6 place-items-center rounded bg-sidebar-foreground text-sidebar">
                            icon(
                                data: iconify_icon!("feather:hexagon"),
                                attrs: attributes! {
                                    aria-hidden="true"
                                    class="size-3.5"
                                }
                            )
                        </span>
                        "Netamos"
                    </a>
                </div>

                <div class="border-b border-sidebar-border p-3">
                    <div class="flex min-w-0 items-stretch gap-0.5">
                        <a
                            href=(tenant_root.clone())
                            aria-label=(format!("Open {tenant_name} tenant home"))
                            class="flex h-10 min-w-0 flex-1 items-center gap-2.5 rounded-md px-2 text-sm font-medium hover:bg-white/5 focus-visible:outline-none focus-visible:ring-2 focus-visible:ring-ring"
                        >
                            <span
                                aria-hidden="true"
                                class="grid size-7 shrink-0 place-items-center rounded-md bg-white/10 text-[10px] font-semibold"
                            >
                                (tenant_initial)
                            </span>
                            <span class="truncate">(tenant_name)</span>
                        </a>
                        dropdown_menu(
                            attrs: attributes! { class="shrink-0" },
                            dropdown_menu_trigger(
                                attrs: attributes! {
                                    aria-label="Switch tenant"
                                    class="grid h-10 w-7 place-items-center rounded-md text-sidebar-muted hover:bg-white/5 hover:text-sidebar-foreground"
                                },
                                icon(
                                    data: iconify_icon!("feather:chevron-down"),
                                    attrs: attributes! {
                                        aria-hidden="true"
                                        class="size-3 transition-transform group-open:rotate-180"
                                    }
                                )
                            )
                            dropdown_menu_content(
                                attrs: attributes! { class="top-full right-0 left-auto mt-1 w-[12.5rem]" },
                                dropdown_menu_label("Tenants")
                                for candidate in mock::tenants() {
                                    let candidate_path = format!("/tenants/{}", candidate.slug);
                                    let candidate_name = if candidate.slug == tenant.slug {
                                        tenant_name
                                    } else {
                                        candidate.name
                                    };

                                    dropdown_menu_link(
                                        attrs: attributes! {
                                            href=(candidate_path)
                                            aria-current=(if candidate.slug == tenant.slug { "true" } else { "false" })
                                        },
                                        if candidate.slug == tenant.slug {
                                            icon(
                                                data: iconify_icon!("feather:check"),
                                                attrs: attributes! {
                                                    aria-hidden="true"
                                                    class="size-4"
                                                }
                                            )
                                        } else {
                                            <span
                                                aria-hidden="true"
                                                class="grid size-4 place-items-center rounded bg-foreground/8 text-[9px] font-semibold"
                                            >
                                                (candidate.name.chars().next().unwrap_or('T'))
                                            </span>
                                        }
                                        <span class="min-w-0 flex-1 truncate text-left">(candidate_name)</span>
                                    )
                                }
                            )
                        )
                    </div>
                </div>

                <nav class="flex-1 p-3" aria-label="Tenant">
                    <div class="space-y-1">
                        if projects_current {
                            <span aria-current="page" class="sidebar-link sidebar-link-active">
                                icon(
                                    data: iconify_icon!("feather:grid"),
                                    attrs: attributes! {
                                        aria-hidden="true"
                                        class="size-4"
                                    }
                                )
                                "Projects"
                                <span class="ml-auto text-xs text-sidebar-muted">(project_count)</span>
                            </span>
                        } else {
                            <a
                                href=(tenant_root.clone())
                                class=(if projects_active { "sidebar-link sidebar-link-active" } else { "sidebar-link" })
                            >
                                icon(
                                    data: iconify_icon!("feather:grid"),
                                    attrs: attributes! {
                                        aria-hidden="true"
                                        class="size-4"
                                    }
                                )
                                "Projects"
                                <span class="ml-auto text-xs text-sidebar-muted">(project_count)</span>
                            </a>
                        }
                        if changes_active {
                            <span aria-current="page" class="sidebar-link sidebar-link-active">
                                icon(
                                    data: iconify_icon!("feather:activity"),
                                    attrs: attributes! {
                                        aria-hidden="true"
                                        class="size-4"
                                    }
                                )
                                "Changes"
                            </span>
                        } else {
                            <a href=(changes_path.clone()) class="sidebar-link">
                                icon(
                                    data: iconify_icon!("feather:activity"),
                                    attrs: attributes! {
                                        aria-hidden="true"
                                        class="size-4"
                                    }
                                )
                                "Changes"
                            </a>
                        }
                        if usage_active {
                            <span aria-current="page" class="sidebar-link sidebar-link-active">
                                icon(
                                    data: iconify_icon!("feather:bar-chart-2"),
                                    attrs: attributes! {
                                        aria-hidden="true"
                                        class="size-4"
                                    }
                                )
                                "Usage"
                            </span>
                        } else {
                            <a href=(usage_path.clone()) class="sidebar-link">
                                icon(
                                    data: iconify_icon!("feather:bar-chart-2"),
                                    attrs: attributes! {
                                        aria-hidden="true"
                                        class="size-4"
                                    }
                                )
                                "Usage"
                            </a>
                        }
                    </div>
                    <div class="mt-3 border-t border-sidebar-border pt-3">
                        if tenant_settings_active {
                            <span aria-current="page" class="sidebar-link sidebar-link-active">
                                icon(
                                    data: iconify_icon!("feather:settings"),
                                    attrs: attributes! {
                                        aria-hidden="true"
                                        class="size-4"
                                    }
                                )
                                "Tenant settings"
                            </span>
                        } else {
                            <a href=(settings_path.clone()) class="sidebar-link">
                                icon(
                                    data: iconify_icon!("feather:settings"),
                                    attrs: attributes! {
                                        aria-hidden="true"
                                        class="size-4"
                                    }
                                )
                                "Tenant settings"
                            </a>
                        }
                    </div>
                </nav>

                <div class="border-t border-sidebar-border p-3">
                    dropdown_menu(
                        attrs: attributes! { class="w-full" },
                        dropdown_menu_trigger(
                            attrs: attributes! {
                                aria-label="Open account menu"
                                class="flex h-10 w-full items-center gap-2.5 rounded-md px-2 hover:bg-white/5"
                            },
                            <span
                                aria-hidden="true"
                                class="grid size-7 shrink-0 place-items-center rounded-full bg-white/10 text-[10px] font-semibold"
                            >
                                "KD"
                            </span>
                            <span class="min-w-0 flex-1 truncate text-left text-sm">"Khue Doan"</span>
                            icon(
                                data: iconify_icon!("feather:more-horizontal"),
                                attrs: attributes! {
                                    aria-hidden="true"
                                    class="size-4 text-sidebar-muted"
                                }
                            )
                        )
                        dropdown_menu_content(
                            attrs: attributes! {
                                class="right-0 left-0"
                                style="top: auto; bottom: 100%; margin-top: 0; margin-bottom: 0.25rem;"
                            },
                            <form action="/login" method="get">
                                dropdown_menu_item(
                                    attrs: attributes! { type="submit" },
                                    icon(
                                        data: iconify_icon!("feather:log-out"),
                                        attrs: attributes! {
                                            aria-hidden="true"
                                            class="size-4"
                                        }
                                    )
                                    "Exit demo"
                                )
                            </form>
                        )
                    )
                </div>
            </aside>

            <div class="min-w-0">
                <header class="sticky top-0 z-40 flex h-14 items-center border-b border-border bg-background/95 px-4 backdrop-blur lg:hidden">
                    <a
                        href=(tenant_root.clone())
                        aria-label="Netamos home"
                        class="flex shrink-0 items-center gap-2 font-semibold"
                    >
                        <span class="grid size-7 place-items-center rounded-md bg-primary text-primary-foreground">
                            icon(
                                data: iconify_icon!("feather:hexagon"),
                                attrs: attributes! {
                                    aria-hidden="true"
                                    class="size-4"
                                }
                            )
                        </span>
                        <span class="hidden sm:inline">"Netamos"</span>
                    </a>

                    <div class="ml-auto mr-1 flex min-w-0 items-stretch gap-0.5">
                        <a
                            href=(tenant_root.clone())
                            aria-label=(format!("Open {tenant_name} tenant home"))
                            class="flex h-8 min-w-0 max-w-32 items-center gap-2 rounded-md px-1.5 text-sm font-medium hover:bg-surface focus-visible:outline-none focus-visible:ring-2 focus-visible:ring-ring sm:max-w-44"
                        >
                            <span
                                aria-hidden="true"
                                class="grid size-6 shrink-0 place-items-center rounded-md bg-surface text-[10px] font-semibold"
                            >
                                (tenant_initial)
                            </span>
                            <span class="truncate">(tenant_name)</span>
                        </a>
                        dropdown_menu(
                            attrs: attributes! { class="shrink-0" },
                            dropdown_menu_trigger(
                                attrs: attributes! {
                                    aria-label="Switch tenant"
                                    class="grid h-8 w-7 place-items-center rounded-md text-muted-foreground hover:bg-surface hover:text-foreground"
                                },
                                icon(
                                    data: iconify_icon!("feather:chevron-down"),
                                    attrs: attributes! {
                                        aria-hidden="true"
                                        class="size-3 transition-transform group-open:rotate-180"
                                    }
                                )
                            )
                            dropdown_menu_content(
                                attrs: attributes! { class="top-full right-0 left-auto mt-1 w-64" },
                                dropdown_menu_label("Tenants")
                                for candidate in mock::tenants() {
                                    let candidate_path = format!("/tenants/{}", candidate.slug);
                                    let candidate_name = if candidate.slug == tenant.slug {
                                        tenant_name
                                    } else {
                                        candidate.name
                                    };

                                    dropdown_menu_link(
                                        attrs: attributes! {
                                            href=(candidate_path)
                                            aria-current=(if candidate.slug == tenant.slug { "true" } else { "false" })
                                        },
                                        if candidate.slug == tenant.slug {
                                            icon(
                                                data: iconify_icon!("feather:check"),
                                                attrs: attributes! {
                                                    aria-hidden="true"
                                                    class="size-4"
                                                }
                                            )
                                        } else {
                                            <span
                                                aria-hidden="true"
                                                class="grid size-4 place-items-center rounded bg-foreground/8 text-[9px] font-semibold"
                                            >
                                                (candidate.name.chars().next().unwrap_or('T'))
                                            </span>
                                        }
                                        <span class="min-w-0 flex-1 truncate text-left">(candidate_name)</span>
                                        <span class="shrink-0 text-xs text-muted-foreground">(candidate.slug)</span>
                                    )
                                }
                            )
                        )
                    </div>

                    dropdown_menu(
                        dropdown_menu_trigger(
                            attrs: attributes! {
                                aria-label="Open navigation"
                                class="grid size-8 place-items-center rounded-md hover:bg-surface"
                            },
                            icon(
                                data: iconify_icon!("feather:menu"),
                                attrs: attributes! {
                                    aria-hidden="true"
                                    class="size-4"
                                }
                            )
                        )
                        dropdown_menu_content(
                            attrs: attributes! { class="top-full right-0 left-auto mt-1 w-56" },
                            dropdown_menu_label("Navigate")
                            dropdown_menu_link(
                                attrs: attributes! {
                                    href=(tenant_root.clone())
                                    aria-current=(if projects_current { "page" } else { "false" })
                                    class=(if projects_active { "bg-foreground/5" } else { "" })
                                },
                                icon(
                                    data: iconify_icon!("feather:grid"),
                                    attrs: attributes! {
                                        aria-hidden="true"
                                        class="size-4"
                                    }
                                )
                                "Projects"
                            )
                            dropdown_menu_link(
                                attrs: attributes! {
                                    href=(changes_path.clone())
                                    aria-current=(if changes_active { "page" } else { "false" })
                                    class=(if changes_active { "bg-foreground/5" } else { "" })
                                },
                                icon(
                                    data: iconify_icon!("feather:activity"),
                                    attrs: attributes! {
                                        aria-hidden="true"
                                        class="size-4"
                                    }
                                )
                                "Changes"
                            )
                            dropdown_menu_link(
                                attrs: attributes! {
                                    href=(usage_path.clone())
                                    aria-current=(if usage_active { "page" } else { "false" })
                                    class=(if usage_active { "bg-foreground/5" } else { "" })
                                },
                                icon(
                                    data: iconify_icon!("feather:bar-chart-2"),
                                    attrs: attributes! {
                                        aria-hidden="true"
                                        class="size-4"
                                    }
                                )
                                "Usage"
                            )
                            dropdown_menu_separator()
                            dropdown_menu_link(
                                attrs: attributes! {
                                    href=(settings_path.clone())
                                    aria-current=(if tenant_settings_active { "page" } else { "false" })
                                    class=(if tenant_settings_active { "bg-foreground/5" } else { "" })
                                },
                                icon(
                                    data: iconify_icon!("feather:settings"),
                                    attrs: attributes! {
                                        aria-hidden="true"
                                        class="size-4"
                                    }
                                )
                                "Tenant settings"
                            )
                            dropdown_menu_separator()
                            <form action="/login" method="get">
                                dropdown_menu_item(
                                    attrs: attributes! { type="submit" },
                                    icon(
                                        data: iconify_icon!("feather:log-out"),
                                        attrs: attributes! {
                                            aria-hidden="true"
                                            class="size-4"
                                        }
                                    )
                                    "Exit demo"
                                )
                            </form>
                        )
                    )
                </header>
                (slot.await?)
            </div>
        </div>
    }
}
