use crate::{
    components::{
        badge::{BadgeVariant, badge},
        button::{ButtonSize, ButtonVariant, button, button_variants},
        card::{card, card_content, card_description, card_footer, card_header, card_title},
        input::input,
        label::label,
        progress::progress,
        select::select,
        switch::switch,
        textarea::textarea,
    },
    mock,
};
use topcoat::{
    Result,
    context::Cx,
    icon::{icon, iconify::iconify_icon},
    router::{RouterErrorExt, Slot, layout, page, path_param, redirect_permanent, uri},
    view::{attributes, view},
};

#[path_param]
struct Tenant(str);

#[path_param]
struct Project(str);

#[path_param]
struct Environment(str);

#[path_param]
struct Component(str);

#[page("/")]
async fn index() -> Result {
    Err(redirect_permanent("/login").into())
}

#[page("/login")]
async fn login() -> Result {
    view! {
        <main class="grid min-h-screen place-items-center px-4 py-12">
            card(
                attrs: attributes! { class="w-full max-w-md" },
                card_header(
                    attrs: attributes! { class="items-center text-center" },
                    <span class="mb-2">
                        icon(data: iconify_icon!("feather:hexagon"), attrs: attributes! { class="size-10" })
                    </span>
                    card_title(attrs: attributes! { class="text-xl" }, "Welcome to Netamos")
                    card_description("Deploy and operate applications on your own infrastructure.")
                )
                card_content(
                    <form action="/tenants/default" method="get">
                        button(
                            attrs: attributes! { class="w-full" type="submit" },
                            icon(data: iconify_icon!("feather:log-in"), attrs: attributes! { class="size-4" })
                            "Continue with SSO"
                        )
                    </form>
                    <p class="mt-4 text-center text-xs text-muted-foreground">
                        "Authentication is managed by your organization."
                    </p>
                )
            )
        </main>
    }
}

#[page("/tenants/new")]
async fn create_tenant() -> Result {
    view! {
        <main class="mx-auto max-w-2xl px-4 py-8 sm:px-6">
            <header class="mb-6">
                <p class="text-sm text-muted-foreground">"Tenant"</p>
                <h1 class="text-2xl font-semibold">"Create tenant"</h1>
                <p class="mt-1 text-sm text-muted-foreground">"Create an isolated home for projects and domains."</p>
            </header>
            card(
                card_header(
                    card_title("Tenant details")
                    card_description("Names can be changed later without changing the slug.")
                )
                card_content(
                    <form action="/tenants/default" method="get" class="space-y-5">
                        <div class="space-y-2">
                            label(attrs: attributes! { for="tenant-name" }, "Display name")
                            input(attrs: attributes! { id="tenant-name" name="name" value="Netamos" required=(true) })
                        </div>
                        <div class="space-y-2">
                            label(attrs: attributes! { for="tenant-slug" }, "Slug")
                            input(attrs: attributes! { id="tenant-slug" name="slug" value="default" required=(true) })
                            <p class="text-xs text-muted-foreground">"Used in URLs and Kubernetes resource names."</p>
                        </div>
                        <div class="flex justify-end gap-2">
                            <a href="/tenants/default" class=(button_variants(ButtonVariant::Outline, ButtonSize::Md))>"Cancel"</a>
                            button(attrs: attributes! { type="submit" }, "Create tenant")
                        </div>
                    </form>
                )
            )
        </main>
    }
}

