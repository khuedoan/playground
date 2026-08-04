use crate::mock;
use topcoat::{
    Result,
    asset::asset,
    context::Cx,
    icon::{icon, iconify::iconify_icon},
    router::{RouterErrorExt, Slot, layout, path_param, query_params, redirect_permanent, uri},
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
        <html lang="en" data-theme="light">
            <head>
                <meta charset="utf-8">
                <meta name="viewport" content="width=device-width, initial-scale=1">
                <title>"Netamos"</title>
                <link rel="stylesheet" href=(asset!("assets/main.css"))>
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
    let stored_tenant_name = tenant.name;
    let tenant_name = if query.action.as_deref() == Some("save-tenant") {
        query.display_name.as_deref().unwrap_or(stored_tenant_name)
    } else {
        stored_tenant_name
    };
    let project_count = tenant.projects.len();

    view! {
        <div class="drawer lg:drawer-open">
            <input id="app-drawer" type="checkbox" class="drawer-toggle">

            <div class="drawer-content min-w-0">
                <header class="navbar bg-base-100 lg:hidden">
                    <div class="navbar-start">
                        <label for="app-drawer" class="btn btn-square btn-ghost" aria-label="Open navigation">
                            icon(
                                data: iconify_icon!("feather:menu"),
                                attrs: attributes! { aria-hidden="true" class="size-4" }
                            )
                        </label>
                    </div>

                    <div class="navbar-center">
                        <a href=(tenant_root.clone()) class="btn btn-ghost text-xl" aria-label="Netamos home">
                            icon(
                                data: iconify_icon!("feather:hexagon"),
                                attrs: attributes! { aria-hidden="true" class="size-4" }
                            )
                            "Netamos"
                        </a>
                    </div>

                    <div class="navbar-end">
                        <details class="dropdown dropdown-end" data-dropdown-menu="">
                            <summary class="btn btn-ghost" aria-label="Switch tenant">(tenant_name)</summary>
                            <ul class="menu dropdown-content bg-base-100 rounded-box z-50 mt-2 w-52 p-2 shadow-sm">
                                <li class="menu-title">"Tenants"</li>
                                for candidate in mock::tenants() {
                                    let candidate_path = format!("/tenants/{}", candidate.slug);
                                    let candidate_name = if candidate.slug == tenant.slug {
                                        tenant_name
                                    } else {
                                        candidate.name
                                    };

                                    <li>
                                        <a href=(candidate_path) aria-current=(if candidate.slug == tenant.slug { "true" } else { "false" })>
                                            if candidate.slug == tenant.slug {
                                                icon(
                                                    data: iconify_icon!("feather:check"),
                                                    attrs: attributes! { aria-hidden="true" class="size-4" }
                                                )
                                            }
                                            (candidate_name)
                                        </a>
                                    </li>
                                }
                            </ul>
                        </details>
                    </div>
                </header>

                (slot.await?)
            </div>

            <div class="drawer-side z-50">
                <label for="app-drawer" aria-label="Close navigation" class="drawer-overlay"></label>
                <aside class="flex min-h-full w-64 flex-col bg-base-200">
                    <div class="navbar">
                        <a href=(tenant_root.clone()) class="btn btn-ghost text-xl" aria-label="Netamos home">
                            icon(
                                data: iconify_icon!("feather:hexagon"),
                                attrs: attributes! { aria-hidden="true" class="size-4" }
                            )
                            "Netamos"
                        </a>
                    </div>

                    <details class="dropdown dropdown-bottom mx-2" data-dropdown-menu="">
                        <summary class="btn btn-ghost w-full justify-start" aria-label="Switch tenant">
                            icon(
                                data: iconify_icon!("feather:users"),
                                attrs: attributes! { aria-hidden="true" class="size-4" }
                            )
                            (tenant_name)
                            icon(
                                data: iconify_icon!("feather:chevron-down"),
                                attrs: attributes! { aria-hidden="true" class="size-4" }
                            )
                        </summary>
                        <ul class="menu dropdown-content bg-base-100 rounded-box z-50 mt-2 w-52 p-2 shadow-sm">
                            <li class="menu-title">"Tenants"</li>
                            for candidate in mock::tenants() {
                                let candidate_path = format!("/tenants/{}", candidate.slug);
                                let candidate_name = if candidate.slug == tenant.slug {
                                    tenant_name
                                } else {
                                    candidate.name
                                };

                                <li>
                                    <a href=(candidate_path) aria-current=(if candidate.slug == tenant.slug { "true" } else { "false" })>
                                        if candidate.slug == tenant.slug {
                                            icon(
                                                data: iconify_icon!("feather:check"),
                                                attrs: attributes! { aria-hidden="true" class="size-4" }
                                            )
                                        }
                                        (candidate_name)
                                    </a>
                                </li>
                            }
                        </ul>
                    </details>

                    <ul class="menu w-full flex-1">
                        <li>
                            <a href=(tenant_root.clone()) class=(if projects_active { "menu-active" } else { "" }) aria-current=(if projects_active { "page" } else { "false" })>
                                icon(
                                    data: iconify_icon!("feather:grid"),
                                    attrs: attributes! { aria-hidden="true" class="size-4" }
                                )
                                "Projects"
                                <span class="badge badge-sm">(project_count)</span>
                            </a>
                        </li>
                        <li>
                            <a href=(changes_path) class=(if changes_active { "menu-active" } else { "" }) aria-current=(if changes_active { "page" } else { "false" })>
                                icon(
                                    data: iconify_icon!("feather:activity"),
                                    attrs: attributes! { aria-hidden="true" class="size-4" }
                                )
                                "Changes"
                            </a>
                        </li>
                        <li>
                            <a href=(usage_path) class=(if usage_active { "menu-active" } else { "" }) aria-current=(if usage_active { "page" } else { "false" })>
                                icon(
                                    data: iconify_icon!("feather:bar-chart-2"),
                                    attrs: attributes! { aria-hidden="true" class="size-4" }
                                )
                                "Usage"
                            </a>
                        </li>
                        <li>
                            <a href=(settings_path) class=(if tenant_settings_active { "menu-active" } else { "" }) aria-current=(if tenant_settings_active { "page" } else { "false" })>
                                icon(
                                    data: iconify_icon!("feather:settings"),
                                    attrs: attributes! { aria-hidden="true" class="size-4" }
                                )
                                "Tenant settings"
                            </a>
                        </li>
                    </ul>

                    <form action="/login" method="get" class="m-2">
                        <button class="btn btn-ghost w-full justify-start" type="submit">
                            icon(
                                data: iconify_icon!("feather:log-out"),
                                attrs: attributes! { aria-hidden="true" class="size-4" }
                            )
                            "Exit demo"
                        </button>
                    </form>
                </aside>
            </div>
        </div>
    }
}
