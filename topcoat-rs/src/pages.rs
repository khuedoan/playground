use crate::mock;
use topcoat::{
    Result,
    context::Cx,
    icon::{icon, iconify::iconify_icon},
    router::{
        RouterErrorExt, Slot, layout, page, path_param, query_params, redirect, redirect_permanent,
        uri,
    },
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

#[path_param]
struct Volume(str);

#[allow(dead_code)]
#[query_params(error = redirect("?"))]
struct UiQuery {
    action: Option<String>,
    auto_deploy: Option<String>,
    backup_policy: Option<String>,
    backups: Option<String>,
    component: Option<String>,
    command: Option<String>,
    compute: Option<String>,
    cron: Option<String>,
    delete_data: Option<String>,
    description: Option<String>,
    display_name: Option<String>,
    domain: Option<String>,
    error: Option<String>,
    exposure: Option<String>,
    eviction: Option<String>,
    high_availability: Option<String>,
    has_storage: Option<String>,
    kind: Option<String>,
    memory: Option<String>,
    mount_path: Option<String>,
    name: Option<String>,
    notice: Option<String>,
    port: Option<String>,
    persistence: Option<String>,
    plan: Option<String>,
    q: Option<String>,
    region: Option<String>,
    replicas: Option<String>,
    size: Option<String>,
    slug: Option<String>,
    source: Option<String>,
    source_kind: Option<String>,
    storage: Option<String>,
    storage_enabled: Option<String>,
    restore_storage: Option<String>,
    timezone: Option<String>,
    variables: Option<String>,
    version: Option<String>,
    volume: Option<String>,
}

#[page("/")]
async fn index() -> Result {
    Err(redirect_permanent("/login").into())
}

#[page("/login")]
async fn login() -> Result {
    view! {
        <main class="grid min-h-screen place-items-center px-4 py-12">
            <div class="card bg-base-100 w-full max-w-md"><div class="card-body items-center text-center"><span class="mb-2">
                        icon(data: iconify_icon!("feather:hexagon"), attrs: attributes! { class="size-10" })
                    </span>
                    <h3 class="card-title">"Welcome to Netamos"</h3>
                    <p>"Deploy and operate applications on your own infrastructure."</p><form action="/tenants/khuedoan" method="get" class="w-full">
                        <button class="btn btn-primary w-full" type="submit">icon(data: iconify_icon!("feather:log-in"), attrs: attributes! { class="size-4" })
                            "Enter product demo"</button>
                    </form>
                    <p class="mt-4 text-center text-xs">
                        "This prototype uses representative sample workloads."
                    </p></div></div>
        </main>
    }
}

#[page("/new-tenant")]
async fn create_tenant() -> Result {
    Err(redirect_permanent("/tenants/khuedoan").into())
}

#[page("/tenants/{tenant}")]
async fn tenant_overview(cx: &Cx) -> Result {
    let tenant_slug = path_param::<Tenant>(cx);
    mock::tenant(tenant_slug).ok_or_not_found()?;
    let query = query_params::<UiQuery>(cx)?;
    let mutation_error = if query.action.as_deref() == Some("create-project") {
        match mock::create_project(
            tenant_slug,
            query.name.as_deref().unwrap_or(""),
            query.description.as_deref().unwrap_or(""),
        ) {
            Ok(project) => {
                let destination = format!(
                    "{}?notice=project-created",
                    project_path(tenant_slug, project.slug),
                );
                return Err(redirect(&destination).into());
            }
            Err(error) => Some(error),
        }
    } else {
        None
    };
    let tenant = mock::tenant(tenant_slug).ok_or_not_found()?;
    let feedback_is_error = mutation_error.is_some();
    let feedback = mutation_error.or_else(|| feedback_message(query));
    let project_filter = query.q.as_deref().unwrap_or("").trim().to_lowercase();
    let created_project: Option<&str> = None;
    let created_project_search = format!(
        "{} provisioning",
        query.description.as_deref().unwrap_or("")
    )
    .to_lowercase();
    let created_project_matches = created_project.is_some()
        && (project_filter.is_empty() || created_project_search.contains(&project_filter));
    let visible_project_count = tenant
        .projects
        .iter()
        .filter(|project| project_matches_filter(project, &project_filter))
        .count()
        + usize::from(created_project_matches);
    let tenant_is_empty =
        tenant.projects.is_empty() && created_project.is_none() && project_filter.is_empty();

    view! {
        <main class="p-6">
            <header class="mb-7 flex flex-wrap items-end justify-between gap-4">
                <h1 class="text-2xl font-semibold tracking-tight">"Projects"</h1>
                if !tenant_is_empty {
                    <div class="flex w-full items-center gap-2 sm:w-auto">
                        <form action=(tenant_path(tenant.slug)) method="get" class="min-w-0 flex-1 sm:w-64">
                            <label class="relative block">
                                <span class="sr-only">"Filter projects"</span>
                                icon(data: iconify_icon!("feather:search"), attrs: attributes! { class="pointer-events-none absolute top-1/2 left-3 size-3.5 -translate-y-1/2" })
                                <input class="input w-full" name="q"
                                    value=(query.q.as_deref().unwrap_or(""))
                                    placeholder="Filter projects…"
                                    data-project-filter="">
                            </label>
                            <button type="submit" class="sr-only">"Filter projects"</button>
                        </form>
                        <a
                            href=(format!("{}/projects/new", tenant_path(tenant.slug)))
                            class="btn btn-primary"
                        >
                            icon(data: iconify_icon!("feather:plus"), attrs: attributes! { class="size-4" })
                            "New project"
                        </a>
                    </div>
                }
            </header>

            feedback_banner(message: feedback, is_error: feedback_is_error)

            <section class="card bg-base-100" aria-labelledby="projects-heading">
                <h2 id="projects-heading" class="sr-only">"Project list"</h2>
                <div class="hidden grid-cols-[minmax(0,2fr)_minmax(9rem,1fr)_8rem_7rem_7rem_1rem] gap-4 px-4 py-2.5 text-xs font-medium sm:grid">
                    <span>"Project"</span>
                    <span>"Environment"</span>
                    <span>"Region"</span>
                    <span>"Components"</span>
                    <span>"Status"</span>
                    <span class="sr-only">"Open"</span>
                </div>
                <div>
                    match created_project {
                        Some(project_name) => <article
                            data-project-card=""
                            data-project-search=(created_project_search)
                            hidden=(!created_project_matches)
                            class="grid gap-2 px-4 py-3.5 sm:grid-cols-[minmax(0,2fr)_minmax(9rem,1fr)_8rem_7rem_7rem_1rem] sm:items-center sm:gap-4"
                        >
                            <span class="min-w-0">
                                <span class="block truncate text-sm font-medium">(project_name)</span>
                                <span class="mt-0.5 block truncate text-xs">
                                    (query.description.as_deref().unwrap_or("New project"))
                                </span>
                            </span>
                            <span class="flex flex-wrap items-center gap-x-3 text-xs sm:hidden">
                                <span>"Production"</span>
                                <span>"Region pending"</span>
                                <span>"0 components"</span>
                            </span>
                            <span class="hidden text-xs sm:block">
                                "Production"
                            </span>
                            <span class="hidden text-xs sm:block">
                                "—"
                            </span>
                            <span class="hidden text-xs sm:block">
                                "0"
                            </span>
                            <span class="flex items-center gap-2 text-xs">
                                <span class="status status-warning"></span>
                                "Provisioning"
                            </span>
                            icon(data: iconify_icon!("feather:loader"), attrs: attributes! { class="hidden size-3.5 animate-spin sm:block" })
                        </article>,
                        None => "",
                    }
                    for project in tenant.projects {
                        let component_count = project
                            .environments
                            .iter()
                            .map(|environment| environment.components.len())
                            .sum::<usize>();
                        let healthy_count = project
                            .environments
                            .iter()
                            .flat_map(|environment| environment.components)
                            .filter(|component| {
                                component
                                    .observability
                                    .is_some_and(|observability| observability.health == "Healthy")
                            })
                            .count();
                        let project_healthy =
                            component_count > 0 && healthy_count == component_count;
                        let state_label = if project_healthy {
                            "Healthy"
                        } else if component_count == 0 {
                            "Pending"
                        } else {
                            "Attention"
                        };
                        let environment_label = project
                            .environments
                            .first()
                            .map(|environment| {
                                if project.environments.len() > 1 {
                                    format!("{} +{}", environment.name, project.environments.len() - 1)
                                } else {
                                    environment.name.to_owned()
                                }
                            })
                            .unwrap_or_else(|| "None".to_owned());
                        let region = project
                            .environments
                            .first()
                            .map_or("—", |environment| environment.region);
                        let search_text = project_search_text(project);
                        let matches_filter = project_matches_filter(project, &project_filter);

                        <a
                            href=(project_path(tenant.slug, project.slug))
                            data-project-card=""
                            data-project-search=(search_text)
                            hidden=(!matches_filter)
                            class="group grid gap-2 px-4 py-3.5 sm:grid-cols-[minmax(0,2fr)_minmax(9rem,1fr)_8rem_7rem_7rem_1rem] sm:items-center sm:gap-4"
                        >
                            <span class="min-w-0">
                                <span class="block truncate text-sm font-medium">(project.name)</span>
                                <span class="mt-0.5 block truncate text-xs">(project.description)</span>
                            </span>
                            <span class="flex flex-wrap items-center gap-x-3 text-xs sm:hidden">
                                <span>(environment_label.clone())</span>
                                <span>(region)</span>
                                <span>(count_label(component_count, "component", "components"))</span>
                            </span>
                            <span class="hidden min-w-0 text-xs sm:block">
                                <span class="truncate">(environment_label)</span>
                                <span class="mt-0.5 hidden text-xs sm:block">
                                    (count_label(project.environments.len(), "environment", "environments"))
                                </span>
                            </span>
                            <span class="hidden text-xs sm:block">
                                (region)
                            </span>
                            <span class="hidden text-xs sm:block">
                                (component_count)
                            </span>
                            <span class="flex items-center gap-2 text-xs">
                                <span class=(if project_healthy { "status status-success" } else { "status status-warning" })></span>
                                (state_label)
                            </span>
                            icon(data: iconify_icon!("feather:chevron-right"), attrs: attributes! { class="hidden size-3.5 sm:block" })
                        </a>
                    }
                    <div
                        data-project-empty=""
                        hidden=(visible_project_count != 0)
                        class="px-6 py-10 text-center"
                    >
                        <h3 class="text-sm font-semibold">
                            if tenant_is_empty { "No projects yet" } else { "No matching projects" }
                        </h3>
                        <p class="mt-1 text-xs">
                            if tenant_is_empty {
                                "Create the first project for this tenant."
                            } else {
                                "Try a project, environment, or component name."
                            }
                        </p>
                        if tenant_is_empty {
                            <a
                                href=(format!("{}/projects/new", tenant_path(tenant.slug)))
                                class="btn btn-outline btn-sm"
                            >
                                icon(data: iconify_icon!("feather:plus"), attrs: attributes! { class="size-3.5" })
                                "Create project"
                            </a>
                        }
                    </div>
                </div>
            </section>

        </main>
    }
}

#[page("/tenants/{tenant}/activity")]
async fn legacy_tenant_activity(cx: &Cx) -> Result {
    let target = format!("{}/changes", tenant_path(path_param::<Tenant>(cx)),);

    Err(redirect_permanent(&target).into())
}

#[page("/tenants/{tenant}/changes")]
async fn tenant_changes(cx: &Cx) -> Result {
    let tenant = mock::tenant(path_param::<Tenant>(cx)).ok_or_not_found()?;

    view! {
        <main class="p-6">
            <header class="mb-7">
                <h1 id="changes-heading" class="text-2xl font-semibold tracking-tight">"Changes"</h1>
                <p class="mt-1 text-sm">"GitOps changes across every project."</p>
            </header>

            <section class="card bg-base-100" aria-labelledby="changes-heading">
                if tenant.changes.is_empty() {
                    <div class="px-5 py-10 text-center">
                        <p class="text-sm font-medium">"No changes yet"</p>
                        <p class="mt-1 text-xs">"Changes appear here after GitOps reconciliation."</p>
                    </div>
                } else {
                    <ol>
                        for change in tenant.changes {
                            <li>
                                <a
                                    href=(change_destination(tenant.slug, change))
                                    class="group flex h-11 min-w-0 items-center gap-3 px-4"
                                >
                                    <span class="min-w-0 flex-1 truncate text-sm font-medium">
                                        (change.summary)
                                    </span>
                                    <code class="hidden w-16 shrink-0 text-xs sm:block">
                                        (change.sha)
                                    </code>
                                    <span class="hidden w-64 shrink-0 truncate text-xs lg:block">
                                        (change.target.project_slug)
                                        " / "
                                        (change.target.environment_slug)
                                        " / "
                                        (change.target.component_slug)
                                    </span>
                                    <span class="hidden w-28 shrink-0 truncate text-xs xl:block">
                                        (change.author)
                                    </span>
                                    <time class="w-24 shrink-0 text-right text-xs">
                                        (change.time)
                                    </time>
                                    icon(data: iconify_icon!("feather:chevron-right"), attrs: attributes! {
                                        aria-hidden="true"
                                        class="size-3.5 shrink-0"
                                    })
                                </a>
                            </li>
                        }
                    </ol>
                }
            </section>
        </main>
    }
}

#[page("/tenants/{tenant}/usage")]
async fn tenant_usage(cx: &Cx) -> Result {
    let tenant = mock::tenant(path_param::<Tenant>(cx)).ok_or_not_found()?;
    let usage = tenant_usage_totals(tenant);
    let compute_quota = 2_000.0;
    let memory_quota = 8_000.0;
    let egress_quota = 500.0;

    view! {
        <main class="p-6">
            <header class="mb-7 flex flex-wrap items-start justify-between gap-4">
                <div>
                    <h1 class="text-2xl font-semibold tracking-tight">"Usage"</h1>
                    <p class="mt-1 text-sm">
                        "Sample metering for July 1–31, 2026 · updated July 27 at 10:45 ICT"
                    </p>
                    <p class="mt-1 text-xs">"Preview data, not an invoice."</p>
                </div>
                <div class="px-3 py-2 text-right">
                    <p class="text-xs">"Plan"</p>
                    <p class="text-sm font-medium">"Team preview"</p>
                </div>
            </header>

            <dl class="stats stats-vertical sm:stats-horizontal mb-6 w-full">
                <div class="stat">
                    <dt class="stat-title">"Compute"</dt>
                    <dd class="stat-value">(format!("{:.1}", usage.0))</dd>
                    <dd class="stat-desc">(format!("of {compute_quota:.0} vCPU hours"))</dd>
                    <progress class="progress progress-primary w-full" value=(usage.0 / compute_quota * 100.0) max=(100.0) aria-label="Compute quota used"></progress>
                </div>
                <div class="stat">
                    <dt class="stat-title">"Memory"</dt>
                    <dd class="stat-value">(format!("{:.1}", usage.1))</dd>
                    <dd class="stat-desc">(format!("of {memory_quota:.0} GiB hours"))</dd>
                    <progress class="progress progress-primary w-full" value=(usage.1 / memory_quota * 100.0) max=(100.0) aria-label="Memory quota used"></progress>
                </div>
                <div class="stat">
                    <dt class="stat-title">"Egress"</dt>
                    <dd class="stat-value">(format!("{:.1}", usage.2))</dd>
                    <dd class="stat-desc">(format!("of {egress_quota:.0} GB"))</dd>
                    <progress class="progress progress-primary w-full" value=(usage.2 / egress_quota * 100.0) max=(100.0) aria-label="Egress quota used"></progress>
                </div>
            </dl>

            <section class="card bg-base-100" aria-labelledby="usage-projects-heading">
                <header class="px-5 py-4">
                    <h2 id="usage-projects-heading" class="text-sm font-semibold">"Usage by project"</h2>
                </header>
                <div class="hidden grid-cols-[minmax(0,1fr)_9rem_9rem_8rem_1rem] gap-4 px-5 py-3 text-xs font-medium sm:grid">
                    <span>"Project"</span>
                    <span class="text-right">"Compute"</span>
                    <span class="text-right">"Memory"</span>
                    <span class="text-right">"Egress"</span>
                    <span class="sr-only">"Open"</span>
                </div>
                <div>
                    if tenant.projects.is_empty() {
                        <div class="px-5 py-10 text-center text-sm">
                            "No metered project usage yet."
                        </div>
                    } else {
                        for project in tenant.projects {
                            <a
                                href=(project_path(tenant.slug, project.slug))
                                class="group grid grid-cols-3 gap-3 px-5 py-3.5 sm:grid-cols-[minmax(0,1fr)_9rem_9rem_8rem_1rem] sm:items-center sm:gap-4"
                            >
                                <span class="col-span-3 text-sm font-medium sm:col-span-1">(project.name)</span>
                                <span class="font-mono text-xs sm:text-right">
                                    <span class="mb-0.5 block sm:hidden">"Compute"</span>
                                    (format!("{:.1} vCPUh", project.usage.compute_vcpu_hours))
                                </span>
                                <span class="font-mono text-xs sm:text-right">
                                    <span class="mb-0.5 block sm:hidden">"Memory"</span>
                                    (format!("{:.1} GiBh", project.usage.memory_gib_hours))
                                </span>
                                <span class="font-mono text-xs sm:text-right">
                                    <span class="mb-0.5 block sm:hidden">"Egress"</span>
                                    (format!("{:.1} GB", project.usage.egress_gb))
                                </span>
                                icon(data: iconify_icon!("feather:chevron-right"), attrs: attributes! { class="hidden size-3.5 sm:block" })
                            </a>
                        }
                    }
                </div>
            </section>
        </main>
    }
}

#[page("/tenants/{tenant}/projects/new")]
async fn create_project(cx: &Cx) -> Result {
    let tenant_slug = path_param::<Tenant>(cx);
    mock::tenant(tenant_slug).ok_or_not_found()?;
    let action = tenant_path(tenant_slug);

    view! {
        <main class="mx-auto max-w-2xl p-6">
            <nav class="mb-5 flex flex-wrap items-center gap-1.5 text-xs" aria-label="Breadcrumb">
                <a href=(tenant_path(tenant_slug))>"Projects"</a>
                icon(data: iconify_icon!("feather:chevron-right"), attrs: attributes! { class="size-3" })
                <span aria-current="page">"New project"</span>
            </nav>
            <header class="mb-6">
                <h1 class="text-2xl font-semibold">"Create project"</h1>
                <p class="mt-1 text-sm">"A production environment is created automatically."</p>
            </header>
            <div class="card bg-base-100"><div class="card-body"><h3 class="card-title">"Project details"</h3><form action=(action) method="get" class="space-y-5">
                        <input type="hidden" name="action" value="create-project">
                        <div class="space-y-2">
                            <label class="label" for="project-name">"Name"</label>
                            <input class="input w-full" id="project-name"
                                name="name"
                                placeholder="Customer portal"
                                required=(true)>
                        </div>
                        <div class="space-y-2">
                            <label class="label" for="project-description">"Description"</label>
                            <textarea class="textarea w-full" id="project-description"
                                name="description"
                                placeholder="What this project contains"></textarea>
                        </div>
                        <div class="flex justify-end gap-2">
                            <a href=(tenant_path(tenant_slug)) class="btn btn-outline">"Cancel"</a>
                            <button class="btn btn-primary" type="submit">"Create project"</button>
                        </div>
                    </form></div></div>
        </main>
    }
}

#[page("/tenants/{tenant}/projects/{project}")]
async fn project_overview(cx: &Cx) -> Result {
    let tenant_slug = path_param::<Tenant>(cx);
    let project_slug = path_param::<Project>(cx);
    mock::project(tenant_slug, project_slug).ok_or_not_found()?;
    let query = query_params::<UiQuery>(cx)?;
    let mutation_error = if query.action.as_deref() == Some("create-environment") {
        match mock::create_environment(
            tenant_slug,
            project_slug,
            query.name.as_deref().unwrap_or(""),
            region_label(query.region.as_deref()),
        ) {
            Ok(environment) => {
                let destination = format!(
                    "{}?notice=environment-created",
                    environment_path(tenant_slug, project_slug, environment.slug),
                );
                return Err(redirect(&destination).into());
            }
            Err(error) => Some(error),
        }
    } else {
        None
    };
    let tenant = mock::tenant(tenant_slug).ok_or_not_found()?;
    let project = mock::project(tenant_slug, project_slug).ok_or_not_found()?;
    let feedback_is_error = mutation_error.is_some();
    let feedback = mutation_error.or_else(|| feedback_message(query));
    let created_environment: Option<&str> = None;
    let environment_count = project.environments.len() + usize::from(created_environment.is_some());
    let component_count = project
        .environments
        .iter()
        .map(|environment| environment.components.len())
        .sum::<usize>();

    view! {
        <main class="p-6">
            <nav class="mb-5 flex items-center gap-1.5 text-xs" aria-label="Breadcrumb">
                <a href=(tenant_path(tenant_slug))>"Projects"</a>
                <span class="inline-flex min-w-0 items-center gap-1.5">
                    breadcrumb_separator()
                    project_context_selector(
                        tenant: tenant,
                        project: project,
                        current_environment_slug: None,
                    )
                </span>
            </nav>
            <header class="mb-7 flex flex-wrap items-start justify-between gap-4">
                <div>
                    <h1 class="text-2xl font-semibold tracking-tight">(project.name)</h1>
                    <p class="mt-1 text-sm">(project.description)</p>
                    <p class="mt-2 flex flex-wrap gap-x-3 text-xs">
                        <span>(count_label(environment_count, "environment", "environments"))</span>
                        <span>(count_label(component_count, "component", "components"))</span>
                    </p>
                </div>
                <a
                    href=(format!("{}/environments/new", project_path(tenant_slug, project.slug)))
                    class="btn btn-primary"
                >
                    icon(data: iconify_icon!("feather:plus"), attrs: attributes! { class="size-4" })
                    "New environment"
                </a>
            </header>

            feedback_banner(message: feedback, is_error: feedback_is_error)

            <section class="card bg-base-100" aria-labelledby="environments-heading">
                <h2 id="environments-heading" class="sr-only">"Environment list"</h2>
                <div class="hidden grid-cols-[minmax(10rem,1.5fr)_8rem_7rem_8rem_minmax(10rem,1fr)_1rem] gap-4 px-4 py-2.5 text-xs font-medium md:grid">
                    <span>"Environment"</span>
                    <span>"Region"</span>
                    <span>"Components"</span>
                    <span>"Health"</span>
                    <span>"Latest change"</span>
                    <span class="sr-only">"Open"</span>
                </div>

                <div>
                    match created_environment {
                        Some(environment_name) => <article class="grid gap-2 px-4 py-3.5 md:grid-cols-[minmax(10rem,1.5fr)_8rem_7rem_8rem_minmax(10rem,1fr)_1rem] md:items-center md:gap-4">
                            <span class="min-w-0">
                                <span class="block truncate text-sm font-medium">(environment_name)</span>
                                <span class="mt-0.5 block text-xs">"Creating environment"</span>
                            </span>
                            <span class="text-xs md:hidden">
                                (region_label(query.region.as_deref()))
                            </span>
                            <span class="hidden text-xs md:block">
                                (region_label(query.region.as_deref()))
                            </span>
                            <span class="hidden text-xs md:block">
                                "0"
                            </span>
                            <span class="flex items-center gap-2 text-xs">
                                <span class="status status-warning"></span>
                                "Provisioning"
                            </span>
                            <span class="hidden text-xs md:block">
                                "Queued"
                            </span>
                            icon(data: iconify_icon!("feather:loader"), attrs: attributes! { class="hidden size-3.5 animate-spin md:block" })
                        </article>,
                        None => "",
                    }
                    for environment in project.environments {
                        let healthy_components = environment
                            .components
                            .iter()
                            .filter(|component| {
                                component
                                    .observability
                                    .is_some_and(|observability| observability.health == "Healthy")
                            })
                            .count();
                        let all_healthy = healthy_components == environment.components.len()
                            && !environment.components.is_empty();
                        let latest_change = environment
                            .components
                            .iter()
                            .flat_map(|component| component.changes)
                            .next();
                        <a
                            href=(environment_path(tenant_slug, project.slug, environment.slug))
                            class="group grid gap-2 px-4 py-3.5 md:grid-cols-[minmax(10rem,1.5fr)_8rem_7rem_8rem_minmax(10rem,1fr)_1rem] md:items-center md:gap-4"
                        >
                            <span class="min-w-0">
                                <span class="block truncate text-sm font-medium">(environment.name)</span>
                                <span class="mt-0.5 block text-xs">
                                    (count_label(environment.components.len(), "component", "components"))
                                </span>
                            </span>
                            <span class="text-xs md:hidden">
                                (environment.region)
                            </span>
                            <span class="hidden text-xs md:block">
                                (environment.region)
                            </span>
                            <span class="hidden text-xs md:block">
                                (environment.components.len())
                            </span>
                            <span class="flex items-center gap-2 text-xs">
                                <span class=(if all_healthy { "status status-success" } else { "status status-warning" })></span>
                                if all_healthy { "Healthy" } else if healthy_components == 0 { "Pending" } else { "Attention" }
                            </span>
                            <span class="hidden min-w-0 text-xs md:block">
                                match latest_change {
                                    Some(change) => <span class="flex items-center gap-2">
                                        <code>(change.sha)</code>
                                        <span>(change.time)</span>
                                    </span>,
                                    None => <span>"No changes"</span>,
                                }
                            </span>
                            icon(data: iconify_icon!("feather:chevron-right"), attrs: attributes! { class="hidden size-3.5 md:block" })
                        </a>
                    }
                    if environment_count == 0 {
                        <div class="px-5 py-10 text-center">
                            <p class="text-sm font-medium">"No environments yet"</p>
                            <p class="mt-1 text-xs">"Create an environment to add components."</p>
                        </div>
                    }
                </div>
            </section>
        </main>
    }
}

#[page("/tenants/{tenant}/projects/{project}/environments/new")]
async fn create_environment(cx: &Cx) -> Result {
    let tenant_slug = path_param::<Tenant>(cx);
    let project_slug = path_param::<Project>(cx);
    let project = mock::project(tenant_slug, project_slug).ok_or_not_found()?;
    let action = project_path(tenant_slug, project_slug);
    let default_region = project
        .environments
        .first()
        .map_or("Helsinki", |environment| environment.region);

    view! {
        <main class="mx-auto max-w-2xl p-6">
            <nav class="mb-5 flex items-center gap-1.5 text-xs" aria-label="Breadcrumb">
                <a href=(tenant_path(tenant_slug))>"Projects"</a>
                icon(data: iconify_icon!("feather:chevron-right"), attrs: attributes! { class="size-3" })
                <a href=(project_path(tenant_slug, project_slug))>(project.name)</a>
                icon(data: iconify_icon!("feather:chevron-right"), attrs: attributes! { class="size-3" })
                <span aria-current="page">"New environment"</span>
            </nav>
            <header class="mb-6">
                <h1 class="text-2xl font-semibold">"Create environment"</h1>
                <p class="mt-1 text-sm">"Create an isolated target for this project's desired state."</p>
            </header>
            <div class="card bg-base-100"><div class="card-body"><h3 class="card-title">"Environment details"</h3><form action=(action) method="get" class="space-y-5">
                        <input type="hidden" name="action" value="create-environment">
                        <div class="space-y-2">
                            <label class="label" for="environment-name">"Name"</label>
                            <input class="input w-full" id="environment-name"
                                name="name"
                                placeholder="Staging"
                                required=(true)
                                data-environment-name=""
                                data-existing-environments=(project
                                    .environments
                                    .iter()
                                    .map(|environment| environment.slug)
                                    .collect::<Vec<_>>()
                                    .join(","))>
                            <p class="text-xs">"Must be unique within this project."</p>
                        </div>
                        <div class="space-y-2">
                            <label class="label" for="environment-region">"Region"</label>
                            <select class="select w-full" id="environment-region" name="region"><option value="helsinki" selected=(default_region == "Helsinki")>"Helsinki"</option>
                                <option value="saigon" selected=(default_region == "Saigon")>"Saigon"</option></select>
                        </div>
                        <div class="flex justify-end gap-2">
                            <a href=(project_path(tenant_slug, project_slug)) class="btn btn-outline">"Cancel"</a>
                            <button class="btn btn-primary" type="submit">"Create environment"</button>
                        </div>
                    </form></div></div>
        </main>
    }
}

#[page("/tenants/{tenant}/projects/{project}/environments/{environment}")]
async fn environment_overview(cx: &Cx) -> Result {
    let tenant_slug = path_param::<Tenant>(cx);
    let project_slug = path_param::<Project>(cx);
    let environment_slug = path_param::<Environment>(cx);
    let initial_tenant = mock::tenant(tenant_slug).ok_or_not_found()?;
    let initial_environment =
        mock::environment(tenant_slug, project_slug, environment_slug).ok_or_not_found()?;
    let query = query_params::<UiQuery>(cx)?;
    let mutation_error = match query.action.as_deref() {
        Some("create-component") => match mock::create_component(
            tenant_slug,
            project_slug,
            environment_slug,
            new_component_from_query(query, initial_tenant),
        ) {
            Ok(component) => {
                let destination = format!(
                    "{}?notice=component-created",
                    component_path(tenant_slug, project_slug, environment_slug, component.slug,),
                );
                return Err(redirect(&destination).into());
            }
            Err(error) => Some(error),
        },
        Some("delete-component") => {
            let requested_component = query.component.as_deref().unwrap_or("");
            let component_slug = initial_environment
                .components
                .iter()
                .find(|component| {
                    component.slug == requested_component || component.name == requested_component
                })
                .map(|component| component.slug)
                .unwrap_or(requested_component);
            match mock::delete_component(
                tenant_slug,
                project_slug,
                environment_slug,
                component_slug,
            ) {
                Ok(()) => {
                    let destination = format!(
                        "{}?notice=component-deleted",
                        environment_path(tenant_slug, project_slug, environment_slug),
                    );
                    return Err(redirect(&destination).into());
                }
                Err(error) => Some(error),
            }
        }
        Some("delete-retained-storage") => {
            match mock::remove_volume(
                tenant_slug,
                project_slug,
                environment_slug,
                query.volume.as_deref().unwrap_or(""),
            ) {
                Ok(()) => {
                    let destination = format!(
                        "{}?notice=storage-deleted",
                        environment_path(tenant_slug, project_slug, environment_slug),
                    );
                    return Err(redirect(&destination).into());
                }
                Err(error) => Some(error),
            }
        }
        _ => None,
    };
    let tenant = mock::tenant(tenant_slug).ok_or_not_found()?;
    let project = mock::project(tenant_slug, project_slug).ok_or_not_found()?;
    let environment =
        mock::environment(tenant_slug, project_slug, environment_slug).ok_or_not_found()?;
    let feedback_is_error = mutation_error.is_some();
    let feedback = mutation_error.or_else(|| feedback_message(query));
    let deleted_component: Option<&str> = None;
    let created_component: Option<&str> = None;
    let retained_storage: Option<&mock::Volume> = None;
    let visible_component_count = environment
        .components
        .iter()
        .filter(|component| Some(component.name) != deleted_component)
        .count()
        + usize::from(created_component.is_some());
    let healthy_count = environment
        .components
        .iter()
        .filter(|component| {
            Some(component.name) != deleted_component
                && component
                    .observability
                    .is_some_and(|observability| observability.health == "Healthy")
        })
        .count();
    let environment_healthy =
        visible_component_count > 0 && healthy_count == visible_component_count;
    let change_count = environment
        .components
        .iter()
        .filter(|component| Some(component.name) != deleted_component)
        .map(|component| component.changes.len())
        .sum::<usize>();

    view! {
        <main class="p-6">
            <nav class="mb-5 flex flex-wrap items-center gap-1.5 text-xs" aria-label="Breadcrumb">
                <a href=(tenant_path(tenant_slug))>"Projects"</a>
                <span class="inline-flex min-w-0 items-center gap-1.5">
                    breadcrumb_separator()
                    project_context_selector(
                        tenant: tenant,
                        project: project,
                        current_environment_slug: Some(environment.slug),
                    )
                </span>
                <span class="inline-flex min-w-0 items-center gap-1.5">
                    breadcrumb_separator()
                    environment_context_selector(
                        tenant_slug: tenant.slug,
                        project: project,
                        environment: environment,
                        component: None,
                        component_suffix: "",
                    )
                </span>
            </nav>
            <header class="mb-5 flex flex-wrap items-start justify-between gap-4">
                <div>
                    <h1 id="components-heading" class="text-2xl font-semibold tracking-tight">"Components"</h1>
                    <p class="mt-2 flex flex-wrap items-center gap-x-4 gap-y-1 text-xs">
                        <span>(count_label(visible_component_count, "component", "components"))</span>
                        <span>(environment.region)</span>
                        <span class="flex items-center gap-1.5">
                            <span class=(if environment_healthy { "status status-success" } else { "status status-warning" })></span>
                            (format!("{healthy_count}/{visible_component_count} healthy"))
                        </span>
                    </p>
                </div>
                <a
                    href=(format!(
                        "{}/new-component",
                        environment_path(tenant_slug, project_slug, environment.slug),
                    ))
                    class="btn btn-primary"
                >
                    icon(data: iconify_icon!("feather:plus"), attrs: attributes! { class="size-4" })
                    "New component"
                </a>
            </header>

            feedback_banner(message: feedback, is_error: feedback_is_error)

            <div class="space-y-6">
                <section id="components" class="card bg-base-100" aria-labelledby="components-heading">
                    <div class="hidden grid-cols-[minmax(12rem,2fr)_9rem_6rem_6rem_7rem_8rem_1rem] gap-4 px-4 py-2.5 text-xs font-medium md:grid">
                        <span>"Component"</span>
                        <span>"Kind"</span>
                        <span>"CPU"</span>
                        <span>"Memory"</span>
                        <span>"Replicas"</span>
                        <span>"Status"</span>
                        <span class="sr-only">"Open"</span>
                    </div>
                    <div>
                        match created_component {
                            Some(component_name) => <article class="grid gap-2 px-4 py-3.5 md:grid-cols-[minmax(12rem,2fr)_9rem_6rem_6rem_7rem_8rem_1rem] md:items-center md:gap-4">
                                <span class="min-w-0">
                                    <span class="block truncate font-mono text-sm font-medium">(component_name)</span>
                                    <span class="mt-0.5 block truncate text-xs">
                                        "Preparing "
                                        (query.source.as_deref().unwrap_or("configured source"))
                                    </span>
                                </span>
                                <span class="flex flex-wrap items-center gap-x-3 text-xs md:hidden">
                                    <span>(component_kind_label(query.kind.as_deref()))</span>
                                    <span>(format!("{} desired", query.replicas.as_deref().unwrap_or("1")))</span>
                                </span>
                                <span class="hidden text-xs md:block">
                                    (component_kind_label(query.kind.as_deref()))
                                </span>
                                <span class="hidden text-xs md:block">
                                    "—"
                                </span>
                                <span class="hidden text-xs md:block">
                                    "—"
                                </span>
                                <span class="hidden text-xs md:block">
                                    (query.replicas.as_deref().unwrap_or("1"))
                                </span>
                                <span class="flex items-center gap-2 text-xs">
                                    <span class="status status-warning"></span>
                                    "Building"
                                </span>
                                icon(data: iconify_icon!("feather:loader"), attrs: attributes! { class="hidden size-3.5 animate-spin md:block" })
                            </article>,
                            None => "",
                        }
                        for component in environment.components {
                            if Some(component.name) != deleted_component {
                                <a
                                    href=(component_path(
                                        tenant_slug,
                                        project_slug,
                                        environment.slug,
                                        component.slug,
                                    ))
                                    class="group grid gap-2 px-4 py-3.5 md:grid-cols-[minmax(12rem,2fr)_9rem_6rem_6rem_7rem_8rem_1rem] md:items-center md:gap-4"
                                >
                                    <span class="min-w-0 flex-1">
                                        <span class="block truncate font-mono text-sm font-medium">(component.name)</span>
                                        <span class="mt-0.5 block truncate text-xs">(component.summary)</span>
                                    </span>
                                    <span class="flex flex-wrap items-center gap-x-3 text-xs md:hidden">
                                        <span>(component.kind)</span>
                                        match component.observability {
                                            Some(observability) => <span class="contents">
                                                <span>(format!("CPU {}%", observability.cpu_percent))</span>
                                                <span>(format!("Memory {}%", observability.memory_percent))</span>
                                                <span>(observability.replicas)</span>
                                            </span>,
                                            None => <span class="contents">
                                                <span>(format!("{} desired", setting_value(component, "Replicas")))</span>
                                            </span>,
                                        }
                                    </span>
                                    <span class="hidden text-xs md:block">
                                        (component.kind)
                                    </span>
                                    match component.observability {
                                        Some(observability) => <span class="contents">
                                            <span class="hidden text-xs md:block">
                                                (format!("{}%", observability.cpu_percent))
                                            </span>
                                            <span class="hidden text-xs md:block">
                                                (format!("{}%", observability.memory_percent))
                                            </span>
                                            <span class="hidden text-xs md:block">
                                                (observability.replicas.split_whitespace().next().unwrap_or("—"))
                                            </span>
                                            <span class="flex items-center gap-2 text-xs">
                                                <span class="status status-success"></span>
                                                (observability.health)
                                            </span>
                                        </span>,
                                        None => <span class="contents">
                                            <span class="hidden text-xs md:block">
                                                "—"
                                            </span>
                                            <span class="hidden text-xs md:block">
                                                "—"
                                            </span>
                                            <span class="hidden text-xs md:block">
                                                (setting_value(component, "Replicas"))
                                            </span>
                                            <span class="flex items-center gap-2 text-xs">
                                                <span class="status status-warning"></span>
                                                (component.state)
                                            </span>
                                        </span>,
                                    }
                                    icon(data: iconify_icon!("feather:chevron-right"), attrs: attributes! { class="hidden size-3.5 md:block" })
                                </a>
                            }
                        }
                        if visible_component_count == 0 {
                            <div class="px-5 py-10 text-center">
                                <p class="text-sm font-medium">"No components yet"</p>
                                <p class="mt-1 text-xs">"Add a component to this environment."</p>
                            </div>
                        }
                    </div>
                </section>

                if let Some(storage) = retained_storage {
                    <section
                        id="retained-storage"
                        class="card bg-base-100"
                        aria-labelledby="retained-storage-heading"
                    >
                        <header class="px-4 py-3">
                            <h2 id="retained-storage-heading" class="text-sm font-semibold">"Retained data"</h2>
                        </header>
                        <div class="grid gap-3 px-4 py-3.5 sm:grid-cols-[minmax(0,1fr)_7rem_8rem_auto_auto] sm:items-center sm:gap-4">
                            <span class="min-w-0">
                                <span class="block truncate text-sm font-medium">
                                    "Data from "
                                    "component"
                                </span>
                                <span class="mt-0.5 block truncate text-xs">
                                    (storage.binding.as_ref().map_or("/data", |binding| binding.mount_path))
                                </span>
                            </span>
                            <span class="text-xs">(format!("{} GiB", storage.capacity_gib))</span>
                            <span class="text-xs">"Retained now"</span>
                            <a
                                href=(format!(
                                    "{}/new-component?restore_storage={}",
                                    environment_path(tenant_slug, project_slug, environment.slug),
                                    storage.slug,
                                ))
                                class="btn btn-outline btn-sm"
                            >
                                "Use in new component"
                            </a>
                            <form
                                action=(environment_path(tenant_slug, project_slug, environment.slug))
                                method="get"
                                data-confirm="Permanently delete this retained data and its backups?"
                            >
                                <input type="hidden" name="action" value="delete-retained-storage">
                                <input type="hidden" name="volume" value=(storage.slug)>
                                <button class="btn btn-ghost btn-sm" type="submit">"Delete"</button>
                            </form>
                        </div>
                    </section>
                }

                if change_count > 0 {
                    <section id="changes" class="card bg-base-100" aria-labelledby="environment-changes-heading">
                        <header class="flex h-11 items-center justify-between px-4">
                            <h2 id="environment-changes-heading" class="text-sm font-semibold">"Changes"</h2>
                            <span class="text-xs">
                                (count_label(change_count, "change", "changes"))
                            </span>
                        </header>
                        <ol>
                            for component in environment.components {
                                if Some(component.name) != deleted_component {
                                    for change in component.changes {
                                        <li>
                                            <a
                                                href=(format!(
                                                    "{}/changes",
                                                    component_path(
                                                        tenant_slug,
                                                        project_slug,
                                                        environment.slug,
                                                        component.slug,
                                                    ),
                                                ))
                                                class="group flex h-11 min-w-0 items-center gap-3 px-4"
                                            >
                                                <span class="min-w-0 flex-1 truncate text-sm font-medium">
                                                    (change.summary)
                                                </span>
                                                <code class="hidden w-16 shrink-0 text-xs sm:block">
                                                    (change.sha)
                                                </code>
                                                <span class="hidden w-32 shrink-0 truncate font-mono text-xs md:block">
                                                    (component.name)
                                                </span>
                                                <span class="hidden w-28 shrink-0 truncate text-xs lg:block">
                                                    (change.author)
                                                </span>
                                                <time class="w-24 shrink-0 text-right text-xs">
                                                    (change.time)
                                                </time>
                                                icon(data: iconify_icon!("feather:chevron-right"), attrs: attributes! {
                                                    aria-hidden="true"
                                                    class="size-3.5 shrink-0"
                                                })
                                            </a>
                                        </li>
                                    }
                                }
                            }
                        </ol>
                    </section>
                }
            </div>
        </main>
    }
}

#[page("/tenants/{tenant}/projects/{project}/environments/{environment}/volumes/new")]
async fn legacy_create_volume(cx: &Cx) -> Result {
    let target = format!(
        "{}/new-component",
        environment_path(
            path_param::<Tenant>(cx),
            path_param::<Project>(cx),
            path_param::<Environment>(cx),
        ),
    );

    Err(redirect_permanent(&target).into())
}

#[page("/tenants/{tenant}/projects/{project}/environments/{environment}/volumes/{volume}")]
async fn legacy_volume_detail(cx: &Cx) -> Result {
    let tenant_slug = path_param::<Tenant>(cx);
    let project_slug = path_param::<Project>(cx);
    let environment_slug = path_param::<Environment>(cx);
    let volume = mock::volume(
        tenant_slug,
        project_slug,
        environment_slug,
        path_param::<Volume>(cx),
    )
    .ok_or_not_found()?;
    let target = match volume.binding.as_ref() {
        Some(binding) => format!(
            "{}/settings#storage",
            component_path(
                tenant_slug,
                project_slug,
                environment_slug,
                binding.component_slug,
            ),
        ),
        None => environment_path(tenant_slug, project_slug, environment_slug),
    };

    Err(redirect_permanent(&target).into())
}

#[page("/tenants/{tenant}/projects/{project}/environments/{environment}/new-component")]
async fn create_component(cx: &Cx) -> Result {
    let tenant_slug = path_param::<Tenant>(cx);
    let project_slug = path_param::<Project>(cx);
    let environment_slug = path_param::<Environment>(cx);
    let tenant = mock::tenant(tenant_slug).ok_or_not_found()?;
    let project = mock::project(tenant_slug, project_slug).ok_or_not_found()?;
    let environment =
        mock::environment(tenant_slug, project_slug, environment_slug).ok_or_not_found()?;
    let query = query_params::<UiQuery>(cx)?;
    let action = environment_path(tenant_slug, project_slug, environment_slug);
    let restored_storage = query.restore_storage.as_deref().and_then(|storage_slug| {
        environment
            .volumes
            .iter()
            .find(|storage| storage.slug == storage_slug)
    });
    let storage_enabled = restored_storage.is_some();
    let storage_size = restored_storage.map_or(10, |storage| storage.capacity_gib);
    let storage_mount_path = restored_storage
        .and_then(|storage| storage.binding.as_ref())
        .map_or("/data", |binding| binding.mount_path);
    let storage_backup_policy =
        restored_storage.map_or("Disabled", |storage| storage.backup_policy);

    view! {
        <main class="mx-auto max-w-2xl p-6">
            <nav class="mb-5 flex flex-wrap items-center gap-1.5 text-xs" aria-label="Breadcrumb">
                <a href=(tenant_path(tenant_slug))>"Projects"</a>
                icon(data: iconify_icon!("feather:chevron-right"), attrs: attributes! { class="size-3" })
                <a href=(project_path(tenant_slug, project_slug))>(project.name)</a>
                icon(data: iconify_icon!("feather:chevron-right"), attrs: attributes! { class="size-3" })
                <a href=(environment_path(tenant_slug, project_slug, environment_slug))>
                    (environment.name)
                </a>
                icon(data: iconify_icon!("feather:chevron-right"), attrs: attributes! { class="size-3" })
                <span aria-current="page">"New component"</span>
            </nav>
            <header class="mb-6">
                <h1 class="text-2xl font-semibold">"New component"</h1>
                <p class="mt-1 text-sm">
                    "Add a component to "
                    (environment.name)
                    "."
                </p>
            </header>
            <form
                action=(action)
                method="get"
                class="card bg-base-100"
            >
                <div class="space-y-6 px-5 py-5 sm:px-6">
                        <input type="hidden" name="action" value="create-component">
                        <input type="hidden" name="variables" data-variable-serialized="">
                        <section class="space-y-4" aria-labelledby="component-basics-heading">
                            <h2 id="component-basics-heading" class="text-sm font-semibold">"Component"</h2>
                            <div class="grid gap-5 sm:grid-cols-2">
                                <div class="space-y-2">
                                    <label class="label" for="component-kind">"Type"</label>
                                    <select class="select w-full" id="component-kind"
                                            name="kind"
                                            data-component-kind-select=""><option value="application">"Application"</option>
                                        <option value="cron-job">"Cron job"</option>
                                        <option value="postgresql">"PostgreSQL"</option>
                                        <option value="valkey">"Valkey"</option></select>
                                </div>
                                <div class="space-y-2">
                                    <label class="label" for="component-name">"Name"</label>
                                    <input class="input w-full" id="component-name"
                                        name="name"
                                        placeholder="web"
                                        required=(true)
                                        pattern="[A-Za-z0-9](?:[A-Za-z0-9\\-]{0,61}[A-Za-z0-9])?"
                                        title="Use letters, numbers, and hyphens."
                                        data-component-name="">
                                </div>
                            </div>
                        </section>

                        <fieldset data-component-kind-fields="application" class="space-y-6">
                            <legend class="sr-only">"Application settings"</legend>
                            <section class="space-y-4 pt-5" aria-labelledby="new-component-source-heading">
                                <h2 id="new-component-source-heading" class="text-sm font-semibold">"Source"</h2>
                                <div class="grid gap-5 sm:grid-cols-[10rem_minmax(0,1fr)]">
                                    <div class="space-y-2">
                                        <label class="label" for="application-source-kind">"Deploy from"</label>
                                        <select class="select w-full" id="application-source-kind"
                                                name="source_kind"
                                                data-application-source-kind=""><option value="repository">"Git repository"</option>
                                            <option value="image">"Container image"</option></select>
                                    </div>
                                    <div class="space-y-2">
                                        <label class="label" for="application-source">"Repository or image"</label>
                                        <input class="input w-full" id="application-source"
                                                name="source"
                                                placeholder="https://github.com/owner/repository"
                                                required=(true)
                                                data-application-source="">
                                    </div>
                                </div>
                            </section>

                            <section class="space-y-4 pt-5" aria-labelledby="new-component-access-heading">
                                <h2 id="new-component-access-heading" class="text-sm font-semibold">"Access"</h2>
                                <div class="grid gap-4 sm:grid-cols-[10rem_minmax(0,1fr)] sm:items-start">
                                    <div class="space-y-2">
                                        <label class="label" for="application-exposure">"Visibility"</label>
                                        <select class="select w-full" id="application-exposure"
                                                name="exposure"
                                                data-network-exposure=""><option value="Public">"Public"</option>
                                            <option value="Private">"Private"</option></select>
                                    </div>
                                    <div data-public-network-fields="" class="min-w-0 space-y-3 sm:pt-0.5">
                                        <div class="min-w-0">
                                            <p class="text-xs">"Domain"</p>
                                            <code
                                                data-managed-domain-preview=""
                                                data-domain-tenant=(tenant_slug)
                                                data-domain-project=(project_slug)
                                                data-domain-environment=(environment_slug)
                                                class="mt-1 block break-all text-xs"
                                            >
                                                (managed_domain(tenant_slug, project_slug, environment_slug, "app"))
                                            </code>
                                        </div>
                                        <details class="group">
                                            <summary class="flex w-fit cursor-pointer list-none items-center gap-1 text-xs font-medium outline-none [&::-webkit-details-marker]:hidden">
                                                "Use a custom domain"
                                                icon(
                                                    data: iconify_icon!("feather:chevron-down"),
                                                    attrs: attributes! { class="size-3.5 group-open:rotate-180" }
                                                )
                                            </summary>
                                            if tenant.domains.is_empty() {
                                                <div class="mt-3 max-w-sm px-3 py-3">
                                                    <p class="text-xs">
                                                        "Register and verify a tenant domain before assigning a custom hostname."
                                                    </p>
                                                    <a
                                                        href=(format!("{}/settings", tenant_path(tenant_slug)))
                                                        class="mt-2 inline-block text-xs font-medium"
                                                    >
                                                        "Register a domain"
                                                    </a>
                                                </div>
                                            } else {
                                                <div class="mt-3 max-w-sm space-y-2">
                                                    <label class="label" for="application-domain">"Hostname"</label>
                                                    <input class="input w-full" id="application-domain"
                                                        name="domain"
                                                        placeholder=(format!("app.{}", tenant.domains[0]))
                                                        inputmode="url"
                                                        autocomplete="url"
                                                        data-custom-domain=""
                                                        data-registered-domains=(tenant.domains.join(","))
                                                        pattern="[A-Za-z0-9](?:[A-Za-z0-9\\-]{0,61}[A-Za-z0-9])?(?:[.][A-Za-z0-9](?:[A-Za-z0-9\\-]{0,61}[A-Za-z0-9])?)+"
                                                        title="Enter a hostname under a verified tenant domain.">
                                                    <p class="text-xs">
                                                        "Verified domains: "
                                                        (tenant.domains.join(", "))
                                                    </p>
                                                </div>
                                            }
                                        </details>
                                    </div>
                                </div>
                            </section>

                            <section data-variable-editor="" class="space-y-4 pt-5" aria-labelledby="new-component-variables-heading">
                                <div class="flex flex-wrap items-start justify-between gap-3">
                                    <div>
                                        <h2 id="new-component-variables-heading" class="text-sm font-semibold">"Environment variables"</h2>
                                        <p class="mt-1 text-xs">"Values are encrypted and write-only."</p>
                                    </div>
                                    <button
                                        type="button"
                                        data-add-variable=""
                                        class="btn btn-outline btn-sm"
                                    >
                                        icon(data: iconify_icon!("feather:plus"), attrs: attributes! { class="size-3.5" })
                                        "Add variable"
                                    </button>
                                </div>
                                <div data-variable-rows="" class="space-y-3"></div>
                                <template data-variable-template="">
                                    <div data-variable-row="" data-variable-new="" class="grid gap-2 sm:grid-cols-[minmax(10rem,1fr)_minmax(12rem,1.4fr)_auto] sm:items-end">
                                        <label class="space-y-1.5">
                                            <span class="block text-xs">"Key"</span>
                                            <input class="input w-full" data-variable-key=""
                                                placeholder="VARIABLE_NAME"
                                                aria-label="Variable key"
                                                pattern="[A-Za-z_][A-Za-z0-9_]*"
                                                title="Use letters, numbers, and underscores; do not start with a number."
                                                required=(true)>
                                        </label>
                                        <label class="space-y-1.5">
                                            <span class="block text-xs">"Value"</span>
                                            <input class="input w-full" data-variable-value=""
                                                type="password"
                                                placeholder="Required"
                                                aria-label="Variable value"
                                                autocomplete="new-password"
                                                required=(true)>
                                        </label>
                                        <button
                                            type="button"
                                            data-remove-variable=""
                                            aria-label="Remove variable"
                                            class="btn btn-outline"
                                        >
                                            icon(data: iconify_icon!("feather:trash-2"), attrs: attributes! { class="size-3.5" })
                                            <span class="sm:sr-only">"Remove"</span>
                                        </button>
                                    </div>
                                </template>
                                <p data-variable-status="" class="sr-only" aria-live="polite"></p>
                            </section>

                            <section
                                data-storage-config=""
                                class="space-y-4 pt-5"
                                aria-labelledby="application-storage-heading"
                            >
                                <div class="flex flex-wrap items-start justify-between gap-4">
                                    <div>
                                        <h2 id="application-storage-heading" class="text-sm font-semibold">"Persistent storage"</h2>
                                        <p class="mt-1 text-xs">"One private disk, owned by this component."</p>
                                    </div>
                                    <div class="flex items-center gap-2">
                                        <input class="toggle toggle-primary" type="checkbox" role="switch" id="application-storage"
                                            name="storage_enabled"
                                            checked=(storage_enabled)
                                            data-storage-toggle="">
                                        <label class="label" for="application-storage">"Enable"</label>
                                    </div>
                                </div>
                                <div
                                    data-storage-fields=""
                                    hidden=(!storage_enabled)
                                    class="space-y-4"
                                >
                                    match restored_storage {
                                        Some(storage) => <div class="px-3 py-2.5">
                                            <input
                                                type="hidden"
                                                name="restore_storage"
                                                value=(storage.slug)
                                                disabled=(!storage_enabled)
                                            >
                                            <p class="text-sm font-medium">"Using retained data from "(storage.name)</p>
                                            <p class="mt-1 text-xs">
                                                (format!("{} GiB · backups preserved", storage.capacity_gib))
                                            </p>
                                        </div>,
                                        None => "",
                                    }
                                    <div class="grid gap-4 sm:grid-cols-2">
                                        <div class="space-y-2">
                                            <label class="label" for="application-storage-size">"Size (GiB)"</label>
                                            <input class="input w-full" id="application-storage-size"
                                                name="size"
                                                type="number"
                                                value=(storage_size)
                                                min=(storage_size)
                                                required=(true)
                                                disabled=(!storage_enabled)>
                                        </div>
                                        <div class="space-y-2">
                                            <label class="label" for="application-mount-path">"Mount path"</label>
                                            <input class="input w-full" id="application-mount-path"
                                                name="mount_path"
                                                value=(storage_mount_path)
                                                pattern="/.*"
                                                title="Use an absolute path beginning with /."
                                                required=(true)
                                                disabled=(!storage_enabled)>
                                        </div>
                                    </div>
                                    <div class="max-w-xs space-y-2">
                                        <label class="label" for="application-backup-policy">"Backup policy"</label>
                                        <select class="select w-full" id="application-backup-policy"
                                                name="backup_policy"
                                                disabled=(!storage_enabled)><option value="Disabled" selected=(storage_backup_policy == "Disabled")>"Disabled"</option>
                                            <option value="Daily · retain 7" selected=(storage_backup_policy == "Daily · retain 7")>"Daily · keep 7"</option>
                                            <option value="Daily · retain 14" selected=(storage_backup_policy == "Daily · retain 14")>"Daily · keep 14"</option>
                                            <option value="Weekly · retain 4" selected=(storage_backup_policy == "Weekly · retain 4")>"Weekly · keep 4"</option></select>
                                    </div>
                                </div>
                            </section>

                            <details class="group pt-5">
                                <summary class="flex cursor-pointer list-none items-center justify-between text-sm font-semibold outline-none [&::-webkit-details-marker]:hidden">
                                    "Advanced"
                                    icon(
                                        data: iconify_icon!("feather:chevron-down"),
                                        attrs: attributes! { class="size-4 group-open:rotate-180" }
                                    )
                                </summary>
                                <div class="mt-4 grid gap-5 sm:grid-cols-2">
                                    <div class="space-y-2">
                                        <label class="label" for="application-replicas">"Replicas"</label>
                                        <input class="input w-full" id="application-replicas" name="replicas" type="number" value="1" min="0">
                                    </div>
                                    <div class="space-y-2">
                                        <label class="label" for="application-port">"Service port"</label>
                                        <input class="input w-full" id="application-port"
                                            name="port"
                                            type="number"
                                            placeholder="8080"
                                            min="1"
                                            max="65535">
                                        <p class="text-xs">
                                            "Optional. Buildpack apps receive the platform port in "
                                            <code>"PORT"</code>
                                            "; container images can override it here."
                                        </p>
                                    </div>
                                    <div class="flex items-center gap-2 sm:col-span-2">
                                        <input class="toggle toggle-primary" type="checkbox" role="switch" id="application-auto-deploy" name="auto_deploy" checked=(true)>
                                        <label class="label" for="application-auto-deploy">"Automatically deploy updates"</label>
                                    </div>
                                </div>
                            </details>
                        </fieldset>

                        <fieldset
                            data-component-kind-fields="cron-job"
                            class="space-y-5 pt-5"
                            hidden=(true)
                        >
                            <legend class="sr-only">"Cron job settings"</legend>
                            <div class="space-y-2">
                                <label class="label" for="cron-source">"Repository or image"</label>
                                <input class="input w-full" id="cron-source"
                                    name="source"
                                    placeholder="https://github.com/owner/repository"
                                    required=(true)
                                    disabled=(true)>
                            </div>
                            <div class="space-y-2">
                                <label class="label" for="cron-command">"Command"</label>
                                <input class="input w-full" id="cron-command"
                                    name="command"
                                    placeholder="bin/run-report"
                                    required=(true)
                                    disabled=(true)>
                            </div>
                            <div class="grid gap-5 sm:grid-cols-2">
                                <div class="space-y-2">
                                    <label class="label" for="cron-expression">"Cron expression"</label>
                                    <input class="input w-full" id="cron-expression"
                                    name="cron"
                                    placeholder="0 3 * * *"
                                    required=(true)
                                    disabled=(true)>
                                </div>
                                <div class="space-y-2">
                                    <label class="label" for="cron-timezone">"Timezone"</label>
                                    <select class="select w-full" id="cron-timezone" name="timezone" disabled=(true)><option value="UTC">"UTC"</option>
                                        <option value="Asia/Ho_Chi_Minh">"Asia/Ho_Chi_Minh"</option>
                                        <option value="Europe/Helsinki">"Europe/Helsinki"</option></select>
                                </div>
                            </div>
                            <section
                                data-storage-config=""
                                class="space-y-4 pt-5"
                                aria-labelledby="cron-storage-heading"
                            >
                                <div class="flex flex-wrap items-start justify-between gap-4">
                                    <div>
                                        <h2 id="cron-storage-heading" class="text-sm font-semibold">"Persistent storage"</h2>
                                        <p class="mt-1 text-xs">"One private disk, owned by this component."</p>
                                    </div>
                                    <div class="flex items-center gap-2">
                                        <input class="toggle toggle-primary" type="checkbox" role="switch" id="cron-storage"
                                            name="storage_enabled"
                                            data-storage-toggle=""
                                            disabled=(true)>
                                        <label class="label" for="cron-storage">"Enable"</label>
                                    </div>
                                </div>
                                <div data-storage-fields="" hidden=(true) class="space-y-4">
                                    <div class="grid gap-4 sm:grid-cols-2">
                                        <div class="space-y-2">
                                            <label class="label" for="cron-storage-size">"Size (GiB)"</label>
                                            <input class="input w-full" id="cron-storage-size"
                                                name="size"
                                                type="number"
                                                value="10"
                                                min="1"
                                                required=(true)
                                                disabled=(true)>
                                        </div>
                                        <div class="space-y-2">
                                            <label class="label" for="cron-mount-path">"Mount path"</label>
                                            <input class="input w-full" id="cron-mount-path"
                                                name="mount_path"
                                                value="/data"
                                                pattern="/.*"
                                                title="Use an absolute path beginning with /."
                                                required=(true)
                                                disabled=(true)>
                                        </div>
                                    </div>
                                    <div class="max-w-xs space-y-2">
                                        <label class="label" for="cron-backup-policy">"Backup policy"</label>
                                        <select class="select w-full" id="cron-backup-policy"
                                                name="backup_policy"
                                                disabled=(true)><option value="Disabled">"Disabled"</option>
                                            <option value="Daily · retain 7">"Daily · keep 7"</option>
                                            <option value="Daily · retain 14">"Daily · keep 14"</option>
                                            <option value="Weekly · retain 4">"Weekly · keep 4"</option></select>
                                    </div>
                                </div>
                            </section>
                        </fieldset>

                        <fieldset
                            data-component-kind-fields="postgresql"
                            class="space-y-5 pt-5"
                            hidden=(true)
                        >
                            <legend class="sr-only">"Managed PostgreSQL settings"</legend>
                            <div class="grid gap-5 sm:grid-cols-2">
                                <div class="space-y-2">
                                    <label class="label" for="postgres-version">"PostgreSQL version"</label>
                                    <select class="select w-full" id="postgres-version" name="version" disabled=(true)><option value="17">"17"</option>
                                        <option value="16">"16"</option>
                                        <option value="15">"15"</option></select>
                                </div>
                                <div class="space-y-2">
                                    <label class="label" for="postgres-plan">"Compute plan"</label>
                                    <select class="select w-full" id="postgres-plan" name="plan" disabled=(true)><option value="shared-1">"Shared · 1 vCPU / 1 GiB"</option>
                                        <option value="standard-2">"Standard · 2 vCPU / 4 GiB"</option>
                                        <option value="standard-4">"Standard · 4 vCPU / 8 GiB"</option></select>
                                </div>
                            </div>
                            <div class="space-y-2">
                                <label class="label" for="postgres-storage">"Storage (GiB)"</label>
                                <input class="input w-full" id="postgres-storage"
                                    name="storage"
                                    type="number"
                                    value="20"
                                    min="10"
                                    disabled=(true)>
                            </div>
                            <div class="grid gap-3 sm:grid-cols-2">
                                <div class="flex items-center gap-2">
                                    <input class="toggle toggle-primary" type="checkbox" role="switch" id="postgres-backups" name="backups" checked=(true) disabled=(true)>
                                    <label class="label" for="postgres-backups">"Daily backups"</label>
                                </div>
                                <div class="flex items-center gap-2">
                                    <input class="toggle toggle-primary" type="checkbox" role="switch" id="postgres-ha" name="high_availability" disabled=(true)>
                                    <label class="label" for="postgres-ha">"High availability"</label>
                                </div>
                            </div>
                        </fieldset>

                        <fieldset
                            data-component-kind-fields="valkey"
                            class="space-y-5 pt-5"
                            hidden=(true)
                        >
                            <legend class="sr-only">"Managed Valkey settings"</legend>
                            <div class="grid gap-5 sm:grid-cols-2">
                                <div class="space-y-2">
                                    <label class="label" for="valkey-version">"Valkey version"</label>
                                    <select class="select w-full" id="valkey-version" name="version" disabled=(true)><option value="8">"8"</option>
                                        <option value="7">"7"</option></select>
                                </div>
                                <div class="space-y-2">
                                    <label class="label" for="valkey-plan">"Memory plan"</label>
                                    <select class="select w-full" id="valkey-plan" name="plan" disabled=(true)><option value="256mb">"Shared · 256 MiB"</option>
                                        <option value="1gb">"Standard · 1 GiB"</option>
                                        <option value="4gb">"Standard · 4 GiB"</option></select>
                                </div>
                            </div>
                            <div class="grid gap-5 sm:grid-cols-2">
                                <div class="space-y-2">
                                    <label class="label" for="valkey-persistence">"Persistence"</label>
                                    <select class="select w-full" id="valkey-persistence" name="persistence" disabled=(true)><option value="none">"None"</option>
                                        <option value="aof">"AOF"</option>
                                        <option value="snapshot">"Snapshots"</option></select>
                                </div>
                                <div class="space-y-2">
                                    <label class="label" for="valkey-eviction">"Eviction policy"</label>
                                    <select class="select w-full" id="valkey-eviction" name="eviction" disabled=(true)><option value="allkeys-lru">"allkeys-lru"</option>
                                        <option value="volatile-lru">"volatile-lru"</option>
                                        <option value="noeviction">"noeviction"</option></select>
                                </div>
                            </div>
                        </fieldset>
                </div>
                <div class="flex justify-end gap-2 px-5 py-4 sm:px-6">
                    <a href=(environment_path(tenant_slug, project_slug, environment_slug)) class="btn btn-outline">"Cancel"</a>
                    <button class="btn btn-primary" type="submit">"Create component"</button>
                </div>
            </form>
        </main>
    }
}

#[layout("/tenants/{tenant}/projects/{project}/environments/{environment}/components/{component}")]
async fn component_layout(cx: &Cx, slot: Slot<'_>) -> Result {
    let tenant_slug = path_param::<Tenant>(cx);
    let project_slug = path_param::<Project>(cx);
    let environment_slug = path_param::<Environment>(cx);
    let tenant = mock::tenant(tenant_slug).ok_or_not_found()?;
    let project = mock::project(tenant_slug, project_slug).ok_or_not_found()?;
    let environment =
        mock::environment(tenant_slug, project_slug, environment_slug).ok_or_not_found()?;
    let component = current_component(cx)?;
    let dependencies =
        mock::dependencies(tenant_slug, project_slug, environment_slug, component.slug);
    let base_path = component_path(tenant_slug, project_slug, environment_slug, component.slug);
    let settings_path = format!("{base_path}/settings");
    let changes_path = format!("{base_path}/changes");
    let current_path = uri(cx).path();
    let component_suffix = if current_path == settings_path {
        "/settings"
    } else if current_path == changes_path {
        "/changes"
    } else {
        ""
    };
    let query = query_params::<UiQuery>(cx)?;
    let feedback = feedback_message(query);

    view! {
        <main class="p-6">
            <nav class="mb-5 flex flex-wrap items-center gap-1.5 text-xs" aria-label="Breadcrumb">
                <a href=(tenant_path(tenant_slug))>"Projects"</a>
                <span class="inline-flex min-w-0 items-center gap-1.5">
                    breadcrumb_separator()
                    project_context_selector(
                        tenant: tenant,
                        project: project,
                        current_environment_slug: Some(environment.slug),
                    )
                </span>
                <span class="inline-flex min-w-0 items-center gap-1.5">
                    breadcrumb_separator()
                    environment_context_selector(
                        tenant_slug: tenant.slug,
                        project: project,
                        environment: environment,
                        component: Some(component),
                        component_suffix: component_suffix,
                    )
                </span>
            </nav>
            <header class="mb-5 flex flex-wrap items-start justify-between gap-4">
                <div>
                    <div class="flex items-center gap-1">
                        <h1 class="font-mono text-2xl font-semibold tracking-tight">(component.name)</h1>
                        component_title_switcher(
                            tenant_slug: tenant.slug,
                            project_slug: project.slug,
                            environment: environment,
                            component: component,
                            component_suffix: component_suffix,
                        )
                    </div>
                    <p class="mt-1 text-sm">(component.summary)</p>
                    <p class="mt-2 flex flex-wrap items-center gap-x-4 gap-y-1 text-xs">
                        <span>(component.kind)</span>
                        <span class="flex items-center gap-1.5">
                            <span class=(if component.observability.is_some() { "status status-success" } else { "status status-warning" })></span>
                            match component.observability {
                                Some(observability) => <span>(observability.health)</span>,
                                None => <span>(component.state)</span>,
                            }
                        </span>
                    </p>
                    if !dependencies.is_empty() {
                        <p class="mt-2 flex flex-wrap items-center gap-2 text-xs">
                            <span>"Depends on"</span>
                            for dependency in dependencies {
                                <a
                                    href=(component_path(
                                        tenant_slug,
                                        project_slug,
                                        environment_slug,
                                        dependency.slug,
                                    ))
                                    class="font-medium"
                                >
                                    (dependency.name)
                                </a>
                            }
                        </p>
                    }
                </div>
                match component.url {
                    Some(url) => <a
                        href=(url)
                        target="_blank"
                        rel="noopener"
                        class="btn btn-outline"
                    >
                        icon(data: iconify_icon!("feather:external-link"), attrs: attributes! { class="size-4" })
                        "Open app"
                    </a>,
                    None => "",
                }
            </header>
            feedback_banner(message: feedback)
            <nav role="tablist" class="tabs tabs-border mb-6" aria-label="Component">
                if current_path == base_path {
                    <span role="tab" aria-current="page" class="tab tab-active">"Overview"</span>
                } else {
                    <a role="tab" href=(base_path.clone()) class="tab">"Overview"</a>
                }
                if current_path == settings_path {
                    <span role="tab" aria-current="page" class="tab tab-active">"Settings"</span>
                } else {
                    <a role="tab" href=(settings_path.clone()) class="tab">"Settings"</a>
                }
                if current_path == changes_path {
                    <span role="tab" aria-current="page" class="tab tab-active">"Changes"</span>
                } else {
                    <a role="tab" href=(changes_path.clone()) class="tab">"Changes"</a>
                }
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
            <section class="card card-body bg-base-100 items-center text-center" aria-labelledby="observability-pending-heading">
                <h2 id="observability-pending-heading" class="text-sm font-semibold">"Observability pending"</h2>
                <p class="mt-1 text-xs">"Metrics and logs will appear after the first deployment."</p>
            </section>
        };
    };
    let platform_settings = component
        .settings
        .iter()
        .filter(|setting| {
            !setting.value.is_empty()
                && matches!(
                    setting.label,
                    "Port"
                        | "Domain"
                        | "Backup"
                        | "Authentication"
                        | "Egress"
                        | "Secrets"
                        | "Dependency"
                        | "Visibility"
                        | "Accelerator"
                        | "Provider"
                        | "Protocol"
                        | "Depends on"
                        | "Integrations"
                        | "Database"
                        | "Network"
                        | "Capabilities"
                        | "Configuration"
                        | "Backend"
                )
        })
        .collect::<Vec<_>>();

    view! {
        <section class="space-y-5" aria-labelledby="observability-heading">
            <h2 id="observability-heading" class="sr-only">"Runtime overview"</h2>
            <dl class="stats stats-vertical sm:stats-horizontal w-full">
                <div class="stat">
                    <dt class="stat-title">"Uptime"</dt>
                    <dd class="stat-value">(observability.uptime)</dd>
                </div>
                <div class="stat">
                    <dt class="stat-title">(observability.primary_metric.label)</dt>
                    <dd class="stat-value">(observability.primary_metric.value)</dd>
                </div>
                <div class="stat">
                    <dt class="stat-title">(observability.secondary_metric.label)</dt>
                    <dd class="stat-value">(observability.secondary_metric.value)</dd>
                </div>
            </dl>

            <div class="grid items-start gap-5 lg:grid-cols-2">
                <section class="card bg-base-100" aria-labelledby="resource-usage-heading">
                    <header class="flex items-center justify-between px-4 py-3">
                        <h3 id="resource-usage-heading" class="text-sm font-semibold">"Resource usage"</h3>
                        <span class="text-xs">"Sample snapshot"</span>
                    </header>
                    <div class="px-4">
                        <div class="py-4">
                            <div class="mb-2 flex items-center justify-between gap-4 text-sm">
                                <span>"CPU"</span>
                                <span class="font-medium">(format!("{}%", observability.cpu_percent))</span>
                            </div>
                            <progress class="progress progress-primary w-full" value=(observability.cpu_percent) max=(100.0) aria-label="CPU utilization"></progress>
                        </div>
                        <div class="py-4">
                            <div class="mb-2 flex items-center justify-between gap-4 text-sm">
                                <span>"Memory"</span>
                                <span class="font-medium">(format!("{}%", observability.memory_percent))</span>
                            </div>
                            <progress class="progress progress-primary w-full" value=(observability.memory_percent) max=(100.0) aria-label="Memory utilization"></progress>
                        </div>
                    </div>
                </section>

                <section class="card bg-base-100" aria-labelledby="runtime-details-heading">
                    <header class="px-4 py-3">
                        <h3 id="runtime-details-heading" class="text-sm font-semibold">"Runtime details"</h3>
                    </header>
                    <dl class="px-4 text-xs">
                        <div class="flex items-start justify-between gap-4 py-3">
                            <dt>"Applied revision"</dt>
                            <dd class="max-w-64 break-words text-right font-mono font-medium">(observability.release)</dd>
                        </div>
                        <div class="flex items-start justify-between gap-4 py-3">
                            <dt>"Replicas"</dt>
                            <dd class="text-right font-medium">(observability.replicas)</dd>
                        </div>
                        <div class="flex items-start justify-between gap-4 py-3">
                            <dt>"Desired state"</dt>
                            <dd class="text-right font-medium">(component.state)</dd>
                        </div>
                        for setting in platform_settings {
                            <div class="flex items-start justify-between gap-4 py-3">
                                <dt>(setting.label)</dt>
                                <dd class="max-w-64 break-words text-right font-medium">(setting.value)</dd>
                            </div>
                        }
                    </dl>
                </section>
            </div>

            <section
                data-log-viewer=""
                class="card bg-base-100"
                aria-labelledby="logs-heading"
            >
                <header class="space-y-3 px-4 py-3">
                    <div class="flex items-center justify-between gap-3">
                        <h3 id="logs-heading" class="text-sm font-semibold">"Logs"</h3>
                        <span data-log-count="" aria-live="polite" class="text-xs">
                            (count_label(observability.logs.len(), "line", "lines"))
                        </span>
                    </div>
                    <div class="grid gap-2 sm:grid-cols-[minmax(0,1fr)_9rem]">
                        <label class="relative block">
                            <span class="sr-only">"Filter logs"</span>
                            icon(data: iconify_icon!("feather:search"), attrs: attributes! { class="pointer-events-none absolute top-1/2 left-3 size-3.5 -translate-y-1/2" })
                            <input class="input w-full" type="search"
                                placeholder="Filter logs"
                                data-log-search="">
                        </label>
                        <select class="select w-full" aria-label="Log level" data-log-level=""><option value="All">"All levels"</option>
                            <option value="INFO">"Info"</option>
                            <option value="WARN">"Warning"</option>
                            <option value="ERROR">"Error"</option></select>
                    </div>
                </header>
                <div class="relative">
                    <pre
                        data-log-output=""
                        class="max-h-80 overflow-auto px-4 py-3 font-mono text-xs leading-6"
                        aria-label="Application logs"
                        tabindex="0"
                    ><samp class="block">for line in observability.logs.iter().rev() {
                                <span
                                    data-log-line=""
                                    data-log-level=(line.level)
                                    class="block min-w-max whitespace-pre"
                                ><span class=(match line.level {
                                    "ERROR" => "text-error",
                                    "WARN" => "text-warning",
                                    _ => "text-base-content/60",
                                })><time>(line.time)</time>" "(line.level)</span>" "(line.message)</span>
                            }</samp></pre>
                    <p
                        data-log-empty=""
                        hidden=(true)
                        role="status"
                        class="px-4 py-8 text-center text-sm"
                    >
                        "No logs match these filters."
                    </p>
                </div>
            </section>
        </section>
    }
}