#[page("/tenants/{tenant}")]
async fn tenant_overview(cx: &Cx) -> Result {
    let tenant = mock::tenant(path_param::<Tenant>(cx)).ok_or_not_found()?;
    let environment_count = tenant
        .projects
        .iter()
        .map(|project| project.environments.len())
        .sum::<usize>();

    view! {
        <main class="mx-auto max-w-6xl px-4 py-8 sm:px-6">
            <header class="mb-6 flex flex-wrap items-start justify-between gap-4">
                <div>
                    <p class="text-sm text-muted-foreground">"Tenant"</p>
                    <h1 class="text-2xl font-semibold">(tenant.name)</h1>
                    <p class="mt-1 text-sm text-muted-foreground">
                        (format!("{} projects · {} environments", tenant.projects.len(), environment_count))
                    </p>
                </div>
                <a
                    href=(format!("{}/projects/new", tenant_path(tenant.slug)))
                    class=(button_variants(ButtonVariant::Primary, ButtonSize::Md))
                >
                    icon(data: iconify_icon!("feather:plus"), attrs: attributes! { class="size-4" })
                    "New project"
                </a>
            </header>

            <div class="grid gap-6 lg:grid-cols-[minmax(0,1fr)_22rem]">
                <section aria-labelledby="projects-heading">
                    <header class="mb-3">
                        <h2 id="projects-heading" class="text-lg font-semibold">"Projects"</h2>
                        <p class="text-sm text-muted-foreground">"Applications grouped by delivery lifecycle."</p>
                    </header>
                    <div class="grid gap-4 md:grid-cols-2">
                        for project in tenant.projects {
                            let (project_href, action_label) = if project.environments.len() == 1 {
                                (
                                    environment_path(
                                        tenant.slug,
                                        project.slug,
                                        project.environments[0].slug,
                                    ),
                                    format!("Open {}", project.environments[0].name),
                                )
                            } else {
                                (
                                    project_path(tenant.slug, project.slug),
                                    "View project".to_string(),
                                )
                            };

                            card(
                                attrs: attributes! { class="h-full" },
                                card_header(
                                    card_title(
                                        <span class="flex items-center gap-2">
                                            icon(data: iconify_icon!("feather:folder"), attrs: attributes! { class="size-4" })
                                            (project.name)
                                        </span>
                                    )
                                    card_description((project.description))
                                )
                                card_footer(
                                    attrs: attributes! { class="mt-auto justify-end" },
                                    <a
                                        href=(project_href)
                                        class=(button_variants(ButtonVariant::Outline, ButtonSize::Sm))
                                    >
                                        (action_label)
                                        icon(data: iconify_icon!("feather:arrow-right"), attrs: attributes! { class="size-4" })
                                    </a>
                                )
                            )
                        }
                    </div>
                </section>

                <aside aria-labelledby="changes-heading">
                    card(
                        card_header(
                            card_title(attrs: attributes! { id="changes-heading" }, "Recent changes")
                            card_description("Desired-state updates from Git.")
                        )
                        card_content(change_list(changes: tenant.changes))
                    )
                </aside>
            </div>
        </main>
    }
}

#[page("/tenants/{tenant}/projects/new")]
async fn create_project(cx: &Cx) -> Result {
    let tenant_slug = path_param::<Tenant>(cx);
    mock::tenant(tenant_slug).ok_or_not_found()?;
    let action = project_path(tenant_slug, "storefront");

    view! {
        <main class="mx-auto max-w-2xl px-4 py-8 sm:px-6">
            <nav class="mb-4 text-sm text-muted-foreground" aria-label="Breadcrumb">
                <a href=(tenant_path(tenant_slug)) class="hover:underline">"Home"</a>
                " / "
                <span aria-current="page">"New project"</span>
            </nav>
            <header class="mb-6">
                <h1 class="text-2xl font-semibold">"Create project"</h1>
                <p class="mt-1 text-sm text-muted-foreground">"A production environment is created automatically."</p>
            </header>
            card(
                card_header(card_title("Project details"))
                card_content(
                    <form action=(action) method="get" class="space-y-5">
                        <div class="space-y-2">
                            label(attrs: attributes! { for="project-name" }, "Name")
                            input(attrs: attributes! { id="project-name" name="name" value="storefront" required=(true) })
                        </div>
                        <div class="space-y-2">
                            label(attrs: attributes! { for="project-description" }, "Description")
                            textarea(
                                attrs: attributes! { id="project-description" name="description" },
                                "Customer-facing commerce services"
                            )
                        </div>
                        <div class="flex justify-end gap-2">
                            <a href=(tenant_path(tenant_slug)) class=(button_variants(ButtonVariant::Outline, ButtonSize::Md))>"Cancel"</a>
                            button(attrs: attributes! { type="submit" }, "Create project")
                        </div>
                    </form>
                )
            )
        </main>
    }
}

