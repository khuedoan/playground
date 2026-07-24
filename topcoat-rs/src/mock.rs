pub struct Tenant {
    pub slug: &'static str,
    pub name: &'static str,
    pub domains: &'static [&'static str],
    pub projects: &'static [Project],
    pub changes: &'static [Change],
}

pub struct Project {
    pub slug: &'static str,
    pub name: &'static str,
    pub description: &'static str,
    pub environments: &'static [Environment],
}

pub struct Environment {
    pub slug: &'static str,
    pub name: &'static str,
    pub region: &'static str,
    pub components: &'static [Component],
}

pub struct Component {
    pub slug: &'static str,
    pub name: &'static str,
    pub kind: &'static str,
    pub state: &'static str,
    pub summary: &'static str,
    pub url: Option<&'static str>,
    pub observability: Option<&'static Observability>,
    pub settings: &'static [Setting],
    pub changes: &'static [Change],
}

pub struct Setting {
    pub label: &'static str,
    pub value: &'static str,
}

pub struct Change {
    pub sha: &'static str,
    pub summary: &'static str,
    pub author: &'static str,
    pub time: &'static str,
}

pub struct Observability {
    pub health: &'static str,
    pub release: &'static str,
    pub uptime: &'static str,
    pub primary_metric: Metric,
    pub secondary_metric: Metric,
    pub cpu_percent: f32,
    pub memory_percent: f32,
    pub replicas: &'static str,
    pub logs: &'static [LogLine],
}

pub struct Metric {
    pub label: &'static str,
    pub value: &'static str,
}

pub struct LogLine {
    pub time: &'static str,
    pub level: &'static str,
    pub message: &'static str,
}

static WEB_LOGS: &[LogLine] = &[
    LogLine {
        time: "23:42:18",
        level: "INFO",
        message: "checkout completed order=ord_9f2 duration_ms=118",
    },
    LogLine {
        time: "23:42:16",
        level: "INFO",
        message: "GET /products/linen-shirt 200 duration_ms=24",
    },
    LogLine {
        time: "23:42:14",
        level: "WARN",
        message: "inventory lookup retried sku=linen-shirt attempt=2",
    },
    LogLine {
        time: "23:42:10",
        level: "INFO",
        message: "GET /cart 200 duration_ms=31",
    },
];

static WORKER_LOGS: &[LogLine] = &[
    LogLine {
        time: "23:42:20",
        level: "INFO",
        message: "order confirmation sent order=ord_9f2",
    },
    LogLine {
        time: "23:42:12",
        level: "INFO",
        message: "queue batch processed jobs=24 duration_ms=384",
    },
];

static POSTGRES_LOGS: &[LogLine] = &[LogLine {
    time: "23:42:00",
    level: "INFO",
    message: "checkpoint complete buffers=142 duration_ms=91",
}];

static WEB_OBSERVABILITY: Observability = Observability {
    health: "Healthy",
    release: "8f72c1a · 12 minutes ago",
    uptime: "99.99%",
    primary_metric: Metric {
        label: "Requests",
        value: "1.8M",
    },
    secondary_metric: Metric {
        label: "p95 latency",
        value: "142ms",
    },
    cpu_percent: 62.0,
    memory_percent: 48.0,
    replicas: "2 / 2 ready",
    logs: WEB_LOGS,
};

static WORKER_OBSERVABILITY: Observability = Observability {
    health: "Healthy",
    release: "8f72c1a · 12 minutes ago",
    uptime: "99.97%",
    primary_metric: Metric {
        label: "Jobs processed",
        value: "84.2k",
    },
    secondary_metric: Metric {
        label: "p95 runtime",
        value: "380ms",
    },
    cpu_percent: 38.0,
    memory_percent: 44.0,
    replicas: "1 / 1 ready",
    logs: WORKER_LOGS,
};

static POSTGRES_OBSERVABILITY: Observability = Observability {
    health: "Healthy",
    release: "PostgreSQL 17",
    uptime: "99.99%",
    primary_metric: Metric {
        label: "Queries",
        value: "9.4M",
    },
    secondary_metric: Metric {
        label: "p95 latency",
        value: "18ms",
    },
    cpu_percent: 41.0,
    memory_percent: 67.0,
    replicas: "1 / 1 ready",
    logs: POSTGRES_LOGS,
};

static WEB_CHANGES: &[Change] = &[
    Change {
        sha: "8f72c1a",
        summary: "Deploy storefront checkout fix",
        author: "Kara Smith",
        time: "12 minutes ago",
    },
    Change {
        sha: "bd408ce",
        summary: "Scale web to two replicas",
        author: "Jon Bell",
        time: "Yesterday",
    },
];

static WEB_SETTINGS: &[Setting] = &[
    Setting {
        label: "Source",
        value: "forgejo.example/acme/storefront",
    },
    Setting {
        label: "Branch",
        value: "main",
    },
    Setting {
        label: "Image",
        value: "registry.example/apps/acme/storefront@sha256:8f72c1a",
    },
    Setting {
        label: "Replicas",
        value: "2",
    },
    Setting {
        label: "Port",
        value: "3000",
    },
    Setting {
        label: "Domain",
        value: "storefront.example.com",
    },
];

static WORKER_SETTINGS: &[Setting] = &[
    Setting {
        label: "Source",
        value: "forgejo.example/acme/storefront",
    },
    Setting {
        label: "Branch",
        value: "main",
    },
    Setting {
        label: "Replicas",
        value: "1",
    },
];