#[page(
    "/tenants/{tenant}/projects/{project}/environments/{environment}/components/{component}/settings"
)]
async fn component_settings(cx: &Cx) -> Result {
    let tenant_slug = path_param::<Tenant>(cx);
    let project_slug = path_param::<Project>(cx);
    let environment_slug = path_param::<Environment>(cx);
    let tenant = mock::tenant(tenant_slug).ok_or_not_found()?;
    let component = current_component(cx)?;
    let query = query_params::<UiQuery>(cx)?;
    let action = uri(cx).path();
    let initial_storage =
        mock::storage_for_component(tenant_slug, project_slug, environment_slug, component.slug);
    let dependent_components =
        mock::dependents(tenant_slug, project_slug, environment_slug, component.slug);
    let mutation_error = match query.action.as_deref() {
        Some("save-component") => {
            let update = component_settings_update_from_query(
                component,
                query,
                tenant,
                initial_storage.is_some(),
            );
            match mock::update_component_settings(
                tenant_slug,
                project_slug,
                environment_slug,
                component.slug,
                update,
            ) {
                Ok(_) => {
                    let destination = format!("{action}?notice=component-updated");
                    return Err(redirect(&destination).into());
                }
                Err(error) => Some(error),
            }
        }
        Some("remove-volume") => {
            match mock::remove_volume(
                tenant_slug,
                project_slug,
                environment_slug,
                query.volume.as_deref().unwrap_or(""),
            ) {
                Ok(()) => {
                    let destination = format!("{action}?notice=storage-deleted");
                    return Err(redirect(&destination).into());
                }
                Err(error) => Some(error),
            }
        }
        _ => None,
    };
    let managed_service = match component.kind {
        "Managed PostgreSQL" => Some("postgresql"),
        "Managed Valkey" => Some("valkey"),
        _ => None,
    };

    if let Some(managed_service) = managed_service {
        let settings_submitted = query.action.as_deref() == Some("save-component");
        let version = if settings_submitted {
            query
                .version
                .as_deref()
                .unwrap_or_else(|| setting_value(component, "Version"))
        } else {
            setting_value(component, "Version")
        };
        let plan = if settings_submitted {
            query.plan.as_deref().unwrap_or_else(|| {
                if managed_service == "postgresql" {
                    setting_value(component, "Compute")
                } else {
                    setting_value(component, "Memory")
                }
            })
        } else if managed_service == "postgresql" {
            setting_value(component, "Compute")
        } else {
            setting_value(component, "Memory")
        };
        let size = query
            .size
            .as_deref()
            .unwrap_or_else(|| setting_value(component, "Storage"));
        let backup_policy = query
            .backup_policy
            .as_deref()
            .unwrap_or_else(|| setting_value(component, "Backup"));
        let persistence = query
            .persistence
            .as_deref()
            .unwrap_or_else(|| setting_value(component, "Persistence"));
        let eviction = query
            .eviction
            .as_deref()
            .unwrap_or_else(|| setting_value(component, "Eviction"));
        let environment_action = environment_path(tenant_slug, project_slug, environment_slug);

        return view! {
            <div class="space-y-6">
                feedback_banner(message: mutation_error, is_error: true)
                <div class="card bg-base-100"><div class="card-body"><h3 class="card-title">if managed_service == "postgresql" {
                            "PostgreSQL settings"
                        } else {
                            "Valkey settings"
                        }</h3>
                        <p>"Netamos manages lifecycle, credentials, and health checks."</p><form action=(action) method="get" class="space-y-5" data-settings-form="">
                            <input type="hidden" name="action" value="save-component">
                            if managed_service == "postgresql" {
                                <div class="grid gap-4 sm:grid-cols-2">
                                    <div class="space-y-2">
                                        <label class="label" for="postgres-version">"PostgreSQL version"</label>
                                        <select class="select w-full" id="postgres-version" name="version"><option value="17" selected=(version == "17" || version.is_empty())>"17"</option>
                                            <option value="16" selected=(version == "16")>"16"</option>
                                            <option value="15" selected=(version == "15")>"15"</option></select>
                                    </div>
                                    <div class="space-y-2">
                                        <label class="label" for="postgres-compute">"Compute"</label>
                                        <select class="select w-full" id="postgres-compute" name="plan"><option value="Shared 1 vCPU · 1 GiB" selected=(plan.is_empty() || plan == "Shared 1 vCPU · 1 GiB")>"Shared · 1 vCPU · 1 GiB"</option>
                                            <option value="Dedicated 2 vCPU · 4 GiB" selected=(plan == "Dedicated 2 vCPU · 4 GiB")>"Dedicated · 2 vCPU · 4 GiB"</option></select>
                                    </div>
                                    <div class="space-y-2">
                                        <label class="label" for="postgres-storage">"Storage (GiB)"</label>
                                        <input class="input w-full" id="postgres-storage"
                                            name="size"
                                            type="number"
                                            min="10"
                                            value=(numeric_setting(size, 20))
                                            required=(true)>
                                        <p class="text-xs">"Can only be increased."</p>
                                    </div>
                                    <div class="space-y-2">
                                        <label class="label" for="postgres-backups">"Backups"</label>
                                        <select class="select w-full" id="postgres-backups" name="backup_policy"><option value="Daily · retain 7" selected=(backup_policy.is_empty() || backup_policy == "Daily · retain 7")>"Daily · keep 7"</option>
                                            <option value="Daily · retain 14" selected=(backup_policy == "Daily · retain 14")>"Daily · keep 14"</option></select>
                                    </div>
                                </div>
                                <label class="flex cursor-pointer items-start gap-3 pt-4">
                                    <input
                                        type="checkbox"
                                        name="high_availability"
                                        checked=(if settings_submitted {
                                            query.high_availability.is_some()
                                        } else {
                                            setting_value(component, "High availability") == "Enabled"
                                        })
                                        class="checkbox checkbox-primary"
                                    >
                                    <span>
                                        <span class="block text-sm font-medium">"High availability"</span>
                                        <span class="mt-0.5 block text-xs">"Run a synchronous standby in the same region."</span>
                                    </span>
                                </label>
                            } else {
                                <div class="grid gap-4 sm:grid-cols-2">
                                    <div class="space-y-2">
                                        <label class="label" for="valkey-version">"Valkey version"</label>
                                        <select class="select w-full" id="valkey-version" name="version"><option value="8" selected=(version == "8" || version.is_empty())>"8"</option>
                                            <option value="7" selected=(version == "7")>"7"</option></select>
                                    </div>
                                    <div class="space-y-2">
                                        <label class="label" for="valkey-memory">"Memory"</label>
                                        <select class="select w-full" id="valkey-memory" name="plan"><option value="512 MiB" selected=(plan.is_empty() || plan == "512 MiB")>"512 MiB"</option>
                                            <option value="1 GiB" selected=(plan == "1 GiB")>"1 GiB"</option>
                                            <option value="2 GiB" selected=(plan == "2 GiB")>"2 GiB"</option></select>
                                    </div>
                                    <div class="space-y-2">
                                        <label class="label" for="valkey-persistence">"Persistence"</label>
                                        <select class="select w-full" id="valkey-persistence" name="persistence"><option value="AOF every second" selected=(persistence.is_empty() || persistence == "AOF every second")>"AOF every second"</option>
                                            <option value="Disabled" selected=(persistence == "Disabled")>"Disabled"</option></select>
                                    </div>
                                    <div class="space-y-2">
                                        <label class="label" for="valkey-eviction">"Eviction policy"</label>
                                        <select class="select w-full" id="valkey-eviction" name="eviction"><option value="noeviction" selected=(eviction.is_empty() || eviction == "noeviction")>"No eviction"</option>
                                            <option value="allkeys-lru" selected=(eviction == "allkeys-lru")>"All keys · LRU"</option>
                                            <option value="volatile-lru" selected=(eviction == "volatile-lru")>"Expiring keys · LRU"</option></select>
                                    </div>
                                </div>
                            }
                            <div class="flex items-center justify-between gap-3 pt-5">
                                <p data-settings-status="" aria-live="polite" class="text-xs">
                                    "No unsaved changes"
                                </p>
                                <button class="btn btn-primary" type="submit" data-settings-submit="">"Save settings"</button>
                            </div>
                        </form></div></div>
                <div class="card bg-base-100"><div class="card-body"><h3 class="card-title">"Delete component"</h3>
                        <p>"This permanently removes the managed service and its data."</p>if dependent_components.is_empty() {
                            <form
                                action=(environment_action)
                                method="get"
                                data-confirm="Permanently delete this managed service and all of its data?"
                            >
                                <input type="hidden" name="action" value="delete-component">
                                <input type="hidden" name="component" value=(component.slug)>
                                <button class="btn btn-error" type="submit">"Delete managed service"</button>
                            </form>
                        } else {
                            deletion_dependencies(
                                tenant_slug: tenant_slug.to_owned(),
                                project_slug: project_slug.to_owned(),
                                environment_slug: environment_slug.to_owned(),
                                dependents: dependent_components,
                            )
                        }</div></div>
            </div>
        };
    }

    let branch = setting_value(component, "Branch");
    let source_label = match branch {
        "chart" => "Chart source",
        "image" => "Container image",
        _ => "Repository",
    };
    let settings_submitted = query.action.as_deref() == Some("save-component");
    let source = if settings_submitted {
        query
            .source
            .as_deref()
            .unwrap_or_else(|| setting_value(component, "Source"))
    } else {
        setting_value(component, "Source")
    };
    let replicas = if settings_submitted {
        query
            .replicas
            .as_deref()
            .unwrap_or_else(|| setting_value(component, "Replicas"))
    } else {
        setting_value(component, "Replicas")
    };
    let port = if settings_submitted {
        query
            .port
            .as_deref()
            .unwrap_or_else(|| setting_value(component, "Port"))
    } else {
        setting_value(component, "Port")
    };
    let configured_protocol = setting_value(component, "Protocol");
    let protocol = if configured_protocol == "UDP" {
        "UDP"
    } else {
        "TCP"
    };
    let supports_public_access = !matches!(component.kind, "PostgreSQL" | "Cache");
    let supports_web_domains = supports_public_access && protocol == "TCP";
    let visibility = setting_value(component, "Visibility");
    let network = setting_value(component, "Network");
    let configured_domain = setting_value(component, "Domain");
    let configured_public = visibility == "Public"
        || !configured_domain.is_empty()
        || component.url.is_some()
        || network.contains("LoadBalancer");
    let public_access = supports_public_access
        && if settings_submitted {
            query.exposure.as_deref() == Some("Public")
        } else {
            configured_public
        };
    let domain = if settings_submitted {
        if public_access {
            normalize_hostname(query.domain.as_deref().unwrap_or(""))
        } else {
            String::new()
        }
    } else {
        configured_domain.to_owned()
    };
    let managed_domain =
        managed_domain(tenant_slug, project_slug, environment_slug, component.slug);
    let private_endpoint = if port.is_empty() && branch == "chart" {
        format!("{}.{}.svc.internal", component.slug, environment_slug)
    } else if port.is_empty() {
        format!("{}.{}.svc.internal:8080", component.slug, environment_slug)
    } else {
        format!(
            "{}.{}.svc.internal:{port}",
            component.slug, environment_slug
        )
    };
    let auto_deploy = if settings_submitted {
        query.auto_deploy.is_some()
    } else {
        branch != "image"
    };
    let auto_deploy_label = if branch == "image" {
        "Automatically deploy new image digests"
    } else if branch == "chart" {
        "Automatically deploy chart updates"
    } else {
        "Automatically deploy source changes"
    };
    let environment_action = environment_path(tenant_slug, project_slug, environment_slug);
    let storage =
        mock::storage_for_component(tenant_slug, project_slug, environment_slug, component.slug);
    let storage_requested =
        storage.is_some() || (settings_submitted && query.storage_enabled.is_some());
    let existing_storage_size = storage.map_or(10, |storage| storage.capacity_gib);
    let storage_size = if settings_submitted && storage_requested {
        query
            .size
            .as_deref()
            .and_then(|size| size.parse::<u32>().ok())
            .unwrap_or(existing_storage_size)
    } else {
        existing_storage_size
    };
    let existing_mount_path = storage
        .and_then(|storage| storage.binding.as_ref())
        .map_or("/data", |binding| binding.mount_path);
    let storage_mount_path = if settings_submitted && storage_requested {
        query.mount_path.as_deref().unwrap_or(existing_mount_path)
    } else {
        existing_mount_path
    };
    let existing_backup_policy = storage.map_or("Disabled", |storage| storage.backup_policy);
    let storage_backup_policy = if settings_submitted && storage_requested {
        query
            .backup_policy
            .as_deref()
            .unwrap_or(existing_backup_policy)
    } else {
        existing_backup_policy
    };
    let storage_usage_percent = storage.map_or(0.0, |storage| {
        (storage.used_gib / storage.capacity_gib as f32 * 100.0).clamp(0.0, 100.0)
    });
    let latest_backup = storage.and_then(|storage| storage.backups.first());
    let variables = match (settings_submitted, query.variables.as_deref()) {
        (true, Some(serialized)) => parse_component_variables(serialized),
        _ => component_variable_states(component),
    };
    let serialized_variables = variables
        .iter()
        .map(|variable| variable.key.as_str())
        .collect::<Vec<_>>()
        .join("\n");

    view! {
        <div class="max-w-4xl space-y-6">
            feedback_banner(message: mutation_error, is_error: true)
            <div class="card bg-base-100"><div class="card-body"><form action=(action) method="get" class="space-y-6" data-settings-form="">
                        <input type="hidden" name="action" value="save-component">
                        <input
                            type="hidden"
                            name="variables"
                            value=(serialized_variables)
                            data-variable-serialized=""
                        >
                        <section class="space-y-4" aria-labelledby="source-heading">
                            <h4 id="source-heading" class="font-medium">"Source"</h4>
                            <div class="space-y-2">
                                <label class="label" for="settings-source">(source_label)</label>
                                <input class="input w-full" id="settings-source" name="source" value=(source)>
                            </div>
                            <div class="flex items-center gap-2">
                                <input class="toggle toggle-primary" type="checkbox" role="switch" id="settings-auto-deploy" name="auto_deploy" checked=(auto_deploy)>
                                <label class="label" for="settings-auto-deploy">(auto_deploy_label)</label>
                            </div>
                        </section>
                        <div class="divider"></div>
                        <section class="space-y-5" aria-labelledby="runtime-heading">
                            <h4 id="runtime-heading" class="font-medium">"Runtime"</h4>
                            <div class="grid gap-4 sm:grid-cols-3">
                                <div class="space-y-2">
                                    <label class="label" for="settings-replicas">"Replicas"</label>
                                    <input class="input w-full" id="settings-replicas"
                                        name="replicas"
                                        type="number"
                                        value=(replicas)
                                        min="0">
                                </div>
                                <div class="space-y-2">
                                    <label class="label" for="settings-port">"Service port"</label>
                                    <input class="input w-full" id="settings-port"
                                        name="port"
                                        type="number"
                                        value=(port)
                                        placeholder=(if branch == "chart" { "Chart default" } else { "8080" })
                                        min="1"
                                        max="65535">
                                    if branch == "chart" && port.is_empty() {
                                        <p class="text-xs">"Defined by the chart."</p>
                                    } else if branch != "image" && port.is_empty() {
                                        <p class="text-xs">
                                            "Platform managed; the app receives it in "
                                            <code>"PORT"</code>
                                            "."
                                        </p>
                                    }
                                </div>
                                <div class="space-y-2">
                                    <label class="label" for="settings-exposure">"Visibility"</label>
                                    if supports_public_access {
                                        <select class="select w-full" id="settings-exposure"
                                                name="exposure"
                                                data-network-exposure=""><option value="Private" selected=(!public_access)>"Private"</option>
                                            <option value="Public" selected=(public_access)>"Public"</option></select>
                                    } else {
                                        <input type="hidden" name="exposure" value="Private">
                                        <p class="flex h-9 items-center text-sm">"Private"</p>
                                    }
                                </div>
                            </div>

                            <div class="grid gap-x-4 gap-y-2 text-xs sm:grid-cols-[7rem_minmax(0,1fr)]">
                                <span>"Private endpoint"</span>
                                <code class="break-all">(private_endpoint)</code>
                            </div>

                            if supports_web_domains {
                                <div
                                    data-public-network-fields=""
                                    hidden=(!public_access)
                                    class="space-y-3"
                                >
                                    <div class="grid gap-x-4 gap-y-2 text-xs sm:grid-cols-[7rem_minmax(0,1fr)]">
                                        <span>"Domain"</span>
                                        <code class="break-all">(managed_domain)</code>
                                    </div>
                                    <details class="group" open=(!domain.is_empty())>
                                        <summary class="flex w-fit cursor-pointer list-none items-center gap-1 text-xs font-medium outline-none [&::-webkit-details-marker]:hidden">
                                            if domain.is_empty() {
                                                "Use a custom domain"
                                            } else {
                                                "Custom domain"
                                            }
                                            icon(
                                                data: iconify_icon!("feather:chevron-down"),
                                                attrs: attributes! { class="size-3.5 group-open:rotate-180" }
                                            )
                                        </summary>
                                        if tenant.domains.is_empty() {
                                            <div class="mt-3 max-w-sm px-3 py-3">
                                                <p class="text-xs">
                                                    "Register and verify a tenant domain before assigning a custom hostname."
                                                </p>
                                                <a
                                                    href=(format!("{}/settings", tenant_path(tenant_slug)))
                                                    class="mt-2 inline-block text-xs font-medium"
                                                >
                                                    "Register a domain"
                                                </a>
                                            </div>
                                        } else {
                                            <div class="mt-3 max-w-sm space-y-2">
                                                <label class="label" for="settings-domain">"Hostname"</label>
                                                <input class="input w-full" id="settings-domain"
                                                    name="domain"
                                                    value=(domain.as_str())
                                                    placeholder=(format!("app.{}", tenant.domains[0]))
                                                    inputmode="url"
                                                    autocomplete="url"
                                                    data-custom-domain=""
                                                    data-registered-domains=(tenant.domains.join(","))
                                                    pattern="[A-Za-z0-9](?:[A-Za-z0-9\\-]{0,61}[A-Za-z0-9])?(?:[.][A-Za-z0-9](?:[A-Za-z0-9\\-]{0,61}[A-Za-z0-9])?)+"
                                                    title="Enter a hostname under a verified tenant domain."
                                                    disabled=(!public_access)>
                                                <p class="text-xs">
                                                    "Verified domains: "
                                                    (tenant.domains.join(", "))
                                                </p>
                                            </div>
                                        }
                                    </details>
                                </div>
                            } else if protocol == "UDP" {
                                <p class="text-xs">"Public access provisions a UDP address; custom domains are unavailable."</p>
                            }
                        </section>
                        <div class="divider"></div>
                        <section data-variable-editor="" class="space-y-4" aria-labelledby="variables-heading">
                            <div class="flex flex-wrap items-start justify-between gap-3">
                                <div>
                                    <h4 id="variables-heading" class="font-medium">"Environment variables"</h4>
                                    <p class="mt-1 text-xs">"Values are encrypted and cannot be viewed after saving."</p>
                                </div>
                                <button
                                    type="button"
                                    data-add-variable=""
                                    class="btn btn-outline btn-sm"
                                >
                                    icon(data: iconify_icon!("feather:plus"), attrs: attributes! { class="size-3.5" })
                                    "Add variable"
                                </button>
                            </div>
                            <div data-variable-rows="" class="overflow-hidden empty:hidden">
                                for (variable_index, variable) in variables.iter().enumerate() {
                                    let value_id = format!("variable-value-{variable_index}");
                                    let variable_key = variable.key.as_str();
                                    <div data-variable-row="" data-variable-existing="" class="px-3 py-3">
                                        <input type="hidden" data-variable-key="" value=(variable_key)>
                                        <div class="flex flex-wrap items-center justify-between gap-3">
                                            <div class="min-w-0">
                                                <code class="block truncate text-sm font-medium">(variable_key)</code>
                                                <span class="mt-1 flex items-center gap-2 text-xs">
                                                    <span class="font-mono tracking-wider" aria-label="Value hidden">"••••••••"</span>
                                                    <span class="status status-success"></span>
                                                    "Encrypted value set"
                                                </span>
                                            </div>
                                            <div class="flex items-center gap-1">
                                                <button class="btn btn-ghost btn-sm" type="button"
                                                        data-edit-variable=""
                                                        aria-controls=(value_id.clone())
                                                        aria-expanded="false">"Replace"</button>
                                                <button class="btn btn-ghost btn-sm" type="button"
                                                        data-remove-variable=""
                                                        aria-label=(format!("Remove {variable_key}"))>"Remove"</button>
                                            </div>
                                        </div>
                                        <div
                                            data-variable-value-fields=""
                                            hidden=(true)
                                            class="mt-3 max-w-md space-y-1.5"
                                        >
                                            <label for=(value_id.clone()) class="text-xs font-medium">"New value"</label>
                                            <input class="input w-full" id=(value_id)
                                                data-variable-value=""
                                                type="password"
                                                placeholder="Enter replacement value"
                                                autocomplete="new-password"
                                                aria-label=(format!("New value for {variable_key}"))
                                                required=(true)
                                                disabled=(true)>
                                            <p class="text-xs">"The current value cannot be viewed."</p>
                                        </div>
                                    </div>
                                }
                            </div>
                            <template data-variable-template="">
                                <div data-variable-row="" data-variable-new="" class="grid gap-3 px-3 py-3 sm:grid-cols-[minmax(10rem,1fr)_minmax(12rem,1.4fr)_auto] sm:items-end">
                                    <label class="space-y-1.5 text-xs font-medium">
                                        <span class="block text-xs">"Key"</span>
                                        <input class="input w-full" data-variable-key=""
                                            placeholder="VARIABLE_NAME"
                                            aria-label="Variable key"
                                            pattern="[A-Za-z_][A-Za-z0-9_]*"
                                            title="Use letters, numbers, and underscores; do not start with a number."
                                            required=(true)>
                                    </label>
                                    <label class="space-y-1.5 text-xs font-medium">
                                        <span class="block text-xs">"Value"</span>
                                        <input class="input w-full" data-variable-value=""
                                            type="password"
                                            placeholder="Required"
                                            aria-label="Variable value"
                                            autocomplete="new-password"
                                            required=(true)>
                                    </label>
                                    <button class="btn btn-ghost btn-sm" type="button"
                                            data-remove-variable=""
                                            aria-label="Remove variable">"Remove"</button>
                                </div>
                            </template>
                            <p data-variable-status="" class="sr-only" aria-live="polite"></p>
                        </section>
                        <div class="divider"></div>
                        <section
                            id="storage"
                            data-storage-config=""
                            class="space-y-4"
                            aria-labelledby="component-storage-heading"
                        >
                            <div class="flex flex-wrap items-start justify-between gap-4">
                                <div>
                                    <h4 id="component-storage-heading" class="font-medium">"Storage"</h4>
                                    <p class="mt-1 text-xs">"Data persists across deploys and restarts."</p>
                                </div>
                                match storage {
                                    Some(storage) => <div class="flex items-center gap-2">
                                        if !matches!(storage.state, "Ready" | "Attached") {
                                            <span class="badge badge-outline">(storage.state)</span>
                                        }
                                        <button class="btn btn-ghost btn-sm" type="submit"
                                                form="delete-component-storage">"Delete storage"</button>
                                    </div>,
                                    None => if storage_requested {
                                        <span class="badge badge-ghost">"Provisioning"</span>
                                    } else {
                                        <button class="btn btn-outline btn-sm" type="button" data-add-storage="">"Add storage"</button>
                                    },
                                }
                            </div>

                            match storage {
                                Some(storage) => <div class="max-w-2xl space-y-2">
                                    <div class="flex items-baseline justify-between gap-3 text-xs">
                                        <span class="font-medium">
                                            (format!("{:.1} of {} GiB used", storage.used_gib, storage.capacity_gib))
                                        </span>
                                        <span>(format!("{storage_usage_percent:.0}%"))</span>
                                    </div>
                                    <progress class="progress progress-primary w-full" value=(storage_usage_percent) max=(100.0) aria-label="Storage usage"></progress>
                                </div>,
                                None => "",
                            }

                            if storage_requested {
                                <input type="hidden" name="storage_enabled" value="on">
                            } else {
                                <input
                                    type="checkbox"
                                    name="storage_enabled"
                                    data-storage-toggle=""
                                    hidden=(true)
                                >
                            }
                            <div
                                data-storage-fields=""
                                hidden=(!storage_requested)
                                class="grid gap-4 sm:grid-cols-3"
                            >
                                <div class="space-y-2">
                                    <label class="label" for="settings-storage-size">"Capacity (GiB)"</label>
                                    <input class="input w-full" id="settings-storage-size"
                                        name="size"
                                        type="number"
                                        value=(storage_size)
                                        min=(storage.map_or(1, |storage| storage.capacity_gib))
                                        required=(true)
                                        disabled=(!storage_requested)>
                                    if storage.is_some() {
                                        <p class="text-xs">"Can only be increased."</p>
                                    }
                                </div>
                                <div class="space-y-2">
                                    <label class="label" for="settings-mount-path">"Mount path"</label>
                                    <input class="input w-full" id="settings-mount-path"
                                        name="mount_path"
                                        value=(storage_mount_path)
                                        pattern="/.*"
                                        title="Use an absolute path beginning with /."
                                        required=(true)
                                        disabled=(!storage_requested)>
                                    if storage.is_some() {
                                        <p class="text-xs">"Changing this redeploys the component."</p>
                                    }
                                </div>
                                <div class="space-y-2">
                                    <label class="label" for="settings-backup-policy">"Backups"</label>
                                    <select class="select w-full" id="settings-backup-policy"
                                            name="backup_policy"
                                            disabled=(!storage_requested)><option value="Disabled" selected=(storage_backup_policy == "Disabled")>"Disabled"</option>
                                        <option value="Daily · retain 7" selected=(storage_backup_policy == "Daily · retain 7")>"Daily · keep 7"</option>
                                        <option value="Daily · retain 14" selected=(storage_backup_policy == "Daily · retain 14")>"Daily · keep 14"</option>
                                        <option value="Weekly · retain 4" selected=(storage_backup_policy == "Weekly · retain 4")>"Weekly · keep 4"</option></select>
                                    match latest_backup {
                                        Some(backup) => <p
                                            class="flex flex-wrap gap-x-2 text-xs"
                                            title=(format!("{} · {}", backup.id, backup.state))
                                        >
                                            <span>"Last: " (backup.created_at)</span>
                                            <span>(backup.size)</span>
                                        </p>,
                                        None => "",
                                    }
                                </div>
                            </div>
                        </section>
                        <div class="flex flex-wrap items-center justify-between gap-3 pt-5">
                            <p data-settings-status="" aria-live="polite" class="text-xs">
                                "No unsaved changes"
                            </p>
                            <button class="btn btn-primary" type="submit" data-settings-submit="">"Save component settings"</button>
                        </div>
                    </form>
                    match storage {
                        Some(storage) => <form
                            id="delete-component-storage"
                            action=(action)
                            method="get"
                            data-confirm="Permanently delete this component’s storage and backups? This cannot be undone."
                        >
                            <input type="hidden" name="action" value="remove-volume">
                            <input type="hidden" name="volume" value=(storage.slug)>
                        </form>,
                        None => "",
                    }</div></div>

            <div class="card bg-base-100"><div class="card-body"><h3 class="card-title">"Delete component"</h3>
                    <p>"This stops the workload and removes the component."</p>if !dependent_components.is_empty() {
                        deletion_dependencies(
                            tenant_slug: tenant_slug.to_owned(),
                            project_slug: project_slug.to_owned(),
                            environment_slug: environment_slug.to_owned(),
                            dependents: dependent_components,
                        )
                    } else if storage_requested {
                        <div class="alert alert-warning">
                            icon(data: iconify_icon!("feather:hard-drive"), attrs: attributes! {
                                aria-hidden="true"
                                class="mt-0.5 size-4 shrink-0"
                            })
                            <div>
                                <p class="font-medium">"Storage is still attached"</p>
                                <p class="mt-1 text-xs">
                                    "Delete the component’s storage and backups before deleting the component."
                                </p>
                                <a href="#storage" class="mt-2 inline-block text-xs font-medium">
                                    "Go to storage"
                                </a>
                            </div>
                        </div>
                    } else {
                        <details class="group">
                            <summary
                                class="btn btn-error"
                            >
                                "Delete component"
                                icon(
                                    data: iconify_icon!("feather:chevron-down"),
                                    attrs: attributes! {
                                        aria-hidden="true"
                                        class="size-3.5 group-open:rotate-180"
                                    }
                                )
                            </summary>
                            <form
                                action=(environment_action)
                                method="get"
                                class="mt-5 pt-5"
                                data-confirm="Delete this component from the mock environment?"
                            >
                                <input type="hidden" name="action" value="delete-component">
                                <input type="hidden" name="component" value=(component.slug)>
                                <button class="btn btn-error" type="submit">"Confirm deletion"</button>
                            </form>
                        </details>
                    }</div></div>
        </div>
    }
}