#[page("/tenants/{tenant}/projects/{project}")]
async fn project_overview(cx: &Cx) -> Result {
    let tenant_slug = path_param::<Tenant>(cx);
    let project = mock::project(tenant_slug, path_param::<Project>(cx)).ok_or_not_found()?;

    view! {
        <main class="mx-auto max-w-6xl px-4 py-8 sm:px-6">
            <nav class="mb-4 text-sm text-muted-foreground" aria-label="Breadcrumb">
                <a href=(tenant_path(tenant_slug)) class="hover:underline">"Home"</a>
                " / "
                <span aria-current="page">(project.name)</span>
            </nav>
            <header class="mb-6 flex flex-wrap items-start justify-between gap-4">
                <div>
                    <p class="text-sm text-muted-foreground">"Project"</p>
                    <h1 class="text-2xl font-semibold">(project.name)</h1>
                    <p class="mt-1 text-sm text-muted-foreground">(project.description)</p>
                </div>
                <a
                    href=(format!("{}/environments/new", project_path(tenant_slug, project.slug)))
                    class=(button_variants(ButtonVariant::Primary, ButtonSize::Md))
                >
                    icon(data: iconify_icon!("feather:plus"), attrs: attributes! { class="size-4" })
                    "New environment"
                </a>
            </header>

            card(
                card_header(
                    card_title("Environments")
                    card_description("Deployment targets for this project.")
                )
                card_content(
                    <table class="w-full text-sm">
                        <thead class="text-left text-muted-foreground">
                            <tr>
                                <th scope="col" class="pb-2 font-medium">"Environment"</th>
                                <th scope="col" class="pb-2 font-medium">"Region"</th>
                                <th scope="col" class="pb-2 text-right font-medium">"Components"</th>
                            </tr>
                        </thead>
                        <tbody class="divide-y divide-border">
                            for environment in project.environments {
                                <tr>
                                    <th scope="row" class="py-3 text-left">
                                        <a
                                            href=(environment_path(tenant_slug, project.slug, environment.slug))
                                            class="font-medium hover:underline"
                                        >
                                            icon(data: iconify_icon!("feather:layers"), attrs: attributes! { class="mr-2 inline size-4" })
                                            (environment.name)
                                        </a>
                                    </th>
                                    <td class="py-3 text-muted-foreground">(environment.region)</td>
                                    <td class="py-3 text-right">(environment.components.len())</td>
                                </tr>
                            }
                        </tbody>
                    </table>
                )
            )
        </main>
    }
}

#[page("/tenants/{tenant}/projects/{project}/environments/new")]
async fn create_environment(cx: &Cx) -> Result {
    let tenant_slug = path_param::<Tenant>(cx);
    let project_slug = path_param::<Project>(cx);
    let project = mock::project(tenant_slug, project_slug).ok_or_not_found()?;
    let action = environment_path(tenant_slug, project_slug, "production");

    view! {
        <main class="mx-auto max-w-2xl px-4 py-8 sm:px-6">
            <nav class="mb-4 text-sm text-muted-foreground" aria-label="Breadcrumb">
                <a href=(tenant_path(tenant_slug)) class="hover:underline">"Home"</a>
                " / "
                <a href=(project_path(tenant_slug, project_slug)) class="hover:underline">(project.name)</a>
                " / "
                <span aria-current="page">"New environment"</span>
            </nav>
            <header class="mb-6">
                <h1 class="text-2xl font-semibold">"Create environment"</h1>
                <p class="mt-1 text-sm text-muted-foreground">"Create an isolated deployment target."</p>
            </header>
            card(
                card_header(card_title("Environment details"))
                card_content(
                    <form action=(action) method="get" class="space-y-5">
                        <div class="space-y-2">
                            label(attrs: attributes! { for="environment-name" }, "Name")
                            input(attrs: attributes! { id="environment-name" name="name" value="production" required=(true) })
                        </div>
                        <div class="space-y-2">
                            label(attrs: attributes! { for="environment-region" }, "Region")
                            select(
                                attrs: attributes! { id="environment-region" name="region" },
                                <option value="helsinki">"Helsinki"</option>
                                <option value="frankfurt">"Frankfurt"</option>
                            )
                        </div>
                        <div class="flex justify-end gap-2">
                            <a href=(project_path(tenant_slug, project_slug)) class=(button_variants(ButtonVariant::Outline, ButtonSize::Md))>"Cancel"</a>
                            button(attrs: attributes! { type="submit" }, "Create environment")
                        </div>
                    </form>
                )
            )
        </main>
    }
}