static POSTGRES_SETTINGS: &[Setting] = &[
    Setting {
        label: "Engine",
        value: "PostgreSQL",
    },
    Setting {
        label: "Storage",
        value: "20Gi",
    },
];

static CRON_SETTINGS: &[Setting] = &[
    Setting {
        label: "Source",
        value: "ghcr.io/acme/reporting:latest",
    },
    Setting {
        label: "Schedule",
        value: "0 3 * * *",
    },
];

static PRODUCTION_COMPONENTS: &[Component] = &[
    Component {
        slug: "web",
        name: "web",
        kind: "Application",
        state: "Configured",
        summary: "Public storefront application",
        url: Some("https://storefront.example.com"),
        observability: Some(&WEB_OBSERVABILITY),
        settings: WEB_SETTINGS,
        changes: WEB_CHANGES,
    },
    Component {
        slug: "worker",
        name: "worker",
        kind: "Application",
        state: "Configured",
        summary: "Background order processing",
        url: None,
        observability: Some(&WORKER_OBSERVABILITY),
        settings: WORKER_SETTINGS,
        changes: &[],
    },
    Component {
        slug: "postgres",
        name: "postgres",
        kind: "PostgreSQL",
        state: "Configured",
        summary: "Storefront primary database",
        url: None,
        observability: Some(&POSTGRES_OBSERVABILITY),
        settings: POSTGRES_SETTINGS,
        changes: &[],
    },
];

static STAGING_COMPONENTS: &[Component] = &[Component {
    slug: "web",
    name: "web",
    kind: "Application",
    state: "Pending image",
    summary: "Storefront staging application",
    url: None,
    observability: None,
    settings: WEB_SETTINGS,
    changes: WEB_CHANGES,
}];

static REPORTING_COMPONENTS: &[Component] = &[Component {
    slug: "nightly-report",
    name: "nightly-report",
    kind: "Cron job",
    state: "Configured",
    summary: "Generate the daily operations report",
    url: None,
    observability: None,
    settings: CRON_SETTINGS,
    changes: &[],
}];

static STOREFRONT_ENVIRONMENTS: &[Environment] = &[
    Environment {
        slug: "production",
        name: "Production",
        region: "Helsinki",
        components: PRODUCTION_COMPONENTS,
    },
    Environment {
        slug: "staging",
        name: "Staging",
        region: "Helsinki",
        components: STAGING_COMPONENTS,
    },
];

static INTERNAL_ENVIRONMENTS: &[Environment] = &[Environment {
    slug: "production",
    name: "Production",
    region: "Helsinki",
    components: REPORTING_COMPONENTS,
}];

static PROJECTS: &[Project] = &[
    Project {
        slug: "storefront",
        name: "Storefront",
        description: "Customer-facing commerce services",
        environments: STOREFRONT_ENVIRONMENTS,
    },
    Project {
        slug: "internal-tools",
        name: "Internal tools",
        description: "Operational applications and scheduled jobs",
        environments: INTERNAL_ENVIRONMENTS,
    },
];

static RECENT_CHANGES: &[Change] = &[
    Change {
        sha: "8f72c1a",
        summary: "Deploy storefront checkout fix",
        author: "Kara Smith",
        time: "12 minutes ago",
    },
    Change {
        sha: "14a90df",
        summary: "Create nightly reporting job",
        author: "Jon Bell",
        time: "3 hours ago",
    },
    Change {
        sha: "bd408ce",
        summary: "Scale storefront web to two replicas",
        author: "Kara Smith",
        time: "Yesterday",
    },
];

static DEFAULT_TENANT: Tenant = Tenant {
    slug: "default",
    name: "Netamos",
    domains: &["example.com", "internal.example.com"],
    projects: PROJECTS,
    changes: RECENT_CHANGES,
};

pub fn tenant(slug: &str) -> Option<&'static Tenant> {
    (slug == DEFAULT_TENANT.slug).then_some(&DEFAULT_TENANT)
}

pub fn project(tenant_slug: &str, project_slug: &str) -> Option<&'static Project> {
    tenant(tenant_slug)?
        .projects
        .iter()
        .find(|project| project.slug == project_slug)
}

pub fn environment(
    tenant_slug: &str,
    project_slug: &str,
    environment_slug: &str,
) -> Option<&'static Environment> {
    project(tenant_slug, project_slug)?
        .environments
        .iter()
        .find(|environment| environment.slug == environment_slug)
}

pub fn component(
    tenant_slug: &str,
    project_slug: &str,
    environment_slug: &str,
    component_slug: &str,
) -> Option<&'static Component> {
    environment(tenant_slug, project_slug, environment_slug)?
        .components
        .iter()
        .find(|component| component.slug == component_slug)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn finds_nested_mock_resources() {
        let component = component("default", "storefront", "production", "web").unwrap();

        assert_eq!(component.kind, "Application");
        assert_eq!(component.changes.len(), 2);
        assert_eq!(component.observability.unwrap().health, "Healthy");
    }

    #[test]
    fn rejects_unknown_resources() {
        assert!(tenant("missing").is_none());
        assert!(project("default", "missing").is_none());
        assert!(environment("default", "storefront", "missing").is_none());
        assert!(component("default", "storefront", "production", "missing").is_none());
    }
}