#[page(
    "/tenants/{tenant}/projects/{project}/environments/{environment}/components/{component}/changes"
)]
async fn component_changes(cx: &Cx) -> Result {
    let tenant_slug = path_param::<Tenant>(cx);
    let project_slug = path_param::<Project>(cx);
    let environment_slug = path_param::<Environment>(cx);
    let component_slug = path_param::<Component>(cx);
    let initial_component = current_component(cx)?;
    let query = query_params::<UiQuery>(cx)?;
    let action = uri(cx).path();
    let mutation_error = if query.action.as_deref() == Some("apply-change") {
        let summary = if setting_value(initial_component, "Branch") == "image" {
            "Apply latest container image"
        } else {
            "Reconcile latest source revision"
        };
        match mock::record_component_change(
            tenant_slug,
            project_slug,
            environment_slug,
            component_slug,
            summary,
        ) {
            Ok(_) => {
                let destination = format!("{action}?notice=change-applied");
                return Err(redirect(&destination).into());
            }
            Err(error) => Some(error),
        }
    } else {
        None
    };
    let component = current_component(cx)?;
    let branch = setting_value(component, "Branch");
    let image_backed = branch == "image";
    let change_description = if image_backed {
        "Pull the configured image and update this environment."
    } else {
        "Build the tracked branch and update this environment."
    };
    let reference_label = if image_backed { "Delivery" } else { "Branch" };
    let reference_value = if image_backed {
        "Container image"
    } else {
        branch
    };

    view! {
        <div class="space-y-6">
            feedback_banner(message: mutation_error, is_error: true)
            <div class="card bg-base-100"><div class="card-body"><h3 class="card-title">"Apply latest revision"</h3>
                    <p>(change_description)</p><dl class="mb-5 grid gap-4 text-sm sm:grid-cols-2">
                        <div>
                            <dt>"Source"</dt>
                            <dd class="mt-1 font-mono">(setting_value(component, "Source"))</dd>
                        </div>
                        <div>
                            <dt>(reference_label)</dt>
                            <dd class="mt-1 font-mono">(reference_value)</dd>
                        </div>
                    </dl>
                    <form action=(action) method="get">
                        <input type="hidden" name="action" value="apply-change">
                        <input type="hidden" name="component" value=(component.name)>
                        <button class="btn btn-primary" type="submit">icon(data: iconify_icon!("feather:upload-cloud"), attrs: attributes! { class="size-4" })
                            "Apply change"</button>
                    </form></div></div>

            <section class="card bg-base-100" aria-labelledby="component-changes-heading">
                <header class="flex h-11 items-center justify-between px-4">
                    <h2 id="component-changes-heading" class="text-sm font-semibold">"Changes"</h2>
                    <span class="text-xs">
                        (count_label(component.changes.len(), "change", "changes"))
                    </span>
                </header>
                if component.changes.is_empty() {
                    <p class="px-4 py-8 text-center text-sm">"No changes recorded."</p>
                } else {
                    change_list(changes: component.changes)
                }
            </section>
        </div>
    }
}