#[page("/tenants/{tenant}/projects/{project}/environments/{environment}")]
async fn environment_overview(cx: &Cx) -> Result {
    let tenant_slug = path_param::<Tenant>(cx);
    let project_slug = path_param::<Project>(cx);
    let project = mock::project(tenant_slug, project_slug).ok_or_not_found()?;
    let environment = mock::environment(tenant_slug, project_slug, path_param::<Environment>(cx))
        .ok_or_not_found()?;

    view! {
        <main class="mx-auto max-w-6xl px-4 py-8 sm:px-6">
            <nav class="mb-4 text-sm text-muted-foreground" aria-label="Breadcrumb">
                <a href=(tenant_path(tenant_slug)) class="hover:underline">"Home"</a>
                " / "
                <a href=(project_path(tenant_slug, project_slug)) class="hover:underline">(project.name)</a>
                " / "
                <span aria-current="page">(environment.name)</span>
            </nav>
            <header class="mb-6 flex flex-wrap items-start justify-between gap-4">
                <div>
                    <p class="text-sm text-muted-foreground">"Environment"</p>
                    <h1 class="text-2xl font-semibold">(environment.name)</h1>
                    <p class="mt-1 text-sm text-muted-foreground">(environment.region)</p>
                </div>
                <div class="flex flex-wrap items-center gap-2">
                    badge(variant: BadgeVariant::Outline, "Telemetry connected")
                    <a
                        href=(format!(
                            "{}/new-component",
                            environment_path(tenant_slug, project_slug, environment.slug),
                        ))
                        class=(button_variants(ButtonVariant::Primary, ButtonSize::Md))
                    >
                        icon(data: iconify_icon!("feather:plus"), attrs: attributes! { class="size-4" })
                        "New component"
                    </a>
                </div>
            </header>

            card(
                card_header(
                    card_title("Components")
                    card_description("Desired workloads with runtime output.")
                )
                card_content(
                    <table class="w-full text-sm">
                        <thead class="text-left text-muted-foreground">
                            <tr>
                                <th scope="col" class="pb-2 font-medium">"Component"</th>
                                <th scope="col" class="pb-2 font-medium">"Type"</th>
                                <th scope="col" class="pb-2 text-right font-medium">"Status"</th>
                            </tr>
                        </thead>
                        <tbody class="divide-y divide-border">
                            for component in environment.components {
                                <tr>
                                    <th scope="row" class="py-3 text-left">
                                        <a
                                            href=(component_path(
                                                tenant_slug,
                                                project_slug,
                                                environment.slug,
                                                component.slug,
                                            ))
                                            class="font-mono font-medium hover:underline"
                                        >
                                            icon(data: iconify_icon!("feather:box"), attrs: attributes! { class="mr-2 inline size-4" })
                                            (component.name)
                                        </a>
                                        <p class="mt-1 font-sans font-normal text-muted-foreground">(component.summary)</p>
                                    </th>
                                    <td class="py-3 text-muted-foreground">(component.kind)</td>
                                    <td class="py-3 text-right">
                                        match component.observability {
                                            Some(observability) => badge(
                                                variant: BadgeVariant::Secondary,
                                                (observability.health)
                                            ),
                                            None => badge(
                                                variant: BadgeVariant::Outline,
                                                (component.state)
                                            ),
                                        }
                                    </td>
                                </tr>
                            }
                        </tbody>
                    </table>
                )
            )
        </main>
    }
}

#[page("/tenants/{tenant}/projects/{project}/environments/{environment}/new-component")]
async fn create_component(cx: &Cx) -> Result {
    let tenant_slug = path_param::<Tenant>(cx);
    let project_slug = path_param::<Project>(cx);
    let environment_slug = path_param::<Environment>(cx);
    let environment =
        mock::environment(tenant_slug, project_slug, environment_slug).ok_or_not_found()?;
    let action = component_path(tenant_slug, project_slug, environment_slug, "web");

    view! {
        <main class="mx-auto max-w-2xl px-4 py-8 sm:px-6">
            <nav class="mb-4 text-sm text-muted-foreground" aria-label="Breadcrumb">
                <a href=(environment_path(tenant_slug, project_slug, environment_slug)) class="hover:underline">
                    (environment.name)
                </a>
                " / "
                <span aria-current="page">"New component"</span>
            </nav>
            <header class="mb-6">
                <h1 class="text-2xl font-semibold">"Create component"</h1>
                <p class="mt-1 text-sm text-muted-foreground">"Deploy from source, a container image, or a managed service."</p>
            </header>
            card(
                card_header(card_title("Component details"))
                card_content(
                    <form action=(action) method="get" class="space-y-5">
                        <div class="grid gap-5 sm:grid-cols-2">
                            <div class="space-y-2">
                                label(attrs: attributes! { for="component-kind" }, "Type")
                                select(
                                    attrs: attributes! { id="component-kind" name="kind" },
                                    <option value="application">"Application"</option>
                                    <option value="cron-job">"Cron job"</option>
                                    <option value="postgresql">"PostgreSQL"</option>
                                    <option value="valkey">"Valkey"</option>
                                )
                            </div>
                            <div class="space-y-2">
                                label(attrs: attributes! { for="component-name" }, "Name")
                                input(attrs: attributes! { id="component-name" name="name" value="web" required=(true) })
                            </div>
                        </div>
                        <div class="space-y-2">
                            label(attrs: attributes! { for="component-source" }, "Source")
                            input(
                                attrs: attributes! {
                                    id="component-source"
                                    name="source"
                                    value="forgejo.example/acme/storefront"
                                    required=(true)
                                }
                            )
                            <p class="text-xs text-muted-foreground">"Repository or container image reference."</p>
                        </div>
                        <div class="grid gap-5 sm:grid-cols-2">
                            <div class="space-y-2">
                                label(attrs: attributes! { for="component-replicas" }, "Replicas")
                                input(attrs: attributes! { id="component-replicas" name="replicas" type="number" value="2" min="0" })
                            </div>
                            <div class="space-y-2">
                                label(attrs: attributes! { for="component-port" }, "Port")
                                input(attrs: attributes! { id="component-port" name="port" type="number" value="3000" min="1" })
                            </div>
                        </div>
                        <div class="flex items-center gap-2">
                            switch(attrs: attributes! { id="auto-deploy" name="auto-deploy" checked=(true) })
                            label(attrs: attributes! { for="auto-deploy" }, "Deploy when the source branch changes")
                        </div>
                        <div class="flex justify-end gap-2">
                            <a href=(environment_path(tenant_slug, project_slug, environment_slug)) class=(button_variants(ButtonVariant::Outline, ButtonSize::Md))>"Cancel"</a>
                            button(attrs: attributes! { type="submit" }, "Create component")
                        </div>
                    </form>
                )
            )
        </main>
    }
}

#[layout("/tenants/{tenant}/projects/{project}/environments/{environment}/components/{component}")]
async fn component_layout(cx: &Cx, slot: Slot<'_>) -> Result {
    let tenant_slug = path_param::<Tenant>(cx);
    let project_slug = path_param::<Project>(cx);
    let environment_slug = path_param::<Environment>(cx);
    let component = current_component(cx)?;
    let base_path = component_path(tenant_slug, project_slug, environment_slug, component.slug);
    let settings_path = format!("{base_path}/settings");
    let changes_path = format!("{base_path}/changes");
    let current_path = uri(cx).path();

    view! {
        <main class="mx-auto max-w-6xl px-4 py-8 sm:px-6">
            <nav class="mb-4 flex flex-wrap gap-x-1 text-sm text-muted-foreground" aria-label="Breadcrumb">
                <a href=(tenant_path(tenant_slug)) class="hover:underline">"Home"</a>
                " / "
                <a href=(project_path(tenant_slug, project_slug)) class="hover:underline">(project_slug)</a>
                " / "
                <a
                    href=(environment_path(tenant_slug, project_slug, environment_slug))
                    class="hover:underline"
                >
                    (environment_slug)
                </a>
                " / "
                <span aria-current="page">(component.name)</span>
            </nav>
            <header class="mb-4 flex flex-wrap items-start justify-between gap-4">
                <div>
                    <div class="flex flex-wrap items-center gap-2">
                        <h1 class="font-mono text-2xl font-semibold">(component.name)</h1>
                        badge(variant: BadgeVariant::Outline, (component.kind))
                        badge(variant: BadgeVariant::Secondary, (component.state))
                    </div>
                    <p class="mt-1 text-sm text-muted-foreground">(component.summary)</p>
                </div>
                <div class="flex gap-2">
                    match component.url {
                        Some(url) => <a
                            href=(url)
                            target="_blank"
                            rel="noopener"
                            class=(button_variants(ButtonVariant::Outline, ButtonSize::Md))
                        >
                            icon(data: iconify_icon!("feather:external-link"), attrs: attributes! { class="size-4" })
                            "Open app"
                        </a>,
                        None => "",
                    }
                    <a
                        href=(changes_path.clone())
                        class=(button_variants(ButtonVariant::Primary, ButtonSize::Md))
                    >
                        icon(data: iconify_icon!("feather:upload-cloud"), attrs: attributes! { class="size-4" })
                        "Deploy"
                    </a>
                </div>
            </header>
            <nav class="mb-6 flex gap-2" aria-label="Component">
                <a
                    href=(base_path.clone())
                    class=(button_variants(
                        if current_path == base_path {
                            ButtonVariant::Secondary
                        } else {
                            ButtonVariant::Ghost
                        },
                        ButtonSize::Sm,
                    ))
                >
                    "Observability"
                </a>
                <a
                    href=(settings_path.clone())
                    class=(button_variants(
                        if current_path == settings_path {
                            ButtonVariant::Secondary
                        } else {
                            ButtonVariant::Ghost
                        },
                        ButtonSize::Sm,
                    ))
                >
                    "Settings"
                </a>
                <a
                    href=(changes_path.clone())
                    class=(button_variants(
                        if current_path == changes_path {
                            ButtonVariant::Secondary
                        } else {
                            ButtonVariant::Ghost
                        },
                        ButtonSize::Sm,
                    ))
                >
                    "Deployments"
                </a>
            </nav>
            (slot.await?)
        </main>
    }
}