#[page("/tenants/{tenant}/settings")]
async fn tenant_settings(cx: &Cx) -> Result {
    let tenant_slug = path_param::<Tenant>(cx);
    let initial_tenant = mock::tenant(tenant_slug).ok_or_not_found()?;
    let query = query_params::<UiQuery>(cx)?;
    let mutation_error = match query.action.as_deref() {
        Some("save-tenant") => {
            match mock::update_tenant_name(tenant_slug, query.display_name.as_deref().unwrap_or(""))
            {
                Ok(_) => {
                    let destination = format!(
                        "{}/settings?notice=tenant-updated",
                        tenant_path(tenant_slug),
                    );
                    return Err(redirect(&destination).into());
                }
                Err(error) => Some(error),
            }
        }
        Some("add-domain") => {
            let domain = normalize_hostname(query.domain.as_deref().unwrap_or(""));
            match mock::add_tenant_domain(tenant_slug, &domain) {
                Ok(_) => {
                    let destination = format!(
                        "{}/settings?notice=domain-registered&domain={domain}",
                        tenant_path(tenant_slug),
                    );
                    return Err(redirect(&destination).into());
                }
                Err(error) => Some(error),
            }
        }
        Some("remove-domain") => {
            let domain = query.domain.as_deref().unwrap_or("");
            if domain_assignment_count(initial_tenant, domain) > 0 {
                Some("Remove assigned component hostnames before removing this domain.".to_owned())
            } else {
                match mock::remove_tenant_domain(tenant_slug, domain) {
                    Ok(_) => {
                        let destination = format!(
                            "{}/settings?notice=domain-removed",
                            tenant_path(tenant_slug),
                        );
                        return Err(redirect(&destination).into());
                    }
                    Err(error) => Some(error),
                }
            }
        }
        _ => None,
    };
    let tenant = mock::tenant(tenant_slug).ok_or_not_found()?;
    let feedback_is_error = mutation_error.is_some();
    let feedback = mutation_error.or_else(|| feedback_message(query));
    let display_name = tenant.name;
    let added_domain = (query.notice.as_deref() == Some("domain-registered"))
        .then_some(query.domain.as_deref())
        .flatten();
    let action = uri(cx).path();

    view! {
        <main class="p-6">
            <nav class="mb-5 flex items-center gap-1.5 text-xs" aria-label="Breadcrumb">
                <a href=(tenant_path(tenant.slug))>"Projects"</a>
                icon(data: iconify_icon!("feather:chevron-right"), attrs: attributes! { class="size-3" })
                <span aria-current="page">"Settings"</span>
            </nav>
            <header class="mb-6">
                <h1 class="text-2xl font-semibold">"Tenant settings"</h1>
            </header>
            feedback_banner(message: feedback, is_error: feedback_is_error)
            <div class="grid items-start gap-6 lg:grid-cols-2">
                <div class="card bg-base-100"><div class="card-body"><h3 class="card-title">"General"</h3>
                        <p>"Tenant identity and source of truth."</p><form action=(action) method="get" class="space-y-4" data-settings-form="">
                            <input type="hidden" name="action" value="save-tenant">
                            <div class="space-y-2">
                                <label class="label" for="tenant-display-name">"Display name"</label>
                                <input class="input w-full" id="tenant-display-name" name="display_name" value=(display_name)>
                            </div>
                            <div class="space-y-2">
                                <label class="label" for="tenant-current-slug">"Slug"</label>
                                <input class="input w-full" id="tenant-current-slug" value=(tenant.slug) disabled=(true)>
                            </div>
                            <div class="flex items-center justify-between gap-3 pt-4">
                                <p data-settings-status="" aria-live="polite" class="text-xs">
                                    "No unsaved changes"
                                </p>
                                <button class="btn btn-primary" type="submit" data-settings-submit="">"Save changes"</button>
                            </div>
                        </form></div></div>
                <div class="card bg-base-100"><div class="card-body"><h3 class="card-title">"Registered domains"</h3>
                        <p>"Use verified domains for custom component hostnames."</p><ul class="text-sm">
                            for domain in tenant.domains {
                                let assignments = domain_assignment_count(tenant, domain);
                                let is_pending = added_domain == Some(*domain);
                                <li class="grid gap-3 py-3 first:pt-0 sm:grid-cols-[minmax(0,1fr)_auto] sm:items-center">
                                    <span class="min-w-0">
                                        <span class="flex flex-wrap items-center gap-2">
                                            <code class="break-all">(domain)</code>
                                            if is_pending {
                                                <span class="badge badge-outline">"Pending DNS"</span>
                                            } else {
                                                <span class="badge badge-ghost">"Verified"</span>
                                            }
                                        </span>
                                        <span class="mt-1 block text-xs">
                                            "CNAME target "
                                            <code>"edge.netamos.app"</code>
                                            if assignments > 0 {
                                                (format!(
                                                    " · {}",
                                                    count_label(assignments, "hostname", "hostnames"),
                                                ))
                                            }
                                        </span>
                                    </span>
                                    <form
                                        action=(action)
                                        method="get"
                                        data-confirm=(format!("Remove {domain} from this tenant?"))
                                    >
                                        <input type="hidden" name="action" value="remove-domain">
                                        <input type="hidden" name="domain" value=(domain)>
                                        <button class="btn btn-ghost btn-sm" type="submit"
                                                disabled=(assignments > 0)
                                                title=(if assignments > 0 {
                                                    "Remove assigned hostnames first"
                                                } else {
                                                    "Remove registered domain"
                                                })>"Remove"</button>
                                    </form>
                                </li>
                            }
                        </ul>
                        <form action=(action) method="get" class="mt-5 space-y-3">
                            <input type="hidden" name="action" value="add-domain">
                            <div class="space-y-2">
                                <label class="label" for="new-domain">"Register a domain"</label>
                                <input class="input w-full" id="new-domain"
                                    name="domain"
                                    placeholder="example.com"
                                    required=(true)
                                    pattern="[A-Za-z0-9](?:[A-Za-z0-9\\-]{0,61}[A-Za-z0-9])?(?:[.][A-Za-z0-9](?:[A-Za-z0-9\\-]{0,61}[A-Za-z0-9])?)+"
                                    title="Enter a base domain such as example.com.">
                                <p class="text-xs">
                                    "After registration, point it to "
                                    <code>"edge.netamos.app"</code>
                                    " to verify ownership."
                                </p>
                            </div>
                            <div class="flex justify-end">
                                <button class="btn btn-primary" type="submit">icon(data: iconify_icon!("feather:plus"), attrs: attributes! { class="size-4" })
                                    "Register domain"</button>
                            </div>
                        </form></div></div>
            </div>
        </main>
    }
}

struct ComponentVariableState {
    key: String,
}

fn component_variables(component: &mock::Component) -> &'static [&'static str] {
    component.variables
}

fn component_variable_states(component: &mock::Component) -> Vec<ComponentVariableState> {
    component_variables(component)
        .iter()
        .map(|key| ComponentVariableState {
            key: (*key).to_owned(),
        })
        .collect()
}

fn parse_component_variables(serialized: &str) -> Vec<ComponentVariableState> {
    let mut variables = Vec::new();

    for line in serialized.lines() {
        let key = line.split('\t').next().unwrap_or_default().trim();
        let mut chars = key.chars();
        let valid_start = chars
            .next()
            .is_some_and(|character| character == '_' || character.is_ascii_alphabetic());
        let valid_rest =
            chars.all(|character| character == '_' || character.is_ascii_alphanumeric());

        if valid_start
            && valid_rest
            && !variables
                .iter()
                .any(|variable: &ComponentVariableState| variable.key == key)
        {
            variables.push(ComponentVariableState {
                key: key.to_owned(),
            });
        }
    }

    variables
}

fn managed_domain(tenant: &str, project: &str, environment: &str, component: &str) -> String {
    fn dns_label(value: &str, max_len: usize) -> String {
        let mut normalized_label = String::new();
        let mut previous_was_dash = false;

        for character in value.chars() {
            let normalized = if character.is_ascii_alphanumeric() {
                character.to_ascii_lowercase()
            } else {
                '-'
            };

            if normalized == '-' {
                if normalized_label.is_empty() || previous_was_dash {
                    continue;
                }
                previous_was_dash = true;
            } else {
                previous_was_dash = false;
            }

            normalized_label.push(normalized);
            if normalized_label.len() == max_len {
                break;
            }
        }

        while normalized_label.ends_with('-') {
            normalized_label.pop();
        }

        if normalized_label.is_empty() {
            "app".to_owned()
        } else {
            normalized_label
        }
    }

    let tenant = dns_label(tenant, 63);
    let project = dns_label(project, 63);
    let environment = dns_label(environment, 20);
    let component = dns_label(component, 24);
    let identity = format!("{tenant}/{project}/{environment}/{component}");
    let hash = identity.bytes().fold(2_166_136_261_u32, |hash, byte| {
        (hash ^ u32::from(byte)).wrapping_mul(16_777_619)
    });

    format!(
        "{component}-{environment}-{:06x}.netamos.app",
        hash & 0x00ff_ffff
    )
}