#[page("/tenants/{tenant}/projects/{project}/environments/{environment}/components/{component}")]
async fn component_overview(cx: &Cx) -> Result {
    let component = current_component(cx)?;
    let Some(observability) = component.observability else {
        return view! {
            card(
                card_header(
                    card_title("Observability")
                    card_description("Telemetry will appear after the component is deployed.")
                )
                card_content(
                    badge(variant: BadgeVariant::Outline, "Awaiting deployment")
                )
            )
        };
    };

    view! {
        <section aria-labelledby="observability-heading">
            <header class="mb-4 flex items-center justify-between gap-4">
                <div>
                    <h2 id="observability-heading" class="text-lg font-semibold">"Observability"</h2>
                    <p class="text-sm text-muted-foreground">"A concise view of service health and recent output."</p>
                </div>
                badge(variant: BadgeVariant::Outline, "Last 30 minutes")
            </header>

            <div class="grid grid-cols-2 gap-4 lg:grid-cols-4">
                card(
                    attrs: attributes! { class="gap-3 py-4" },
                    card_content(
                        <dl>
                            <dt class="text-sm text-muted-foreground">"Status"</dt>
                            <dd class="mt-2">badge(variant: BadgeVariant::Secondary, (observability.health))</dd>
                        </dl>
                    )
                )
                card(
                    attrs: attributes! { class="gap-3 py-4" },
                    card_content(
                        <dl>
                            <dt class="text-sm text-muted-foreground">"Uptime"</dt>
                            <dd class="mt-1 text-2xl font-semibold">(observability.uptime)</dd>
                        </dl>
                    )
                )
                card(
                    attrs: attributes! { class="gap-3 py-4" },
                    card_content(
                        <dl>
                            <dt class="text-sm text-muted-foreground">(observability.primary_metric.label)</dt>
                            <dd class="mt-1 text-2xl font-semibold">(observability.primary_metric.value)</dd>
                        </dl>
                    )
                )
                card(
                    attrs: attributes! { class="gap-3 py-4" },
                    card_content(
                        <dl>
                            <dt class="text-sm text-muted-foreground">(observability.secondary_metric.label)</dt>
                            <dd class="mt-1 text-2xl font-semibold">(observability.secondary_metric.value)</dd>
                        </dl>
                    )
                )
            </div>

            <div class="mt-6 grid gap-6 lg:grid-cols-2">
                card(
                    card_header(
                        card_title("Resource usage")
                        card_description("Current utilization across ready replicas.")
                    )
                    card_content(
                        <dl class="space-y-5">
                            <div>
                                <dt class="mb-2 flex justify-between text-sm">
                                    <span class="text-muted-foreground">"CPU"</span>
                                    <span>(format!("{}%", observability.cpu_percent))</span>
                                </dt>
                                <dd>progress(value: observability.cpu_percent, attrs: attributes! { aria-label="CPU utilization" })</dd>
                            </div>
                            <div>
                                <dt class="mb-2 flex justify-between text-sm">
                                    <span class="text-muted-foreground">"Memory"</span>
                                    <span>(format!("{}%", observability.memory_percent))</span>
                                </dt>
                                <dd>progress(value: observability.memory_percent, attrs: attributes! { aria-label="Memory utilization" })</dd>
                            </div>
                        </dl>
                    )
                )

                card(
                    card_header(
                        card_title("Current release")
                        card_description("The active rollout and replica availability.")
                    )
                    card_content(
                        <dl class="space-y-4 text-sm">
                            <div class="flex justify-between gap-4">
                                <dt class="text-muted-foreground">"Release"</dt>
                                <dd class="text-right font-mono">(observability.release)</dd>
                            </div>
                            <div class="flex justify-between gap-4">
                                <dt class="text-muted-foreground">"Replicas"</dt>
                                <dd>(observability.replicas)</dd>
                            </div>
                            <div class="flex justify-between gap-4">
                                <dt class="text-muted-foreground">"Desired state"</dt>
                                <dd>(component.state)</dd>
                            </div>
                        </dl>
                    )
                )
            </div>

            card(
                attrs: attributes! { class="mt-6" },
                card_header(
                    attrs: attributes! { class="flex-row items-center justify-between" },
                    <div>
                        card_title("Recent output")
                        card_description("Latest application log lines.")
                    </div>
                    badge(variant: BadgeVariant::Secondary, "Live tail")
                )
                card_content(
                    <pre
                        class="overflow-x-auto text-xs leading-5"
                        aria-label="Recent application logs"
                    ><samp>for line in observability.logs {
                            <time>(line.time)</time>
                            " "
                            (line.level)
                            " "
                            (line.message)
                            "\n"
                        }</samp></pre>
                )
            )
        </section>
    }
}