fn normalize_hostname(value: &str) -> String {
    let hostname = value.trim().trim_end_matches('.').to_ascii_lowercase();
    let labels = hostname.split('.').collect::<Vec<_>>();
    let is_valid = hostname.len() <= 253
        && labels.len() >= 2
        && labels.iter().all(|dns_part| {
            !dns_part.is_empty()
                && dns_part.len() <= 63
                && dns_part
                    .bytes()
                    .all(|byte| byte.is_ascii_alphanumeric() || byte == b'-')
                && dns_part
                    .as_bytes()
                    .first()
                    .is_some_and(u8::is_ascii_alphanumeric)
                && dns_part
                    .as_bytes()
                    .last()
                    .is_some_and(u8::is_ascii_alphanumeric)
        });

    if is_valid { hostname } else { String::new() }
}

fn setting_value(component: &'static mock::Component, setting_label: &str) -> &'static str {
    component
        .settings
        .iter()
        .find(|setting| setting.label == setting_label)
        .map_or("", |setting| setting.value)
}

fn count_label(count: usize, singular: &str, plural: &str) -> String {
    format!("{count} {}", if count == 1 { singular } else { plural })
}

fn project_search_text(project: &mock::Project) -> String {
    let mut terms = vec![project.name, project.description];
    for environment in project.environments {
        terms.push(environment.name);
        terms.push(environment.region);
        for component in environment.components {
            terms.push(component.name);
            terms.push(component.kind);
        }
    }
    terms.join(" ").to_lowercase()
}

fn project_matches_filter(project: &mock::Project, filter: &str) -> bool {
    filter.is_empty() || project_search_text(project).contains(filter)
}

fn change_destination(tenant: &str, change: &mock::Change) -> String {
    format!(
        "{}/changes",
        component_path(
            tenant,
            change.target.project_slug,
            change.target.environment_slug,
            change.target.component_slug,
        )
    )
}

fn tenant_usage_totals(tenant: &mock::Tenant) -> (f32, f32, f32) {
    tenant
        .projects
        .iter()
        .fold((0.0, 0.0, 0.0), |(compute, memory, egress), project| {
            (
                compute + project.usage.compute_vcpu_hours,
                memory + project.usage.memory_gib_hours,
                egress + project.usage.egress_gb,
            )
        })
}

fn domain_assignment_count(tenant: &mock::Tenant, registered_domain: &str) -> usize {
    tenant
        .projects
        .iter()
        .flat_map(|project| project.environments)
        .flat_map(|environment| environment.components)
        .filter_map(|component| {
            component
                .settings
                .iter()
                .find(|setting| setting.label == "Domain")
                .map(|setting| setting.value.trim().trim_end_matches('.'))
        })
        .filter(|hostname| {
            *hostname == registered_domain
                || hostname
                    .strip_suffix(registered_domain)
                    .is_some_and(|prefix| prefix.ends_with('.'))
        })
        .count()
}

fn region_label(region: Option<&str>) -> &'static str {
    match region {
        Some("saigon") => "Saigon",
        _ => "Helsinki",
    }
}

fn numeric_setting(value: &str, fallback: u32) -> u32 {
    value
        .split_whitespace()
        .next()
        .and_then(|value| value.parse::<u32>().ok())
        .unwrap_or(fallback)
}

fn component_kind_label(kind: Option<&str>) -> &'static str {
    match kind {
        Some("cron-job") => "Cron job",
        Some("postgresql") => "Managed PostgreSQL",
        Some("valkey") => "Managed Valkey",
        _ => "Application",
    }
}

fn new_component_from_query(query: &UiQuery, tenant: &mock::Tenant) -> mock::NewComponent {
    let kind = component_kind_label(query.kind.as_deref()).to_owned();
    let variables = query
        .variables
        .as_deref()
        .map(parse_component_variables)
        .unwrap_or_default()
        .into_iter()
        .map(|variable| variable.key)
        .collect::<Vec<_>>();
    let domain = query
        .domain
        .as_deref()
        .map(normalize_hostname)
        .filter(|hostname| {
            !hostname.is_empty()
                && tenant.domains.iter().any(|registered_domain| {
                    hostname == *registered_domain
                        || hostname
                            .strip_suffix(*registered_domain)
                            .is_some_and(|prefix| prefix.ends_with('.'))
                })
        });
    let volume = query.storage_enabled.as_ref().map(|_| mock::NewVolume {
        capacity_gib: query
            .size
            .as_deref()
            .and_then(|size| size.parse::<u32>().ok())
            .unwrap_or(10),
        mount_path: query
            .mount_path
            .clone()
            .unwrap_or_else(|| "/data".to_owned()),
        backup_policy: query
            .backup_policy
            .clone()
            .unwrap_or_else(|| "Disabled".to_owned()),
    });
    let mut settings = Vec::new();

    match query.kind.as_deref() {
        Some("cron-job") => {
            settings.push((
                "Command".to_owned(),
                query.command.clone().unwrap_or_default(),
            ));
            settings.push((
                "Schedule".to_owned(),
                query.cron.clone().unwrap_or_default(),
            ));
            settings.push((
                "Timezone".to_owned(),
                query.timezone.clone().unwrap_or_else(|| "UTC".to_owned()),
            ));
        }
        Some("postgresql") => {
            settings.push((
                "Version".to_owned(),
                query.version.clone().unwrap_or_else(|| "17".to_owned()),
            ));
            settings.push((
                "Compute".to_owned(),
                match query.plan.as_deref() {
                    Some("standard-2") => "Dedicated 2 vCPU · 4 GiB",
                    Some("standard-4") => "Dedicated 4 vCPU · 8 GiB",
                    _ => "Shared 1 vCPU · 1 GiB",
                }
                .to_owned(),
            ));
            settings.push((
                "Storage".to_owned(),
                format!("{} GiB", query.storage.as_deref().unwrap_or("20")),
            ));
            settings.push((
                "Backup".to_owned(),
                if query.backups.is_some() {
                    "Daily · retain 7"
                } else {
                    "Disabled"
                }
                .to_owned(),
            ));
            settings.push((
                "High availability".to_owned(),
                if query.high_availability.is_some() {
                    "Enabled"
                } else {
                    "Disabled"
                }
                .to_owned(),
            ));
        }
        Some("valkey") => {
            settings.push((
                "Version".to_owned(),
                query.version.clone().unwrap_or_else(|| "8".to_owned()),
            ));
            settings.push((
                "Memory".to_owned(),
                match query.memory.as_deref() {
                    Some("1gb") => "1 GiB",
                    Some("2gb") => "2 GiB",
                    _ => "512 MiB",
                }
                .to_owned(),
            ));
            settings.push((
                "Persistence".to_owned(),
                query
                    .persistence
                    .clone()
                    .unwrap_or_else(|| "AOF every second".to_owned()),
            ));
            settings.push((
                "Eviction".to_owned(),
                query
                    .eviction
                    .clone()
                    .unwrap_or_else(|| "noeviction".to_owned()),
            ));
        }
        _ => {
            settings.push((
                "Replicas".to_owned(),
                query.replicas.clone().unwrap_or_else(|| "1".to_owned()),
            ));
        }
    }

    mock::NewComponent {
        name: query.name.clone().unwrap_or_default(),
        kind,
        source: query.source.clone().unwrap_or_default(),
        source_kind: query.source_kind.clone().unwrap_or_else(|| {
            if matches!(query.kind.as_deref(), Some("postgresql" | "valkey")) {
                "managed".to_owned()
            } else {
                "repository".to_owned()
            }
        }),
        visibility: query
            .exposure
            .clone()
            .unwrap_or_else(|| "Private".to_owned()),
        domain,
        port: query
            .port
            .as_deref()
            .and_then(|port| port.parse::<u16>().ok()),
        variables,
        volume,
        settings,
    }
}