#[page(
    "/tenants/{tenant}/projects/{project}/environments/{environment}/components/{component}/settings"
)]
async fn component_settings(cx: &Cx) -> Result {
    let component = current_component(cx)?;
    let action = uri(cx).path();
    let source = setting_value(component, "Source");
    let replicas = setting_value(component, "Replicas");
    let port = setting_value(component, "Port");
    let domain = setting_value(component, "Domain");
    let environment_action = environment_path(
        path_param::<Tenant>(cx),
        path_param::<Project>(cx),
        path_param::<Environment>(cx),
    );

    view! {
        <div class="space-y-6">
            card(
                card_header(
                    card_title("Settings")
                    card_description("Configure source, runtime, networking, and variables.")
                )
                card_content(
                    <form action=(action) method="get" class="space-y-6">
                        <section class="space-y-4" aria-labelledby="source-heading">
                            <h4 id="source-heading" class="font-medium">"Source"</h4>
                            <div class="space-y-2">
                                label(attrs: attributes! { for="settings-source" }, "Repository or image")
                                input(attrs: attributes! { id="settings-source" name="source" value=(source) })
                            </div>
                            <div class="flex items-center gap-2">
                                switch(attrs: attributes! { id="settings-auto-deploy" name="auto-deploy" checked=(true) })
                                label(attrs: attributes! { for="settings-auto-deploy" }, "Automatically deploy source changes")
                            </div>
                        </section>
                        <hr class="border-border">
                        <section class="space-y-4" aria-labelledby="runtime-heading">
                            <h4 id="runtime-heading" class="font-medium">"Runtime"</h4>
                            <div class="grid gap-4 sm:grid-cols-2">
                                <div class="space-y-2">
                                    label(attrs: attributes! { for="settings-replicas" }, "Replicas")
                                    input(attrs: attributes! { id="settings-replicas" name="replicas" type="number" value=(replicas) min="0" })
                                </div>
                                <div class="space-y-2">
                                    label(attrs: attributes! { for="settings-port" }, "Port")
                                    input(attrs: attributes! { id="settings-port" name="port" type="number" value=(port) min="1" })
                                </div>
                            </div>
                        </section>
                        <hr class="border-border">
                        <section class="space-y-4" aria-labelledby="networking-heading">
                            <h4 id="networking-heading" class="font-medium">"Networking"</h4>
                            <div class="space-y-2">
                                label(attrs: attributes! { for="settings-domain" }, "Public domain")
                                input(attrs: attributes! { id="settings-domain" name="domain" value=(domain) })
                            </div>
                        </section>
                        <hr class="border-border">
                        <section class="space-y-4" aria-labelledby="variables-heading">
                            <h4 id="variables-heading" class="font-medium">"Environment variables"</h4>
                            <div class="space-y-2">
                                label(attrs: attributes! { for="settings-variables" }, "Variables")
                                textarea(
                                    attrs: attributes! { id="settings-variables" name="variables" rows="4" },
                                    "RUST_LOG=info\nCHECKOUT_MODE=production"
                                )
                                <p class="text-xs text-muted-foreground">"One NAME=value pair per line."</p>
                            </div>
                        </section>
                        <div class="flex justify-end">
                            button(
                                attrs: attributes! { type="submit" },
                                icon(data: iconify_icon!("feather:save"), attrs: attributes! { class="size-4" })
                                "Save and deploy"
                            )
                        </div>
                    </form>
                )
            )

            card(
                card_header(
                    card_title("Delete component")
                    card_description("Remove the desired state and prune its runtime resources.")
                )
                card_content(
                    <form action=(environment_action) method="get">
                        button(
                            variant: ButtonVariant::Destructive,
                            attrs: attributes! { type="submit" },
                            icon(data: iconify_icon!("feather:trash-2"), attrs: attributes! { class="size-4" })
                            "Delete component"
                        )
                    </form>
                )
            )
        </div>
    }
}

#[page(
    "/tenants/{tenant}/projects/{project}/environments/{environment}/components/{component}/changes"
)]
async fn component_changes(cx: &Cx) -> Result {
    let component = current_component(cx)?;
    let action = uri(cx).path();

    view! {
        <div class="space-y-6">
            card(
                card_header(
                    card_title("Deploy latest revision")
                    card_description("Build the tracked branch and update this environment.")
                )
                card_content(
                    <dl class="mb-5 grid gap-4 text-sm sm:grid-cols-2">
                        <div>
                            <dt class="text-muted-foreground">"Source"</dt>
                            <dd class="mt-1 font-mono">(setting_value(component, "Source"))</dd>
                        </div>
                        <div>
                            <dt class="text-muted-foreground">"Branch"</dt>
                            <dd class="mt-1 font-mono">(setting_value(component, "Branch"))</dd>
                        </div>
                    </dl>
                    <form action=(action) method="get">
                        button(
                            attrs: attributes! { type="submit" },
                            icon(data: iconify_icon!("feather:upload-cloud"), attrs: attributes! { class="size-4" })
                            "Deploy now"
                        )
                    </form>
                )
            )

            card(
                card_header(
                    card_title("Release history")
                    card_description("Git-backed deployments for this component.")
                )
                card_content(
                    if component.changes.is_empty() {
                        <p class="text-sm text-muted-foreground">"No releases recorded."</p>
                    } else {
                        change_list(changes: component.changes)
                    }
                )
            )
        </div>
    }
}