fn component_settings_update_from_query(
    component: &mock::Component,
    query: &UiQuery,
    tenant: &mock::Tenant,
    has_storage: bool,
) -> mock::ComponentSettingsUpdate {
    let mut settings = Vec::new();

    match component.kind {
        "Managed PostgreSQL" => {
            settings.push((
                "Version".to_owned(),
                query.version.clone().unwrap_or_else(|| "16".to_owned()),
            ));
            settings.push((
                "Compute".to_owned(),
                query
                    .plan
                    .clone()
                    .unwrap_or_else(|| "Shared 1 vCPU · 1 GiB".to_owned()),
            ));
            settings.push((
                "Storage".to_owned(),
                format!("{} GiB", query.size.as_deref().unwrap_or("20")),
            ));
            settings.push((
                "Backup".to_owned(),
                query
                    .backup_policy
                    .clone()
                    .unwrap_or_else(|| "Daily · retain 7".to_owned()),
            ));
            settings.push((
                "High availability".to_owned(),
                if query.high_availability.is_some() {
                    "Enabled"
                } else {
                    "Disabled"
                }
                .to_owned(),
            ));
        }
        "Managed Valkey" => {
            settings.push((
                "Version".to_owned(),
                query.version.clone().unwrap_or_else(|| "8".to_owned()),
            ));
            settings.push((
                "Memory".to_owned(),
                query.plan.clone().unwrap_or_else(|| "512 MiB".to_owned()),
            ));
            settings.push((
                "Persistence".to_owned(),
                query
                    .persistence
                    .clone()
                    .unwrap_or_else(|| "AOF every second".to_owned()),
            ));
            settings.push((
                "Eviction".to_owned(),
                query
                    .eviction
                    .clone()
                    .unwrap_or_else(|| "noeviction".to_owned()),
            ));
        }
        _ => {
            settings.push((
                "Source".to_owned(),
                query.source.clone().unwrap_or_default(),
            ));
            settings.push((
                "Replicas".to_owned(),
                query.replicas.clone().unwrap_or_else(|| "1".to_owned()),
            ));
            settings.push(("Port".to_owned(), query.port.clone().unwrap_or_default()));
            let visibility = query
                .exposure
                .clone()
                .unwrap_or_else(|| "Private".to_owned());
            settings.push(("Visibility".to_owned(), visibility.clone()));
            let domain = if visibility == "Public" {
                query
                    .domain
                    .as_deref()
                    .map(normalize_hostname)
                    .filter(|hostname| {
                        tenant.domains.iter().any(|registered_domain| {
                            hostname == *registered_domain
                                || hostname
                                    .strip_suffix(*registered_domain)
                                    .is_some_and(|prefix| prefix.ends_with('.'))
                        })
                    })
                    .unwrap_or_default()
            } else {
                String::new()
            };
            settings.push(("Domain".to_owned(), domain));
            settings.push((
                "Auto deploy".to_owned(),
                if query.auto_deploy.is_some() {
                    "Enabled"
                } else {
                    "Disabled"
                }
                .to_owned(),
            ));
        }
    }

    let volume =
        (query.storage_enabled.is_some() || has_storage).then(|| mock::VolumeSettingsUpdate {
            capacity_gib: query
                .size
                .as_deref()
                .and_then(|size| size.parse::<u32>().ok())
                .unwrap_or(10),
            mount_path: query
                .mount_path
                .clone()
                .unwrap_or_else(|| "/data".to_owned()),
            backup_policy: query
                .backup_policy
                .clone()
                .unwrap_or_else(|| "Disabled".to_owned()),
        });
    let variables = if matches!(component.kind, "Managed PostgreSQL" | "Managed Valkey") {
        None
    } else {
        Some(
            query
                .variables
                .as_deref()
                .map(parse_component_variables)
                .unwrap_or_default()
                .into_iter()
                .map(|variable| variable.key)
                .collect(),
        )
    };

    mock::ComponentSettingsUpdate {
        settings,
        variables,
        volume,
    }
}

fn feedback_message(query: &UiQuery) -> Option<String> {
    if let Some(error) = query.error.as_deref() {
        return Some(error.to_owned());
    }

    if let Some(notice) = query.notice.as_deref() {
        return match notice {
            "project-created" => Some("Project created.".to_owned()),
            "environment-created" => Some("Environment created.".to_owned()),
            "component-created" => {
                Some("Component created. Reconciliation has started.".to_owned())
            }
            "component-updated" => Some("Component settings saved.".to_owned()),
            "component-deleted" => Some("Component deleted.".to_owned()),
            "storage-deleted" => Some("Storage and backups permanently deleted.".to_owned()),
            "tenant-updated" => Some("Tenant settings saved.".to_owned()),
            "domain-registered" => {
                Some("Domain registered. DNS verification is pending.".to_owned())
            }
            "domain-removed" => Some("Registered domain removed.".to_owned()),
            "change-applied" => Some("Change queued for reconciliation.".to_owned()),
            "component-unavailable" => query.component.as_deref().map(|component| {
                format!("{component} is not deployed in this environment. Showing all components.")
            }),
            _ => None,
        };
    }

    None
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

fn corresponding_environment(
    project: &'static mock::Project,
    current_environment_slug: Option<&str>,
) -> Option<&'static mock::Environment> {
    current_environment_slug
        .and_then(|slug| {
            project
                .environments
                .iter()
                .find(|environment| environment.slug == slug)
        })
        .or_else(|| (project.environments.len() == 1).then(|| &project.environments[0]))
}

fn project_context_destination(
    tenant_slug: &str,
    project: &'static mock::Project,
    current_environment_slug: Option<&str>,
) -> String {
    match current_environment_slug.and_then(|slug| corresponding_environment(project, Some(slug))) {
        Some(environment) => environment_path(tenant_slug, project.slug, environment.slug),
        None => project_path(tenant_slug, project.slug),
    }
}

fn environment_context_destination(
    tenant_slug: &str,
    project_slug: &str,
    environment: &'static mock::Environment,
    component: Option<&'static mock::Component>,
    component_suffix: &str,
) -> String {
    match component {
        Some(component)
            if environment
                .components
                .iter()
                .any(|candidate| candidate.slug == component.slug) =>
        {
            format!(
                "{}{component_suffix}",
                component_path(tenant_slug, project_slug, environment.slug, component.slug,)
            )
        }
        Some(component) => format!(
            "{}?notice=component-unavailable&component={}",
            environment_path(tenant_slug, project_slug, environment.slug),
            component.slug,
        ),
        None => environment_path(tenant_slug, project_slug, environment.slug),
    }
}

fn component_context_destination(
    tenant_slug: &str,
    project_slug: &str,
    environment_slug: &str,
    component_slug: &str,
    component_suffix: &str,
) -> String {
    format!(
        "{}{component_suffix}",
        component_path(tenant_slug, project_slug, environment_slug, component_slug,)
    )
}

#[topcoat::view::component]
async fn breadcrumb_separator() -> Result {
    view! {
        <span aria-hidden="true" class="shrink-0 px-0.5">"/"</span>
    }
}

#[topcoat::view::component]
async fn project_context_selector(
    tenant: &'static mock::Tenant,
    project: &'static mock::Project,
    current_environment_slug: Option<&'static str>,
) -> Result {
    view! {
        if tenant.projects.len() <= 1 {
            <span class="truncate">(project.name)</span>
        } else {
            <details class="dropdown" data-dropdown-menu="" data-project-switcher="">
                <summary class="btn btn-ghost btn-sm" aria-label=(format!("Switch project; current project {}", project.name))>
                    (project.name)
                    icon(data: iconify_icon!("feather:chevron-down"), attrs: attributes! {
                        aria-hidden="true"
                        class="size-3"
                    })
                </summary>
                <ul class="dropdown-content menu bg-base-100 rounded-box z-50 w-72 p-2 shadow-sm">
                    <li class="menu-title">"Switch project"</li>
                    if tenant.projects.len() >= 8 {
                        <li>
                            <label class="sr-only" for="project-switcher-search">"Find project"</label>
                            <input
                                id="project-switcher-search"
                                type="search"
                                autocomplete="off"
                                placeholder="Find project…"
                                data-project-switcher-search=""
                                class="input input-sm w-full"
                            >
                        </li>
                    }
                    for candidate in tenant.projects {
                        let candidate_target = project_context_destination(
                            tenant.slug,
                            candidate,
                            current_environment_slug,
                        );
                        let environment_count = candidate.environments.len();

                        <li>
                            <a
                                href=(candidate_target)
                                data-project-switcher-option=""
                                data-project-search=(format!("{} {}", candidate.name, candidate.slug).to_lowercase())
                                class=(if candidate.slug == project.slug { "menu-active" } else { "" })
                                aria-current=(if candidate.slug == project.slug { "true" } else { "false" })
                            >
                                if candidate.slug == project.slug {
                                    icon(data: iconify_icon!("feather:check"), attrs: attributes! {
                                        aria-hidden="true"
                                        class="size-4"
                                    })
                                }
                                <span class="truncate">(candidate.name)</span>
                                <span class="badge badge-sm">(count_label(environment_count, "env", "envs"))</span>
                            </a>
                        </li>
                    }
                    <li hidden="" data-project-switcher-empty="">"No projects found"</li>
                    <li>
                        <a href=(format!("{}/projects/new", tenant_path(tenant.slug)))>
                            icon(data: iconify_icon!("feather:plus"), attrs: attributes! {
                                aria-hidden="true"
                                class="size-4"
                            })
                            "New project"
                        </a>
                    </li>
                </ul>
            </details>
        }
    }
}

#[topcoat::view::component]
async fn environment_context_selector(
    tenant_slug: &'static str,
    project: &'static mock::Project,
    environment: &'static mock::Environment,
    component: Option<&'static mock::Component>,
    component_suffix: &'static str,
) -> Result {
    view! {
        if project.environments.len() <= 1 {
            <span class="truncate">(environment.name)</span>
        } else {
            <details class="dropdown" data-dropdown-menu="">
                <summary class="btn btn-ghost btn-sm" aria-label=(format!("Switch environment; current environment {}", environment.name))>
                    (environment.name)
                    icon(data: iconify_icon!("feather:chevron-down"), attrs: attributes! {
                        aria-hidden="true"
                        class="size-3"
                    })
                </summary>
                <ul class="dropdown-content menu bg-base-100 rounded-box z-50 max-h-80 w-64 overflow-y-auto p-2 shadow-sm">
                    <li class="menu-title">"Switch environment"</li>
                    for candidate in project.environments {
                        let component_is_available = component.is_none_or(|component| {
                            candidate
                                .components
                                .iter()
                                .any(|candidate_component| candidate_component.slug == component.slug)
                        });
                        let candidate_target = environment_context_destination(
                            tenant_slug,
                            project.slug,
                            candidate,
                            component,
                            component_suffix,
                        );

                        <li>
                            <a
                                href=(candidate_target)
                                class=(if candidate.slug == environment.slug { "menu-active" } else { "" })
                                aria-current=(if candidate.slug == environment.slug { "true" } else { "false" })
                            >
                                if candidate.slug == environment.slug {
                                    icon(data: iconify_icon!("feather:check"), attrs: attributes! {
                                        aria-hidden="true"
                                        class="size-4"
                                    })
                                }
                                <span class="truncate">(candidate.name)</span>
                                <span class="badge badge-sm">(candidate.region)</span>
                                if !component_is_available {
                                    <span class="sr-only">"Component unavailable; opens component list"</span>
                                }
                            </a>
                        </li>
                    }
                </ul>
            </details>
        }
    }
}

#[topcoat::view::component]
async fn component_title_switcher(
    tenant_slug: &'static str,
    project_slug: &'static str,
    environment: &'static mock::Environment,
    component: &'static mock::Component,
    component_suffix: &'static str,
) -> Result {
    view! {
        if environment.components.len() > 1 {
            <details class="dropdown" data-dropdown-menu="">
                <summary class="btn btn-ghost btn-sm btn-square" aria-label=(format!("Switch component; current component {}", component.name))>
                    icon(data: iconify_icon!("feather:chevron-down"), attrs: attributes! {
                        aria-hidden="true"
                        class="size-4"
                    })
                </summary>
                <ul class="dropdown-content menu bg-base-100 rounded-box z-50 max-h-80 w-72 overflow-y-auto p-2 shadow-sm">
                    <li class="menu-title">"Components"</li>
                    for candidate in environment.components {
                        <li>
                            <a
                                href=(component_context_destination(
                                    tenant_slug,
                                    project_slug,
                                    environment.slug,
                                    candidate.slug,
                                    component_suffix,
                                ))
                                class=(if candidate.slug == component.slug { "menu-active" } else { "" })
                                aria-current=(if candidate.slug == component.slug { "true" } else { "false" })
                            >
                                if candidate.slug == component.slug {
                                    icon(data: iconify_icon!("feather:check"), attrs: attributes! {
                                        aria-hidden="true"
                                        class="size-4"
                                    })
                                } else {
                                    <span aria-hidden="true" class=(if candidate.observability.is_some() {
                                        "status status-success"
                                    } else {
                                        "status status-warning"
                                    })></span>
                                }
                                <span class="truncate font-mono">(candidate.name)</span>
                                <span class="badge badge-sm">(candidate.kind)</span>
                            </a>
                        </li>
                    }
                </ul>
            </details>
        } else {
            ""
        }
    }
}

#[topcoat::view::component]
async fn deletion_dependencies(
    tenant_slug: String,
    project_slug: String,
    environment_slug: String,
    dependents: Vec<&'static mock::Component>,
) -> Result {
    view! {
        <div role="alert" class="alert alert-warning">
            <p class="font-medium">"Other components still depend on this component"</p>
            <p class="mt-1 text-xs">
                "Remove or reconfigure these dependencies before deleting it."
            </p>
            <ul class="mt-3 space-y-1.5">
                for dependent in dependents {
                    <li>
                        <a
                            href=(component_path(
                                &tenant_slug,
                                &project_slug,
                                &environment_slug,
                                dependent.slug,
                            ))
                            class="inline-flex items-center gap-1.5 text-xs font-medium"
                        >
                            (dependent.name)
                            icon(data: iconify_icon!("feather:arrow-up-right"), attrs: attributes! {
                                aria-hidden="true"
                                class="size-3"
                            })
                        </a>
                    </li>
                }
            </ul>
        </div>
    }
}

#[topcoat::view::component]
async fn feedback_banner(message: Option<String>, #[default] is_error: bool) -> Result {
    view! {
        match message {
            Some(message) => <div
                data-feedback=""
                role=(if is_error { "alert" } else { "status" })
                aria-live="polite"
                class=(if is_error {
                    "alert alert-error mb-6 items-start"
                } else {
                    "alert alert-success mb-6 items-start"
                })
            >
                <span class="mt-0.5">
                    icon(
                        data: if is_error {
                            iconify_icon!("feather:alert-circle")
                        } else {
                            iconify_icon!("feather:check")
                        },
                        attrs: attributes! { aria-hidden="true" class="size-3" }
                    )
                </span>
                <p class="min-w-0 flex-1 leading-5">(message)</p>
                <button
                    type="button"
                    data-dismiss-feedback=""
                    aria-label="Dismiss notification"
                    class="btn btn-circle btn-ghost btn-xs"
                >
                    icon(data: iconify_icon!("feather:x"), attrs: attributes! { class="size-3.5" })
                </button>
            </div>,
            None => "",
        }
    }
}

#[topcoat::view::component]
async fn change_list(changes: &'static [mock::Change]) -> Result {
    view! {
        <ol>
            for change in changes {
                <li>
                    <article class="flex h-11 min-w-0 items-center gap-3 px-4">
                        <h3 class="min-w-0 flex-1 truncate text-sm font-medium">(change.summary)</h3>
                        <code class="hidden w-16 shrink-0 text-xs sm:block">
                            (change.sha)
                        </code>
                        <span class="hidden w-28 shrink-0 truncate text-xs md:block">
                            (change.author)
                        </span>
                        <time class="w-24 shrink-0 text-right text-xs">
                            (change.time)
                        </time>
                    </article>
                </li>
            }
        </ol>
    }
}

#[cfg(test)]
mod tests {
    use super::{
        component_context_destination, environment_context_destination, managed_domain,
        normalize_hostname, parse_component_variables, project_context_destination,
    };
    use crate::mock;

    #[test]
    fn parses_only_unique_variable_keys() {
        let variables = parse_component_variables(
            "ENDPOINT\thttps://example.com?a=b\tplain\tPlain value\n\
             TOKEN\tsuper-secret-do-not-retain\tsecret\tVault reference\n\
             TOKEN\n\
             9INVALID\n\
             FEATURE-FLAG",
        );

        assert_eq!(variables.len(), 2);
        assert_eq!(variables[0].key, "ENDPOINT");
        assert_eq!(variables[1].key, "TOKEN");
    }

    #[test]
    fn creates_stable_managed_domains() {
        let domain = managed_domain("khuedoan", "finance", "production", "actualbudget");

        assert_eq!(
            domain,
            managed_domain("khuedoan", "finance", "production", "actualbudget")
        );
        assert!(domain.starts_with("actualbudget-production-"));
        assert!(domain.ends_with(".netamos.app"));
        assert_ne!(
            domain,
            managed_domain("khuedoan", "finance", "staging", "actualbudget")
        );
        assert_eq!(
            managed_domain("khuedoan", "test", "production", "My App"),
            managed_domain("khuedoan", "test", "production", "my-app")
        );
    }

    #[test]
    fn accepts_only_normalized_hostnames() {
        assert_eq!(normalize_hostname(" App.Example.COM. "), "app.example.com");
        assert_eq!(normalize_hostname("https://app.example.com/path"), "");
        assert_eq!(normalize_hostname("not a hostname"), "");
    }

    #[test]
    fn preserves_environment_depth_when_switching_projects() {
        let example = mock::project("test", "example").expect("example project");
        let example_service =
            mock::project("test", "example-service").expect("example service project");

        assert_eq!(
            project_context_destination("test", example, Some("staging")),
            "/tenants/test/projects/example/environments/staging"
        );
        assert_eq!(
            project_context_destination("test", example_service, Some("staging")),
            "/tenants/test/projects/example-service/environments/production"
        );
        assert_eq!(
            project_context_destination("test", example, None),
            "/tenants/test/projects/example"
        );
    }

    #[test]
    fn preserves_component_context_only_when_it_exists() {
        let staging = mock::environment("test", "example", "staging").expect("staging environment");
        let example =
            mock::component("test", "example", "production", "example").expect("example component");
        let server = mock::component("test", "example-service", "production", "server")
            .expect("server component");

        assert_eq!(
            environment_context_destination("test", "example", staging, Some(example), "/settings",),
            "/tenants/test/projects/example/environments/staging/components/example/settings"
        );
        assert_eq!(
            environment_context_destination("test", "example", staging, Some(server), "/settings",),
            "/tenants/test/projects/example/environments/staging?notice=component-unavailable&component=server"
        );
    }

    #[test]
    fn preserves_component_subpage_when_switching_components() {
        assert_eq!(
            component_context_destination(
                "khuedoan",
                "media",
                "production",
                "transmission",
                "/changes",
            ),
            "/tenants/khuedoan/projects/media/environments/production/components/transmission/changes"
        );
    }
}