#[page("/tenants/{tenant}/settings")]
async fn tenant_settings(cx: &Cx) -> Result {
    let tenant = mock::tenant(path_param::<Tenant>(cx)).ok_or_not_found()?;
    let action = uri(cx).path();

    view! {
        <main class="mx-auto max-w-6xl px-4 py-8 sm:px-6">
            <nav class="mb-4 text-sm text-muted-foreground" aria-label="Breadcrumb">
                <a href=(tenant_path(tenant.slug)) class="hover:underline">"Home"</a>
                " / "
                <span aria-current="page">"Settings"</span>
            </nav>
            <header class="mb-6">
                <p class="text-sm text-muted-foreground">"Tenant"</p>
                <h1 class="text-2xl font-semibold">"Settings"</h1>
            </header>
            <div class="grid gap-6 lg:grid-cols-2">
                card(
                    card_header(
                        card_title("General")
                        card_description("Tenant identity and source of truth.")
                    )
                    card_content(
                        <form action=(action) method="get" class="space-y-4">
                            <div class="space-y-2">
                                label(attrs: attributes! { for="tenant-display-name" }, "Display name")
                                input(attrs: attributes! { id="tenant-display-name" name="display-name" value=(tenant.name) })
                            </div>
                            <div class="space-y-2">
                                label(attrs: attributes! { for="tenant-current-slug" }, "Slug")
                                input(attrs: attributes! { id="tenant-current-slug" value=(tenant.slug) disabled=(true) })
                            </div>
                            <div class="flex justify-end">
                                button(attrs: attributes! { type="submit" }, "Save changes")
                            </div>
                        </form>
                    )
                )
                card(
                    card_header(
                        card_title("Domains")
                        card_description("Domains registered for this tenant.")
                    )
                    card_content(
                        <ul class="divide-y divide-border text-sm">
                            for domain in tenant.domains {
                                <li class="flex items-center justify-between gap-4 py-3 first:pt-0">
                                    <code>(domain)</code>
                                    badge(variant: BadgeVariant::Outline, "Configured")
                                </li>
                            }
                        </ul>
                        <form action=(action) method="get" class="mt-5 space-y-3">
                            <div class="space-y-2">
                                label(attrs: attributes! { for="new-domain" }, "Add domain")
                                input(attrs: attributes! { id="new-domain" name="domain" placeholder="apps.example.com" })
                            </div>
                            <div class="flex justify-end">
                                button(
                                    attrs: attributes! { type="submit" },
                                    icon(data: iconify_icon!("feather:plus"), attrs: attributes! { class="size-4" })
                                    "Add domain"
                                )
                            </div>
                        </form>
                    )
                )
            </div>
        </main>
    }
}

fn setting_value(component: &'static mock::Component, setting_label: &str) -> &'static str {
    component
        .settings
        .iter()
        .find(|setting| setting.label == setting_label)
        .map_or("", |setting| setting.value)
}

fn current_component(cx: &Cx) -> Result<&'static mock::Component> {
    mock::component(
        path_param::<Tenant>(cx),
        path_param::<Project>(cx),
        path_param::<Environment>(cx),
        path_param::<Component>(cx),
    )
    .ok_or_not_found()
    .map_err(Into::into)
}

fn tenant_path(tenant: &str) -> String {
    format!("/tenants/{tenant}")
}

fn project_path(tenant: &str, project: &str) -> String {
    format!("{}/projects/{project}", tenant_path(tenant))
}

fn environment_path(tenant: &str, project: &str, environment: &str) -> String {
    format!(
        "{}/environments/{environment}",
        project_path(tenant, project)
    )
}

fn component_path(tenant: &str, project: &str, environment: &str, component: &str) -> String {
    format!(
        "{}/components/{component}",
        environment_path(tenant, project, environment)
    )
}

#[topcoat::view::component]
async fn change_list(changes: &'static [mock::Change]) -> Result {
    view! {
        <ol class="divide-y divide-border">
            for change in changes {
                <li class="py-3 first:pt-0 last:pb-0">
                    <article>
                        <h4 class="text-sm font-medium">(change.summary)</h4>
                        <p class="mt-1 text-xs text-muted-foreground">
                            <code>(change.sha)</code>
                            " · "
                            (change.author)
                            " · "
                            <time>(change.time)</time>
                        </p>
                    </article>
                </li>
            }
        </ol>
    }
}
