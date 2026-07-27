use std::{
    collections::HashMap,
    sync::{OnceLock, RwLock},
};

#[derive(Clone, Copy, Debug)]
pub struct Tenant {
    pub slug: &'static str,
    pub name: &'static str,
    pub domains: &'static [&'static str],
    pub projects: &'static [Project],
    pub changes: &'static [Change],
}

#[derive(Clone, Copy, Debug)]
pub struct Project {
    pub slug: &'static str,
    pub name: &'static str,
    pub description: &'static str,
    pub environments: &'static [Environment],
    pub usage: Usage,
}

#[derive(Clone, Copy, Debug, PartialEq)]
pub struct Usage {
    pub compute_vcpu_hours: f32,
    pub memory_gib_hours: f32,
    pub egress_gb: f32,
}

#[derive(Clone, Copy, Debug)]
pub struct Environment {
    pub slug: &'static str,
    pub name: &'static str,
    pub region: &'static str,
    pub components: &'static [Component],
    pub volumes: &'static [Volume],
}

#[derive(Clone, Copy, Debug)]
pub struct Volume {
    pub slug: &'static str,
    pub name: &'static str,
    pub capacity_gib: u32,
    pub used_gib: f32,
    pub state: &'static str,
    pub backup_policy: &'static str,
    pub binding: Option<VolumeBinding>,
    pub backups: &'static [VolumeBackup],
}

#[derive(Clone, Copy, Debug)]
pub struct VolumeBinding {
    pub component_slug: &'static str,
    pub mount_path: &'static str,
}

#[derive(Clone, Copy, Debug)]
pub struct VolumeBackup {
    pub id: &'static str,
    pub created_at: &'static str,
    pub size: &'static str,
    pub state: &'static str,
}

#[derive(Clone, Copy, Debug)]
pub struct Component {
    pub slug: &'static str,
    pub name: &'static str,
    pub kind: &'static str,
    pub state: &'static str,
    pub summary: &'static str,
    pub url: Option<&'static str>,
    pub observability: Option<&'static Observability>,
    pub settings: &'static [Setting],
    pub variables: &'static [&'static str],
    pub changes: &'static [Change],
}

#[derive(Clone, Copy, Debug)]
pub struct Setting {
    pub label: &'static str,
    pub value: &'static str,
}

#[derive(Clone, Copy, Debug)]
pub struct Change {
    pub sha: &'static str,
    pub summary: &'static str,
    pub author: &'static str,
    pub time: &'static str,
    pub target: ChangeTarget,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct ChangeTarget {
    pub project_slug: &'static str,
    pub environment_slug: &'static str,
    pub component_slug: &'static str,
}

#[derive(Clone, Copy, Debug)]
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

#[derive(Clone, Copy, Debug)]
pub struct Metric {
    pub label: &'static str,
    pub value: &'static str,
}

#[derive(Clone, Copy, Debug)]
pub struct LogLine {
    pub time: &'static str,
    pub level: &'static str,
    pub message: &'static str,
}

#[derive(Clone, Debug)]
pub struct NewVolume {
    pub capacity_gib: u32,
    pub mount_path: String,
    pub backup_policy: String,
}

#[derive(Clone, Debug)]
pub struct NewComponent {
    pub name: String,
    pub kind: String,
    pub source: String,
    pub source_kind: String,
    pub visibility: String,
    pub domain: Option<String>,
    pub port: Option<u16>,
    pub variables: Vec<String>,
    pub settings: Vec<(String, String)>,
    pub volume: Option<NewVolume>,
}

#[derive(Clone, Debug)]
pub struct VolumeSettingsUpdate {
    pub capacity_gib: u32,
    pub mount_path: String,
    pub backup_policy: String,
}

#[derive(Clone, Debug, Default)]
pub struct ComponentSettingsUpdate {
    pub settings: Vec<(String, String)>,
    pub variables: Option<Vec<String>>,
    pub volume: Option<VolumeSettingsUpdate>,
}

static APPLICATION_LOGS: &[LogLine] = &[
    LogLine {
        time: "10:42:18",
        level: "INFO",
        message: "GET / 200 duration_ms=18",
    },
    LogLine {
        time: "10:42:11",
        level: "INFO",
        message: "health check passed",
    },
    LogLine {
        time: "10:41:58",
        level: "INFO",
        message: "configuration reconciled",
    },
    LogLine {
        time: "10:41:47",
        level: "WARN",
        message: "request latency above threshold duration_ms=842",
    },
    LogLine {
        time: "10:41:32",
        level: "ERROR",
        message: "upstream request failed status=502",
    },
    LogLine {
        time: "10:41:18",
        level: "INFO",
        message: "upstream retry succeeded",
    },
    LogLine {
        time: "10:40:54",
        level: "INFO",
        message: "POST /jobs 202 duration_ms=31",
    },
    LogLine {
        time: "10:40:31",
        level: "WARN",
        message: "memory usage above 80 percent",
    },
    LogLine {
        time: "10:40:12",
        level: "INFO",
        message: "background worker heartbeat",
    },
    LogLine {
        time: "10:39:48",
        level: "ERROR",
        message: "background job failed reason=timeout",
    },
    LogLine {
        time: "10:39:22",
        level: "INFO",
        message: "background job retry scheduled",
    },
    LogLine {
        time: "10:38:57",
        level: "INFO",
        message: "deployment became ready",
    },
];

static DATA_LOGS: &[LogLine] = &[
    LogLine {
        time: "10:42:00",
        level: "INFO",
        message: "background maintenance completed",
    },
    LogLine {
        time: "10:41:43",
        level: "WARN",
        message: "slow query detected duration_ms=614",
    },
    LogLine {
        time: "10:41:12",
        level: "ERROR",
        message: "replica connection failed",
    },
    LogLine {
        time: "10:40:39",
        level: "INFO",
        message: "replica connection restored",
    },
    LogLine {
        time: "10:40:12",
        level: "INFO",
        message: "connection pool healthy",
    },
    LogLine {
        time: "10:39:54",
        level: "WARN",
        message: "connection pool near capacity active=18 limit=20",
    },
    LogLine {
        time: "10:39:26",
        level: "INFO",
        message: "scheduled backup completed",
    },
    LogLine {
        time: "10:39:04",
        level: "INFO",
        message: "backup retry started",
    },
    LogLine {
        time: "10:38:58",
        level: "ERROR",
        message: "backup upload failed reason=timeout",
    },
    LogLine {
        time: "10:38:17",
        level: "WARN",
        message: "disk usage above 75 percent",
    },
    LogLine {
        time: "10:37:30",
        level: "INFO",
        message: "storage check passed",
    },
    LogLine {
        time: "10:36:55",
        level: "INFO",
        message: "checkpoint completed duration_ms=94",
    },
];

static APPLICATION_OBSERVABILITY: Observability = Observability {
    health: "Healthy",
    release: "Desired state active",
    uptime: "99.98%",
    primary_metric: Metric {
        label: "Requests",
        value: "42.8k",
    },
    secondary_metric: Metric {
        label: "p95 latency",
        value: "48ms",
    },
    cpu_percent: 18.0,
    memory_percent: 31.0,
    replicas: "1 / 1 ready",
    logs: APPLICATION_LOGS,
};

static REPLICATED_APPLICATION_OBSERVABILITY: Observability = Observability {
    health: "Healthy",
    release: "Desired state active",
    uptime: "99.99%",
    primary_metric: Metric {
        label: "Requests",
        value: "286k",
    },
    secondary_metric: Metric {
        label: "p95 latency",
        value: "42ms",
    },
    cpu_percent: 23.0,
    memory_percent: 38.0,
    replicas: "2 / 2 ready",
    logs: APPLICATION_LOGS,
};

static DATA_OBSERVABILITY: Observability = Observability {
    health: "Healthy",
    release: "Desired state active",
    uptime: "99.97%",
    primary_metric: Metric {
        label: "Operations",
        value: "38.2k",
    },
    secondary_metric: Metric {
        label: "Connections",
        value: "12",
    },
    cpu_percent: 28.0,
    memory_percent: 52.0,
    replicas: "1 / 1 ready",
    logs: DATA_LOGS,
};

static BLOG_CHANGES: &[Change] = &[
    Change {
        sha: "3186aa0",
        summary: "Publish cloud lab architecture notes",
        author: "Khue Doan",
        time: "12 minutes ago",
        target: ChangeTarget {
            project_slug: "blog",
            environment_slug: "production",
            component_slug: "blog",
        },
    },
    Change {
        sha: "81ac4e2",
        summary: "Scale blog to two replicas",
        author: "Khue Doan",
        time: "4 days ago",
        target: ChangeTarget {
            project_slug: "blog",
            environment_slug: "production",
            component_slug: "blog",
        },
    },
];

static ACTUALBUDGET_CHANGES: &[Change] = &[Change {
    sha: "2be903d",
    summary: "Configure persistent storage for Actual Budget",
    author: "Khue Doan",
    time: "2 hours ago",
    target: ChangeTarget {
        project_slug: "finance",
        environment_slug: "production",
        component_slug: "actualbudget",
    },
}];

static PAPERLESS_CHANGES: &[Change] = &[Change {
    sha: "40f3a18",
    summary: "Upgrade Paperless and Redis",
    author: "Khue Doan",
    time: "Yesterday",
    target: ChangeTarget {
        project_slug: "paperless",
        environment_slug: "production",
        component_slug: "paperless",
    },
}];

static OLLAMA_CHANGES: &[Change] = &[Change {
    sha: "7a12cc4",
    summary: "Provision local model storage",
    author: "Khue Doan",
    time: "2 days ago",
    target: ChangeTarget {
        project_slug: "ollama",
        environment_slug: "production",
        component_slug: "ollama",
    },
}];

static BLOG_SETTINGS: &[Setting] = &[
    Setting {
        label: "Source",
        value: "khuedoan/blog",
    },
    Setting {
        label: "Branch",
        value: "master",
    },
    Setting {
        label: "Image",
        value: "registry.registry.svc.cluster.local/apps/khuedoan/blog:3186aa0cc22649ce1dd897f80c1ac10e50d7e3af",
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
        value: "www.khuedoan.com",
    },
    Setting {
        label: "Additional domain",
        value: "www.production.khuedoan.com",
    },
];

static HN4E_SETTINGS: &[Setting] = &[
    Setting {
        label: "Source",
        value: "registry.registry.svc.cluster.local/apps/khuedoan/hn4e:1df645dc68d69bc130bfb39b5e2079b896457053",
    },
    Setting {
        label: "Branch",
        value: "image",
    },
    Setting {
        label: "Image",
        value: "registry.registry.svc.cluster.local/apps/khuedoan/hn4e:1df645dc68d69bc130bfb39b5e2079b896457053",
    },
    Setting {
        label: "Replicas",
        value: "1",
    },
    Setting {
        label: "Port",
        value: "3001",
    },
    Setting {
        label: "Domain",
        value: "hn4e.khuedoan.com",
    },
];

static DOCS_SETTINGS: &[Setting] = &[
    Setting {
        label: "Source",
        value: "registry.registry.svc.cluster.local/apps/khuedoan/homelab:606a2557fb7b20630cf4a3c817c33f4ee8048d49",
    },
    Setting {
        label: "Branch",
        value: "image",
    },
    Setting {
        label: "Image",
        value: "registry.registry.svc.cluster.local/apps/khuedoan/homelab:606a2557fb7b20630cf4a3c817c33f4ee8048d49",
    },
    Setting {
        label: "Replicas",
        value: "2",
    },
    Setting {
        label: "Port",
        value: "80",
    },
    Setting {
        label: "Domain",
        value: "homelab.khuedoan.com",
    },
    Setting {
        label: "Additional domain",
        value: "homelab.production.khuedoan.com",
    },
];

static ACTUALBUDGET_SETTINGS: &[Setting] = &[
    Setting {
        label: "Source",
        value: "docker.io/actualbudget/actual-server:latest-alpine",
    },
    Setting {
        label: "Branch",
        value: "image",
    },
    Setting {
        label: "Image",
        value: "docker.io/actualbudget/actual-server:latest-alpine",
    },
    Setting {
        label: "Replicas",
        value: "1",
    },
    Setting {
        label: "Port",
        value: "5006",
    },
    Setting {
        label: "Domain",
        value: "budget.khuedoan.com",
    },
    Setting {
        label: "Additional domain",
        value: "budget.production.khuedoan.com",
    },
    Setting {
        label: "Authentication",
        value: "OpenID Connect",
    },
];

macro_rules! image_settings {
    ($name:ident, $image:expr, $port:expr, $domain:expr; $(($label:expr, $value:expr)),* $(,)?) => {
        static $name: &[Setting] = &[
            Setting {
                label: "Source",
                value: $image,
            },
            Setting {
                label: "Branch",
                value: "image",
            },
            Setting {
                label: "Image",
                value: $image,
            },
            Setting {
                label: "Replicas",
                value: "1",
            },
            Setting {
                label: "Port",
                value: $port,
            },
            Setting {
                label: "Domain",
                value: $domain,
            },
            $(
                Setting {
                    label: $label,
                    value: $value,
                },
            )*
        ];
    };
}

image_settings!(
    EXCALIDRAW_SETTINGS,
    "docker.io/excalidraw/excalidraw:latest",
    "80",
    "draw.khuedoan.com";
);

image_settings!(
    HOMEPAGE_SETTINGS,
    "ghcr.io/gethomepage/homepage:v0.8.8",
    "3000",
    "home.khuedoan.com";
    ("Configuration", "ConfigMap mounted at /app/config")
);

image_settings!(
    JELLYFIN_SETTINGS,
    "docker.io/jellyfin/jellyfin:10.8.13",
    "8096",
    "jellyfin.khuedoan.com";
);

image_settings!(
    TRANSMISSION_SETTINGS,
    "lscr.io/linuxserver/transmission:4.0.5",
    "9091",
    "transmission.khuedoan.com";
);

image_settings!(
    PROWLARR_SETTINGS,
    "lscr.io/linuxserver/prowlarr:1.13.3",
    "9696",
    "prowlarr.khuedoan.com";
);

image_settings!(
    RADARR_SETTINGS,
    "lscr.io/linuxserver/radarr:5.3.6",
    "7878",
    "radarr.khuedoan.com";
);

image_settings!(
    SONARR_SETTINGS,
    "lscr.io/linuxserver/sonarr:4.0.2",
    "8989",
    "sonarr.khuedoan.com";
);

image_settings!(
    JELLYSEERR_SETTINGS,
    "docker.io/fallenbagel/jellyseerr:1.7.0",
    "5055",
    "jellyseerr.khuedoan.com";
);

static ELEMENTWEB_SETTINGS: &[Setting] = &[
    Setting {
        label: "Source",
        value: "https://locmai.github.io/charts",
    },
    Setting {
        label: "Branch",
        value: "chart",
    },
    Setting {
        label: "Chart",
        value: "elementweb",
    },
    Setting {
        label: "Version",
        value: "0.0.6",
    },
    Setting {
        label: "Replicas",
        value: "1",
    },
    Setting {
        label: "Domain",
        value: "chat.khuedoan.com",
    },
    Setting {
        label: "Homeserver",
        value: "https://matrix.khuedoan.com",
    },
];

static DENDRITE_SETTINGS: &[Setting] = &[
    Setting {
        label: "Source",
        value: "https://matrix-org.github.io/dendrite",
    },
    Setting {
        label: "Branch",
        value: "chart",
    },
    Setting {
        label: "Chart",
        value: "dendrite",
    },
    Setting {
        label: "Version",
        value: "0.13.5",
    },
    Setting {
        label: "Replicas",
        value: "1",
    },
    Setting {
        label: "Domain",
        value: "matrix.khuedoan.com",
    },
    Setting {
        label: "Server name",
        value: "matrix.khuedoan.com",
    },
];

static POSTGRESQL_SETTINGS: &[Setting] = &[
    Setting {
        label: "Source",
        value: "dendrite chart 0.13.5",
    },
    Setting {
        label: "Branch",
        value: "chart",
    },
    Setting {
        label: "Replicas",
        value: "1",
    },
    Setting {
        label: "Port",
        value: "5432",
    },
    Setting {
        label: "Domain",
        value: "",
    },
    Setting {
        label: "Visibility",
        value: "Internal only",
    },
];

image_settings!(
    OLLAMA_SETTINGS,
    "docker.io/ollama/ollama:0.1.29",
    "11434",
    "ollama.khuedoan.com";
);

image_settings!(
    OPEN_WEBUI_SETTINGS,
    "ghcr.io/open-webui/open-webui:latest",
    "8080",
    "ai.khuedoan.com";
    ("Provider", "http://ollama:11434")
);

image_settings!(
    PAPERLESS_SETTINGS,
    "ghcr.io/paperless-ngx/paperless-ngx:2.5.4",
    "8000",
    "paperless.khuedoan.com";
    ("Dependency", "redis:6379 · internal")
);

image_settings!(
    REDIS_SETTINGS,
    "docker.io/library/redis:7.2.4",
    "6379",
    "";
    ("Visibility", "Internal only")
);

image_settings!(
    SPEEDTEST_SETTINGS,
    "docker.io/openspeedtest/latest:latest",
    "3000",
    "speedtest.khuedoan.com";
);

image_settings!(
    WIREGUARD_SETTINGS,
    "lscr.io/linuxserver/wireguard:latest",
    "51820",
    "";
    ("Protocol", "UDP"),
    ("Network", "LoadBalancer · UDP 51820"),
    ("Capabilities", "NET_ADMIN"),
    ("Secrets", "WireGuard private key"),
    ("Configuration", "Mounted securely at /config/wg_confs")
);

static TEST_EXAMPLE_PRODUCTION_SETTINGS: &[Setting] = &[
    Setting {
        label: "Source",
        value: "khuedoan/example-service",
    },
    Setting {
        label: "Branch",
        value: "master",
    },
    Setting {
        label: "Image",
        value: "registry.registry.svc.cluster.local/apps/khuedoan/example-service:a9204b4b4c6875a81731745e43c4d76df987f6cd",
    },
    Setting {
        label: "Replicas",
        value: "2",
    },
    Setting {
        label: "Port",
        value: "8080",
    },
    Setting {
        label: "Domain",
        value: "example.production.khuedoan.com",
    },
];

static TEST_EXAMPLE_STAGING_SETTINGS: &[Setting] = &[
    Setting {
        label: "Source",
        value: "khuedoan/example-service",
    },
    Setting {
        label: "Branch",
        value: "master",
    },
    Setting {
        label: "Image",
        value: "registry.registry.svc.cluster.local/apps/khuedoan/example-service:a9204b4b4c6875a81731745e43c4d76df987f6cd",
    },
    Setting {
        label: "Replicas",
        value: "2",
    },
    Setting {
        label: "Port",
        value: "8080",
    },
    Setting {
        label: "Domain",
        value: "example.staging.khuedoan.com",
    },
];

static TEST_SERVER_SETTINGS: &[Setting] = &[
    Setting {
        label: "Source",
        value: "khuedoan/example-service",
    },
    Setting {
        label: "Branch",
        value: "master",
    },
    Setting {
        label: "Image",
        value: "registry.registry.svc.cluster.local/apps/khuedoan/example-service:a9204b4b4c6875a81731745e43c4d76df987f6cd",
    },
    Setting {
        label: "Replicas",
        value: "1",
    },
    Setting {
        label: "Port",
        value: "8009",
    },
    Setting {
        label: "Domain",
        value: "amazingshit.production.khuedoan.com",
    },
];

static BLOG_COMPONENTS: &[Component] = &[Component {
    slug: "blog",
    name: "blog",
    kind: "Application",
    state: "Configured",
    summary: "Personal website built from Git",
    url: Some("https://www.khuedoan.com"),
    observability: Some(&REPLICATED_APPLICATION_OBSERVABILITY),
    settings: BLOG_SETTINGS,
    variables: &[],
    changes: BLOG_CHANGES,
}];

static HN4E_COMPONENTS: &[Component] = &[Component {
    slug: "hn4e",
    name: "hn4e",
    kind: "Application",
    state: "Configured",
    summary: "HN reader and EPUB generator",
    url: Some("https://hn4e.khuedoan.com"),
    observability: Some(&APPLICATION_OBSERVABILITY),
    settings: HN4E_SETTINGS,
    variables: &[],
    changes: &[],
}];

static HOMELAB_COMPONENTS: &[Component] = &[Component {
    slug: "docs",
    name: "docs",
    kind: "Static site",
    state: "Configured",
    summary: "Homelab documentation",
    url: Some("https://homelab.khuedoan.com"),
    observability: Some(&REPLICATED_APPLICATION_OBSERVABILITY),
    settings: DOCS_SETTINGS,
    variables: &[],
    changes: &[],
}];

static EXCALIDRAW_COMPONENTS: &[Component] = &[Component {
    slug: "excalidraw",
    name: "excalidraw",
    kind: "Application",
    state: "Configured",
    summary: "Self-hosted virtual whiteboard",
    url: Some("https://draw.khuedoan.com"),
    observability: Some(&APPLICATION_OBSERVABILITY),
    settings: EXCALIDRAW_SETTINGS,
    variables: &[],
    changes: &[],
}];

static HOMEPAGE_COMPONENTS: &[Component] = &[Component {
    slug: "homepage",
    name: "homepage",
    kind: "Application",
    state: "Configured",
    summary: "Home lab service dashboard",
    url: Some("https://home.khuedoan.com"),
    observability: Some(&APPLICATION_OBSERVABILITY),
    settings: HOMEPAGE_SETTINGS,
    variables: &[],
    changes: &[],
}];

static MEDIA_COMPONENTS: &[Component] = &[
    Component {
        slug: "jellyfin",
        name: "jellyfin",
        kind: "Stateful app",
        state: "Configured",
        summary: "Movies, shows, and music server",
        url: Some("https://jellyfin.khuedoan.com"),
        observability: Some(&DATA_OBSERVABILITY),
        settings: JELLYFIN_SETTINGS,
        variables: &[],
        changes: &[],
    },
    Component {
        slug: "transmission",
        name: "transmission",
        kind: "Application",
        state: "Configured",
        summary: "BitTorrent client",
        url: Some("https://transmission.khuedoan.com"),
        observability: Some(&APPLICATION_OBSERVABILITY),
        settings: TRANSMISSION_SETTINGS,
        variables: &[],
        changes: &[],
    },
    Component {
        slug: "prowlarr",
        name: "prowlarr",
        kind: "Application",
        state: "Configured",
        summary: "Indexer manager",
        url: Some("https://prowlarr.khuedoan.com"),
        observability: Some(&APPLICATION_OBSERVABILITY),
        settings: PROWLARR_SETTINGS,
        variables: &[],
        changes: &[],
    },
    Component {
        slug: "radarr",
        name: "radarr",
        kind: "Application",
        state: "Configured",
        summary: "Movie collection manager",
        url: Some("https://radarr.khuedoan.com"),
        observability: Some(&APPLICATION_OBSERVABILITY),
        settings: RADARR_SETTINGS,
        variables: &[],
        changes: &[],
    },
    Component {
        slug: "sonarr",
        name: "sonarr",
        kind: "Application",
        state: "Configured",
        summary: "TV collection manager",
        url: Some("https://sonarr.khuedoan.com"),
        observability: Some(&APPLICATION_OBSERVABILITY),
        settings: SONARR_SETTINGS,
        variables: &[],
        changes: &[],
    },
    Component {
        slug: "jellyseerr",
        name: "jellyseerr",
        kind: "Application",
        state: "Configured",
        summary: "Media request manager",
        url: Some("https://jellyseerr.khuedoan.com"),
        observability: Some(&APPLICATION_OBSERVABILITY),
        settings: JELLYSEERR_SETTINGS,
        variables: &[],
        changes: &[],
    },
];

static MATRIX_COMPONENTS: &[Component] = &[
    Component {
        slug: "elementweb",
        name: "elementweb",
        kind: "Application",
        state: "Configured",
        summary: "Matrix web client",
        url: Some("https://chat.khuedoan.com"),
        observability: Some(&APPLICATION_OBSERVABILITY),
        settings: ELEMENTWEB_SETTINGS,
        variables: &[],
        changes: &[],
    },
    Component {
        slug: "dendrite",
        name: "dendrite",
        kind: "Stateful app",
        state: "Configured",
        summary: "Matrix homeserver",
        url: Some("https://matrix.khuedoan.com"),
        observability: Some(&DATA_OBSERVABILITY),
        settings: DENDRITE_SETTINGS,
        variables: &[],
        changes: &[],
    },
    Component {
        slug: "postgresql",
        name: "postgresql",
        kind: "PostgreSQL",
        state: "Configured",
        summary: "Internal database enabled by the Dendrite chart",
        url: None,
        observability: Some(&DATA_OBSERVABILITY),
        settings: POSTGRESQL_SETTINGS,
        variables: &[],
        changes: &[],
    },
];

static OLLAMA_COMPONENTS: &[Component] = &[
    Component {
        slug: "ollama",
        name: "ollama",
        kind: "AI runtime",
        state: "Configured",
        summary: "Local model API with persistent model storage",
        url: Some("https://ollama.khuedoan.com"),
        observability: Some(&DATA_OBSERVABILITY),
        settings: OLLAMA_SETTINGS,
        variables: &[],
        changes: OLLAMA_CHANGES,
    },
    Component {
        slug: "open-webui",
        name: "open-webui",
        kind: "Application",
        state: "Configured",
        summary: "Browser chat connected to Ollama",
        url: Some("https://ai.khuedoan.com"),
        observability: Some(&APPLICATION_OBSERVABILITY),
        settings: OPEN_WEBUI_SETTINGS,
        variables: &["OLLAMA_BASE_URL"],
        changes: &[],
    },
];

static PAPERLESS_COMPONENTS: &[Component] = &[
    Component {
        slug: "paperless",
        name: "paperless",
        kind: "Stateful app",
        state: "Configured",
        summary: "Document ingestion, storage, and OCR",
        url: Some("https://paperless.khuedoan.com"),
        observability: Some(&DATA_OBSERVABILITY),
        settings: PAPERLESS_SETTINGS,
        variables: &["PAPERLESS_PORT", "PAPERLESS_ADMIN_USER", "PAPERLESS_URL"],
        changes: PAPERLESS_CHANGES,
    },
    Component {
        slug: "redis",
        name: "redis",
        kind: "Cache",
        state: "Configured",
        summary: "Internal Paperless task broker",
        url: None,
        observability: Some(&DATA_OBSERVABILITY),
        settings: REDIS_SETTINGS,
        variables: &[],
        changes: &[],
    },
];

static SPEEDTEST_COMPONENTS: &[Component] = &[Component {
    slug: "speedtest",
    name: "speedtest",
    kind: "Application",
    state: "Configured",
    summary: "Internal network speed test",
    url: Some("https://speedtest.khuedoan.com"),
    observability: Some(&APPLICATION_OBSERVABILITY),
    settings: SPEEDTEST_SETTINGS,
    variables: &[],
    changes: &[],
}];

static WIREGUARD_COMPONENTS: &[Component] = &[Component {
    slug: "wireguard",
    name: "wireguard",
    kind: "Stateful app",
    state: "Configured",
    summary: "Home lab WireGuard gateway",
    url: None,
    observability: Some(&DATA_OBSERVABILITY),
    settings: WIREGUARD_SETTINGS,
    variables: &["LOG_CONFS", "USE_COREDNS"],
    changes: &[],
}];

static FINANCE_COMPONENTS: &[Component] = &[Component {
    slug: "actualbudget",
    name: "actualbudget",
    kind: "Stateful app",
    state: "Configured",
    summary: "Actual Budget server with OpenID Connect and persistent data",
    url: Some("https://budget.khuedoan.com"),
    observability: Some(&DATA_OBSERVABILITY),
    settings: ACTUALBUDGET_SETTINGS,
    variables: &[
        "ACTUAL_LOGIN_METHOD",
        "ACTUAL_OPENID_DISCOVERY_URL",
        "ACTUAL_OPENID_CLIENT_ID",
        "ACTUAL_OPENID_SERVER_HOSTNAME",
        "ACTUAL_USER_CREATION_MODE",
        "ACTUAL_OPENID_CLIENT_SECRET",
    ],
    changes: ACTUALBUDGET_CHANGES,
}];

static TEST_EXAMPLE_PRODUCTION_COMPONENTS: &[Component] = &[Component {
    slug: "example",
    name: "example",
    kind: "Application",
    state: "Configured",
    summary: "Example service production deployment",
    url: Some("https://example.production.khuedoan.com"),
    observability: Some(&REPLICATED_APPLICATION_OBSERVABILITY),
    settings: TEST_EXAMPLE_PRODUCTION_SETTINGS,
    variables: &["EXAMPLE_SECRET"],
    changes: &[],
}];

static TEST_EXAMPLE_STAGING_COMPONENTS: &[Component] = &[Component {
    slug: "example",
    name: "example",
    kind: "Application",
    state: "Configured",
    summary: "Example service staging deployment",
    url: Some("https://example.staging.khuedoan.com"),
    observability: Some(&REPLICATED_APPLICATION_OBSERVABILITY),
    settings: TEST_EXAMPLE_STAGING_SETTINGS,
    variables: &["EXAMPLE_SECRET"],
    changes: &[],
}];

static TEST_SERVER_COMPONENTS: &[Component] = &[Component {
    slug: "server",
    name: "server",
    kind: "Application",
    state: "Configured",
    summary: "Independent example server",
    url: Some("https://amazingshit.production.khuedoan.com"),
    observability: Some(&APPLICATION_OBSERVABILITY),
    settings: TEST_SERVER_SETTINGS,
    variables: &[],
    changes: &[],
}];

static MATRIX_VOLUMES: &[Volume] = &[Volume {
    slug: "postgresql-data",
    name: "PostgreSQL data",
    capacity_gib: 20,
    used_gib: 3.6,
    state: "Attached",
    backup_policy: "Daily · retain 7",
    binding: Some(VolumeBinding {
        component_slug: "postgresql",
        mount_path: "/var/lib/postgresql/data",
    }),
    backups: &[
        VolumeBackup {
            id: "matrix-postgresql-20260726-0200",
            created_at: "2026-07-26 02:00 UTC",
            size: "3.4 GiB",
            state: "Completed",
        },
        VolumeBackup {
            id: "matrix-postgresql-20260725-0200",
            created_at: "2026-07-25 02:00 UTC",
            size: "3.3 GiB",
            state: "Completed",
        },
    ],
}];

static OLLAMA_VOLUMES: &[Volume] = &[Volume {
    slug: "data",
    name: "Ollama models",
    capacity_gib: 10,
    used_gib: 7.4,
    state: "Attached",
    backup_policy: "Disabled",
    binding: Some(VolumeBinding {
        component_slug: "ollama",
        mount_path: "/root/.ollama",
    }),
    backups: &[],
}];

static PAPERLESS_VOLUMES: &[Volume] = &[Volume {
    slug: "data",
    name: "Paperless data and media",
    capacity_gib: 10,
    used_gib: 4.7,
    state: "Attached",
    backup_policy: "Daily · retain 14",
    binding: Some(VolumeBinding {
        component_slug: "paperless",
        mount_path: "/usr/src/paperless/data",
    }),
    backups: &[
        VolumeBackup {
            id: "paperless-data-20260726-0230",
            created_at: "2026-07-26 02:30 UTC",
            size: "4.4 GiB",
            state: "Completed",
        },
        VolumeBackup {
            id: "paperless-data-20260725-0230",
            created_at: "2026-07-25 02:30 UTC",
            size: "4.3 GiB",
            state: "Completed",
        },
    ],
}];

static FINANCE_VOLUMES: &[Volume] = &[Volume {
    slug: "data",
    name: "Actual Budget data",
    capacity_gib: 1,
    used_gib: 0.4,
    state: "Attached",
    backup_policy: "Daily · retain 7",
    binding: Some(VolumeBinding {
        component_slug: "actualbudget",
        mount_path: "/data",
    }),
    backups: &[
        VolumeBackup {
            id: "actualbudget-data-20260726-0200",
            created_at: "2026-07-26 02:00 UTC",
            size: "386 MiB",
            state: "Completed",
        },
        VolumeBackup {
            id: "actualbudget-data-20260725-0200",
            created_at: "2026-07-25 02:00 UTC",
            size: "381 MiB",
            state: "Completed",
        },
    ],
}];

static BLOG_ENVIRONMENTS: &[Environment] = &[Environment {
    slug: "production",
    name: "Production",
    region: "Helsinki",
    components: BLOG_COMPONENTS,
    volumes: &[],
}];

static HN4E_ENVIRONMENTS: &[Environment] = &[Environment {
    slug: "production",
    name: "Production",
    region: "Helsinki",
    components: HN4E_COMPONENTS,
    volumes: &[],
}];

static HOMELAB_ENVIRONMENTS: &[Environment] = &[Environment {
    slug: "production",
    name: "Production",
    region: "Helsinki",
    components: HOMELAB_COMPONENTS,
    volumes: &[],
}];

static EXCALIDRAW_ENVIRONMENTS: &[Environment] = &[Environment {
    slug: "production",
    name: "Production",
    region: "Saigon",
    components: EXCALIDRAW_COMPONENTS,
    volumes: &[],
}];

static HOMEPAGE_ENVIRONMENTS: &[Environment] = &[Environment {
    slug: "production",
    name: "Production",
    region: "Saigon",
    components: HOMEPAGE_COMPONENTS,
    volumes: &[],
}];

static MEDIA_ENVIRONMENTS: &[Environment] = &[Environment {
    slug: "production",
    name: "Production",
    region: "Saigon",
    components: MEDIA_COMPONENTS,
    volumes: &[],
}];

static MATRIX_ENVIRONMENTS: &[Environment] = &[Environment {
    slug: "production",
    name: "Production",
    region: "Saigon",
    components: MATRIX_COMPONENTS,
    volumes: MATRIX_VOLUMES,
}];

static OLLAMA_ENVIRONMENTS: &[Environment] = &[Environment {
    slug: "production",
    name: "Production",
    region: "Saigon",
    components: OLLAMA_COMPONENTS,
    volumes: OLLAMA_VOLUMES,
}];

static PAPERLESS_ENVIRONMENTS: &[Environment] = &[Environment {
    slug: "production",
    name: "Production",
    region: "Saigon",
    components: PAPERLESS_COMPONENTS,
    volumes: PAPERLESS_VOLUMES,
}];

static SPEEDTEST_ENVIRONMENTS: &[Environment] = &[Environment {
    slug: "production",
    name: "Production",
    region: "Saigon",
    components: SPEEDTEST_COMPONENTS,
    volumes: &[],
}];

static WIREGUARD_ENVIRONMENTS: &[Environment] = &[Environment {
    slug: "production",
    name: "Production",
    region: "Saigon",
    components: WIREGUARD_COMPONENTS,
    volumes: &[],
}];

static FINANCE_ENVIRONMENTS: &[Environment] = &[Environment {
    slug: "production",
    name: "Production",
    region: "Helsinki",
    components: FINANCE_COMPONENTS,
    volumes: FINANCE_VOLUMES,
}];

static TEST_EXAMPLE_ENVIRONMENTS: &[Environment] = &[
    Environment {
        slug: "production",
        name: "Production",
        region: "Helsinki",
        components: TEST_EXAMPLE_PRODUCTION_COMPONENTS,
        volumes: &[],
    },
    Environment {
        slug: "staging",
        name: "Staging",
        region: "Helsinki",
        components: TEST_EXAMPLE_STAGING_COMPONENTS,
        volumes: &[],
    },
];

static TEST_SERVER_ENVIRONMENTS: &[Environment] = &[Environment {
    slug: "production",
    name: "Production",
    region: "Helsinki",
    components: TEST_SERVER_COMPONENTS,
    volumes: &[],
}];

static KHUEDOAN_PROJECTS: &[Project] = &[
    Project {
        slug: "blog",
        name: "Blog",
        description: "Personal website built and deployed from Git",
        environments: BLOG_ENVIRONMENTS,
        usage: Usage {
            compute_vcpu_hours: 8.4,
            memory_gib_hours: 16.2,
            egress_gb: 2.8,
        },
    },
    Project {
        slug: "hn4e",
        name: "HN4E",
        description: "HN reader and EPUB generator",
        environments: HN4E_ENVIRONMENTS,
        usage: Usage {
            compute_vcpu_hours: 6.1,
            memory_gib_hours: 9.2,
            egress_gb: 4.9,
        },
    },
    Project {
        slug: "homelab",
        name: "Homelab docs",
        description: "Public documentation for the home lab",
        environments: HOMELAB_ENVIRONMENTS,
        usage: Usage {
            compute_vcpu_hours: 2.6,
            memory_gib_hours: 5.2,
            egress_gb: 0.6,
        },
    },
    Project {
        slug: "excalidraw",
        name: "Excalidraw",
        description: "Self-hosted virtual whiteboard",
        environments: EXCALIDRAW_ENVIRONMENTS,
        usage: Usage {
            compute_vcpu_hours: 1.8,
            memory_gib_hours: 3.6,
            egress_gb: 0.4,
        },
    },
    Project {
        slug: "homepage",
        name: "Homepage",
        description: "Home lab service dashboard",
        environments: HOMEPAGE_ENVIRONMENTS,
        usage: Usage {
            compute_vcpu_hours: 1.2,
            memory_gib_hours: 2.4,
            egress_gb: 0.2,
        },
    },
    Project {
        slug: "media",
        name: "Media",
        description: "Jellyfin and its coupled media automation services",
        environments: MEDIA_ENVIRONMENTS,
        usage: Usage {
            compute_vcpu_hours: 18.7,
            memory_gib_hours: 42.5,
            egress_gb: 38.4,
        },
    },
    Project {
        slug: "matrix",
        name: "Matrix",
        description: "Element Web, Dendrite, and PostgreSQL",
        environments: MATRIX_ENVIRONMENTS,
        usage: Usage {
            compute_vcpu_hours: 11.2,
            memory_gib_hours: 24.8,
            egress_gb: 6.7,
        },
    },
    Project {
        slug: "ollama",
        name: "Ollama",
        description: "Local model runtime and browser chat",
        environments: OLLAMA_ENVIRONMENTS,
        usage: Usage {
            compute_vcpu_hours: 22.5,
            memory_gib_hours: 64.0,
            egress_gb: 3.0,
        },
    },
    Project {
        slug: "paperless",
        name: "Paperless",
        description: "Document management and its Redis task broker",
        environments: PAPERLESS_ENVIRONMENTS,
        usage: Usage {
            compute_vcpu_hours: 9.6,
            memory_gib_hours: 18.8,
            egress_gb: 1.2,
        },
    },
    Project {
        slug: "speedtest",
        name: "Speedtest",
        description: "Internal network speed test",
        environments: SPEEDTEST_ENVIRONMENTS,
        usage: Usage {
            compute_vcpu_hours: 0.8,
            memory_gib_hours: 1.4,
            egress_gb: 12.6,
        },
    },
    Project {
        slug: "wireguard",
        name: "WireGuard",
        description: "Home lab WireGuard gateway",
        environments: WIREGUARD_ENVIRONMENTS,
        usage: Usage {
            compute_vcpu_hours: 3.7,
            memory_gib_hours: 4.8,
            egress_gb: 8.1,
        },
    },
    Project {
        slug: "finance",
        name: "Finance",
        description: "Actual Budget with OpenID Connect and persistent data",
        environments: FINANCE_ENVIRONMENTS,
        usage: Usage {
            compute_vcpu_hours: 3.8,
            memory_gib_hours: 7.4,
            egress_gb: 0.7,
        },
    },
];

static TEST_PROJECTS: &[Project] = &[
    Project {
        slug: "example",
        name: "Example",
        description: "Example service promoted between staging and production",
        environments: TEST_EXAMPLE_ENVIRONMENTS,
        usage: Usage {
            compute_vcpu_hours: 1.4,
            memory_gib_hours: 2.8,
            egress_gb: 0.3,
        },
    },
    Project {
        slug: "example-service",
        name: "Example service",
        description: "Independent server fixture",
        environments: TEST_SERVER_ENVIRONMENTS,
        usage: Usage {
            compute_vcpu_hours: 0.7,
            memory_gib_hours: 1.1,
            egress_gb: 0.1,
        },
    },
];

static KHUEDOAN_CHANGES: &[Change] = &[
    Change {
        sha: "3186aa0",
        summary: "Publish cloud lab architecture notes",
        author: "Khue Doan",
        time: "12 minutes ago",
        target: ChangeTarget {
            project_slug: "blog",
            environment_slug: "production",
            component_slug: "blog",
        },
    },
    Change {
        sha: "2be903d",
        summary: "Configure persistent storage for Actual Budget",
        author: "Khue Doan",
        time: "2 hours ago",
        target: ChangeTarget {
            project_slug: "finance",
            environment_slug: "production",
            component_slug: "actualbudget",
        },
    },
    Change {
        sha: "40f3a18",
        summary: "Upgrade Paperless and Redis",
        author: "Khue Doan",
        time: "Yesterday",
        target: ChangeTarget {
            project_slug: "paperless",
            environment_slug: "production",
            component_slug: "paperless",
        },
    },
    Change {
        sha: "7a12cc4",
        summary: "Provision local model storage",
        author: "Khue Doan",
        time: "2 days ago",
        target: ChangeTarget {
            project_slug: "ollama",
            environment_slug: "production",
            component_slug: "ollama",
        },
    },
];

static TENANTS: &[Tenant] = &[
    Tenant {
        slug: "khuedoan",
        name: "Khue Doan",
        domains: &["khuedoan.com"],
        projects: KHUEDOAN_PROJECTS,
        changes: KHUEDOAN_CHANGES,
    },
    Tenant {
        slug: "netamos",
        name: "Netamos",
        domains: &["netamos.io"],
        projects: &[],
        changes: &[],
    },
    Tenant {
        slug: "test",
        name: "Test",
        domains: &["khuedoan.com"],
        projects: TEST_PROJECTS,
        changes: &[],
    },
];

type MockState = HashMap<&'static str, &'static Tenant>;

static MOCK_STATE: OnceLock<RwLock<MockState>> = OnceLock::new();
static TENANT_ORDER: [&str; 3] = ["khuedoan", "netamos", "test"];

fn mock_state() -> &'static RwLock<MockState> {
    MOCK_STATE
        .get_or_init(|| RwLock::new(TENANTS.iter().map(|tenant| (tenant.slug, tenant)).collect()))
}

fn leak_value<T: 'static>(value: T) -> &'static T {
    Box::leak(Box::new(value))
}

fn leak_slice<T: 'static>(values: Vec<T>) -> &'static [T] {
    Box::leak(values.into_boxed_slice())
}

fn leak_string(value: String) -> &'static str {
    Box::leak(value.into_boxed_str())
}

fn install_tenant(state: &mut MockState, tenant: Tenant) -> &'static Tenant {
    let tenant = leak_value(tenant);
    state.insert(tenant.slug, tenant);
    tenant
}

fn tenant_not_found(slug: &str) -> String {
    format!("tenant '{slug}' does not exist")
}

fn project_not_found(tenant_slug: &str, project_slug: &str) -> String {
    format!("project '{project_slug}' does not exist in tenant '{tenant_slug}'")
}

fn environment_not_found(tenant_slug: &str, project_slug: &str, environment_slug: &str) -> String {
    format!(
        "environment '{environment_slug}' does not exist in project '{project_slug}' for tenant '{tenant_slug}'"
    )
}

fn component_not_found(
    tenant_slug: &str,
    project_slug: &str,
    environment_slug: &str,
    component_slug: &str,
) -> String {
    format!(
        "component '{component_slug}' does not exist in environment '{environment_slug}' for project '{project_slug}' in tenant '{tenant_slug}'"
    )
}

pub fn normalize_slug(value: &str) -> String {
    let mut slug = String::new();
    let mut previous_was_dash = false;

    for character in value.trim().chars() {
        let normalized = if character.is_ascii_alphanumeric() {
            character.to_ascii_lowercase()
        } else {
            '-'
        };

        if normalized == '-' {
            if slug.is_empty() || previous_was_dash {
                continue;
            }
            previous_was_dash = true;
        } else {
            previous_was_dash = false;
        }

        slug.push(normalized);
        if slug.len() == 63 {
            break;
        }
    }

    while slug.ends_with('-') {
        slug.pop();
    }

    slug
}

fn managed_component_domain(
    tenant_slug: &str,
    project_slug: &str,
    environment_slug: &str,
    component_slug: &str,
) -> String {
    fn dns_label(value: &str, max_len: usize) -> String {
        let mut label = normalize_slug(value);
        label.truncate(max_len);
        while label.ends_with('-') {
            label.pop();
        }
        if label.is_empty() {
            "app".to_owned()
        } else {
            label
        }
    }

    let tenant = dns_label(tenant_slug, 63);
    let project = dns_label(project_slug, 63);
    let environment = dns_label(environment_slug, 20);
    let component = dns_label(component_slug, 24);
    let identity = format!("{tenant}/{project}/{environment}/{component}");
    let hash = identity.bytes().fold(2_166_136_261_u32, |hash, byte| {
        (hash ^ u32::from(byte)).wrapping_mul(16_777_619)
    });

    format!(
        "{component}-{environment}-{:06x}.netamos.app",
        hash & 0x00ff_ffff
    )
}

pub fn normalize_domain(value: &str) -> Result<String, String> {
    let domain = value.trim().trim_end_matches('.').to_ascii_lowercase();
    let labels = domain.split('.').collect::<Vec<_>>();
    let valid = domain.len() <= 253
        && labels.len() >= 2
        && labels.iter().all(|label| {
            !label.is_empty()
                && label.len() <= 63
                && label
                    .bytes()
                    .all(|byte| byte.is_ascii_alphanumeric() || byte == b'-')
                && label
                    .as_bytes()
                    .first()
                    .is_some_and(u8::is_ascii_alphanumeric)
                && label
                    .as_bytes()
                    .last()
                    .is_some_and(u8::is_ascii_alphanumeric)
        });

    if valid {
        Ok(domain)
    } else {
        Err(format!(
            "'{value}' is not a valid domain; use a hostname such as example.com without a scheme or path"
        ))
    }
}

fn normalized_name(name: &str, resource: &str) -> Result<(String, String), String> {
    let display_name = name.trim();
    if display_name.is_empty() {
        return Err(format!("{resource} name cannot be empty"));
    }

    let slug = normalize_slug(display_name);
    if slug.is_empty() {
        return Err(format!(
            "{resource} name '{display_name}' must contain an ASCII letter or number"
        ));
    }

    Ok((display_name.to_owned(), slug))
}

fn normalized_region(region: &str) -> Result<String, String> {
    let region = region.trim();
    if region.is_empty() {
        return Err("environment region cannot be empty".to_owned());
    }

    Ok(match region.to_ascii_lowercase().as_str() {
        "helsinki" => "Helsinki".to_owned(),
        "saigon" => "Saigon".to_owned(),
        _ => region.to_owned(),
    })
}

fn normalized_visibility(visibility: &str) -> Result<&'static str, String> {
    match visibility.trim().to_ascii_lowercase().as_str() {
        "public" => Ok("Public"),
        "private" => Ok("Private"),
        "internal" | "internal only" => Ok("Internal only"),
        _ => Err(format!(
            "visibility '{visibility}' must be Public, Private, or Internal only"
        )),
    }
}

fn normalized_component_kind(kind: &str) -> Result<&'static str, String> {
    match kind.trim().to_ascii_lowercase().as_str() {
        "application" => Ok("Application"),
        "cron-job" | "cron job" => Ok("Cron job"),
        "postgresql" | "managed postgresql" => Ok("Managed PostgreSQL"),
        "valkey" | "managed valkey" => Ok("Managed Valkey"),
        _ => Err(format!(
            "component kind '{kind}' must be application, cron-job, postgresql, or valkey"
        )),
    }
}

fn normalized_variables(variables: Vec<String>) -> Result<Vec<String>, String> {
    let mut normalized = Vec::with_capacity(variables.len());

    for variable in variables {
        let key = variable
            .split_once('\t')
            .map_or(variable.as_str(), |(key, _)| key)
            .trim();
        let mut characters = key.chars();
        let valid_start = characters
            .next()
            .is_some_and(|character| character == '_' || character.is_ascii_alphabetic());
        let valid_rest =
            characters.all(|character| character == '_' || character.is_ascii_alphanumeric());

        if !valid_start || !valid_rest {
            return Err(format!(
                "environment variable '{key}' must use letters, numbers, and underscores and cannot start with a number"
            ));
        }
        if normalized.iter().any(|candidate| candidate == key) {
            return Err(format!("environment variable '{key}' is duplicated"));
        }
        normalized.push(key.to_owned());
    }

    Ok(normalized)
}

fn validated_mount_path(mount_path: &str) -> Result<String, String> {
    let mount_path = mount_path.trim();
    if !mount_path.starts_with('/') {
        return Err(format!(
            "volume mount path '{mount_path}' must be an absolute path"
        ));
    }
    Ok(mount_path.to_owned())
}

fn validated_capacity(capacity_gib: u32) -> Result<u32, String> {
    if capacity_gib == 0 {
        Err("volume capacity must be at least 1 GiB".to_owned())
    } else {
        Ok(capacity_gib)
    }
}

pub fn tenants() -> &'static [Tenant] {
    let state = mock_state()
        .read()
        .unwrap_or_else(|poisoned| poisoned.into_inner());
    leak_slice(
        TENANT_ORDER
            .iter()
            .filter_map(|slug| state.get(slug).map(|tenant| **tenant))
            .collect(),
    )
}

pub fn tenant(slug: &str) -> Option<&'static Tenant> {
    mock_state()
        .read()
        .unwrap_or_else(|poisoned| poisoned.into_inner())
        .get(slug)
        .copied()
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

pub fn volume(
    tenant_slug: &str,
    project_slug: &str,
    environment_slug: &str,
    volume_slug: &str,
) -> Option<&'static Volume> {
    environment(tenant_slug, project_slug, environment_slug)?
        .volumes
        .iter()
        .find(|volume| volume.slug == volume_slug)
}

pub fn storage_for_component(
    tenant_slug: &str,
    project_slug: &str,
    environment_slug: &str,
    component_slug: &str,
) -> Option<&'static Volume> {
    environment(tenant_slug, project_slug, environment_slug)?
        .volumes
        .iter()
        .find(|volume| {
            volume
                .binding
                .as_ref()
                .is_some_and(|binding| binding.component_slug == component_slug)
        })
}

fn dependency_slugs(project_slug: &str, component_slug: &str) -> &'static [&'static str] {
    match (project_slug, component_slug) {
        ("ollama", "open-webui") => &["ollama"],
        ("paperless", "paperless") => &["redis"],
        ("matrix", "dendrite") => &["postgresql"],
        _ => &[],
    }
}

pub fn dependencies(
    tenant_slug: &str,
    project_slug: &str,
    environment_slug: &str,
    component_slug: &str,
) -> Vec<&'static Component> {
    let Some(environment) = environment(tenant_slug, project_slug, environment_slug) else {
        return Vec::new();
    };
    if !environment
        .components
        .iter()
        .any(|component| component.slug == component_slug)
    {
        return Vec::new();
    }

    dependency_slugs(project_slug, component_slug)
        .iter()
        .filter_map(|dependency_slug| {
            environment
                .components
                .iter()
                .find(|component| component.slug == *dependency_slug)
        })
        .collect()
}

pub fn dependents(
    tenant_slug: &str,
    project_slug: &str,
    environment_slug: &str,
    component_slug: &str,
) -> Vec<&'static Component> {
    let Some(environment) = environment(tenant_slug, project_slug, environment_slug) else {
        return Vec::new();
    };
    if !environment
        .components
        .iter()
        .any(|component| component.slug == component_slug)
    {
        return Vec::new();
    }

    environment
        .components
        .iter()
        .filter(|component| {
            dependency_slugs(project_slug, component.slug).contains(&component_slug)
        })
        .collect()
}

fn replace_environment(
    tenant: &Tenant,
    project_slug: &str,
    environment_slug: &str,
    next_environment: Environment,
) -> Result<Tenant, String> {
    let mut projects = tenant.projects.to_vec();
    let project_index = projects
        .iter()
        .position(|project| project.slug == project_slug)
        .ok_or_else(|| project_not_found(tenant.slug, project_slug))?;
    let project = projects[project_index];
    let mut environments = project.environments.to_vec();
    let environment_index = environments
        .iter()
        .position(|environment| environment.slug == environment_slug)
        .ok_or_else(|| environment_not_found(tenant.slug, project_slug, environment_slug))?;

    environments[environment_index] = next_environment;
    projects[project_index] = Project {
        environments: leak_slice(environments),
        ..project
    };

    Ok(Tenant {
        projects: leak_slice(projects),
        ..*tenant
    })
}

fn installed_project(
    tenant: &'static Tenant,
    project_slug: &str,
) -> Result<&'static Project, String> {
    tenant
        .projects
        .iter()
        .find(|project| project.slug == project_slug)
        .ok_or_else(|| project_not_found(tenant.slug, project_slug))
}

fn installed_environment(
    tenant: &'static Tenant,
    project_slug: &str,
    environment_slug: &str,
) -> Result<&'static Environment, String> {
    installed_project(tenant, project_slug)?
        .environments
        .iter()
        .find(|environment| environment.slug == environment_slug)
        .ok_or_else(|| environment_not_found(tenant.slug, project_slug, environment_slug))
}

fn installed_component(
    tenant: &'static Tenant,
    project_slug: &str,
    environment_slug: &str,
    component_slug: &str,
) -> Result<&'static Component, String> {
    installed_environment(tenant, project_slug, environment_slug)?
        .components
        .iter()
        .find(|component| component.slug == component_slug)
        .ok_or_else(|| {
            component_not_found(tenant.slug, project_slug, environment_slug, component_slug)
        })
}

fn normalized_settings(settings: Vec<(String, String)>) -> Result<Vec<(String, String)>, String> {
    let mut normalized = Vec::with_capacity(settings.len());

    for (label, value) in settings {
        let label = label.trim();
        if label.is_empty() {
            return Err("component setting label cannot be empty".to_owned());
        }
        if normalized
            .iter()
            .any(|(candidate, _): &(String, String)| candidate == label)
        {
            return Err(format!("component setting '{label}' is duplicated"));
        }

        let value = match label {
            "Domain" if !value.trim().is_empty() => normalize_domain(&value)?,
            "Port" if !value.trim().is_empty() => {
                let port = value
                    .trim()
                    .parse::<u16>()
                    .map_err(|_| format!("component port '{}' is invalid", value.trim()))?;
                if port == 0 {
                    return Err("component port must be between 1 and 65535".to_owned());
                }
                port.to_string()
            }
            "Visibility" => normalized_visibility(&value)?.to_owned(),
            _ => value.trim().to_owned(),
        };
        normalized.push((label.to_owned(), value));
    }

    Ok(normalized)
}

fn merge_settings(existing: &[Setting], replacements: Vec<(String, String)>) -> &'static [Setting] {
    let mut settings = existing.to_vec();

    for (label, value) in replacements {
        if let Some(setting) = settings.iter_mut().find(|setting| setting.label == label) {
            setting.value = leak_string(value);
        } else {
            settings.push(Setting {
                label: leak_string(label),
                value: leak_string(value),
            });
        }
    }

    leak_slice(settings)
}

fn setting_value<'a>(settings: &'a [Setting], label: &str) -> Option<&'a str> {
    settings
        .iter()
        .find(|setting| setting.label == label)
        .map(|setting| setting.value)
}

fn component_url(settings: &[Setting], managed_domain: Option<String>) -> Option<&'static str> {
    let public = setting_value(settings, "Visibility") == Some("Public");
    let domain = setting_value(settings, "Domain").unwrap_or_default();

    public.then(|| {
        let domain = if domain.is_empty() {
            managed_domain.unwrap_or_default()
        } else {
            domain.to_owned()
        };
        leak_string(format!("https://{domain}"))
    })
}

fn source_branch(source_kind: &str, component_kind: &str) -> Result<&'static str, String> {
    if component_kind == "Managed PostgreSQL" || component_kind == "Managed Valkey" {
        return Ok("managed");
    }

    match source_kind.trim().to_ascii_lowercase().as_str() {
        "" | "repository" | "git" => Ok("master"),
        "image" => Ok("image"),
        "chart" => Ok("chart"),
        _ => Err(format!(
            "source kind '{source_kind}' must be repository, image, or chart"
        )),
    }
}

fn available_volume_slug(environment: &Environment, component_slug: &str) -> String {
    let candidates = [
        "data".to_owned(),
        normalize_slug(&format!("{component_slug}-data")),
    ];

    for candidate in candidates {
        if !environment
            .volumes
            .iter()
            .any(|volume| volume.slug == candidate)
        {
            return candidate;
        }
    }

    for suffix in 2.. {
        let candidate = normalize_slug(&format!("{component_slug}-data-{suffix}"));
        if !environment
            .volumes
            .iter()
            .any(|volume| volume.slug == candidate)
        {
            return candidate;
        }
    }

    unreachable!()
}

fn new_volume(
    environment: &Environment,
    component_slug: &'static str,
    component_name: &str,
    volume: NewVolume,
) -> Result<Volume, String> {
    let capacity_gib = validated_capacity(volume.capacity_gib)?;
    let mount_path = validated_mount_path(&volume.mount_path)?;
    let backup_policy = volume.backup_policy.trim();
    if backup_policy.is_empty() {
        return Err("volume backup policy cannot be empty".to_owned());
    }

    Ok(Volume {
        slug: leak_string(available_volume_slug(environment, component_slug)),
        name: leak_string(format!("{component_name} data")),
        capacity_gib,
        used_gib: 0.0,
        state: "Attached",
        backup_policy: leak_string(backup_policy.to_owned()),
        binding: Some(VolumeBinding {
            component_slug,
            mount_path: leak_string(mount_path),
        }),
        backups: &[],
    })
}

pub fn update_tenant_name(
    tenant_slug: &str,
    display_name: &str,
) -> Result<&'static Tenant, String> {
    let display_name = display_name.trim();
    if display_name.is_empty() {
        return Err("tenant display name cannot be empty".to_owned());
    }

    let mut state = mock_state()
        .write()
        .unwrap_or_else(|poisoned| poisoned.into_inner());
    let current = state
        .get(tenant_slug)
        .copied()
        .ok_or_else(|| tenant_not_found(tenant_slug))?;
    let next = Tenant {
        name: leak_string(display_name.to_owned()),
        ..*current
    };

    Ok(install_tenant(&mut state, next))
}

pub fn add_tenant_domain(tenant_slug: &str, domain: &str) -> Result<&'static Tenant, String> {
    let domain = normalize_domain(domain)?;
    let mut state = mock_state()
        .write()
        .unwrap_or_else(|poisoned| poisoned.into_inner());
    let current = state
        .get(tenant_slug)
        .copied()
        .ok_or_else(|| tenant_not_found(tenant_slug))?;
    if current.domains.contains(&domain.as_str()) {
        return Err(format!(
            "domain '{domain}' is already registered for tenant '{tenant_slug}'"
        ));
    }

    let mut domains = current.domains.to_vec();
    domains.push(leak_string(domain));
    let next = Tenant {
        domains: leak_slice(domains),
        ..*current
    };

    Ok(install_tenant(&mut state, next))
}

pub fn remove_tenant_domain(tenant_slug: &str, domain: &str) -> Result<&'static Tenant, String> {
    let domain = normalize_domain(domain)?;
    let mut state = mock_state()
        .write()
        .unwrap_or_else(|poisoned| poisoned.into_inner());
    let current = state
        .get(tenant_slug)
        .copied()
        .ok_or_else(|| tenant_not_found(tenant_slug))?;
    let Some(index) = current
        .domains
        .iter()
        .position(|candidate| *candidate == domain)
    else {
        return Err(format!(
            "domain '{domain}' is not registered for tenant '{tenant_slug}'"
        ));
    };

    let mut domains = current.domains.to_vec();
    domains.remove(index);
    let next = Tenant {
        domains: leak_slice(domains),
        ..*current
    };

    Ok(install_tenant(&mut state, next))
}

pub fn create_project(
    tenant_slug: &str,
    name: &str,
    description: &str,
) -> Result<&'static Project, String> {
    let (name, slug) = normalized_name(name, "project")?;
    let description = description.trim().to_owned();
    let mut state = mock_state()
        .write()
        .unwrap_or_else(|poisoned| poisoned.into_inner());
    let current = state
        .get(tenant_slug)
        .copied()
        .ok_or_else(|| tenant_not_found(tenant_slug))?;
    if current.projects.iter().any(|project| project.slug == slug) {
        return Err(format!(
            "project slug '{slug}' already exists in tenant '{tenant_slug}'"
        ));
    }

    let environments = leak_slice(vec![Environment {
        slug: "production",
        name: "Production",
        region: "Helsinki",
        components: &[],
        volumes: &[],
    }]);
    let mut projects = current.projects.to_vec();
    projects.push(Project {
        slug: leak_string(slug.clone()),
        name: leak_string(name),
        description: leak_string(description),
        environments,
        usage: Usage {
            compute_vcpu_hours: 0.0,
            memory_gib_hours: 0.0,
            egress_gb: 0.0,
        },
    });
    let next = Tenant {
        projects: leak_slice(projects),
        ..*current
    };
    let installed = install_tenant(&mut state, next);

    installed_project(installed, &slug)
}

pub fn create_environment(
    tenant_slug: &str,
    project_slug: &str,
    name: &str,
    region: &str,
) -> Result<&'static Environment, String> {
    let (name, slug) = normalized_name(name, "environment")?;
    let region = normalized_region(region)?;
    let mut state = mock_state()
        .write()
        .unwrap_or_else(|poisoned| poisoned.into_inner());
    let current = state
        .get(tenant_slug)
        .copied()
        .ok_or_else(|| tenant_not_found(tenant_slug))?;
    let project = current
        .projects
        .iter()
        .find(|project| project.slug == project_slug)
        .copied()
        .ok_or_else(|| project_not_found(tenant_slug, project_slug))?;
    if project
        .environments
        .iter()
        .any(|environment| environment.slug == slug)
    {
        return Err(format!(
            "environment slug '{slug}' already exists in project '{project_slug}'"
        ));
    }

    let mut environments = project.environments.to_vec();
    environments.push(Environment {
        slug: leak_string(slug.clone()),
        name: leak_string(name),
        region: leak_string(region),
        components: &[],
        volumes: &[],
    });
    let mut projects = current.projects.to_vec();
    let project_index = projects
        .iter()
        .position(|candidate| candidate.slug == project_slug)
        .expect("project was found above");
    projects[project_index] = Project {
        environments: leak_slice(environments),
        ..project
    };
    let next = Tenant {
        projects: leak_slice(projects),
        ..*current
    };
    let installed = install_tenant(&mut state, next);

    installed_environment(installed, project_slug, &slug)
}

pub fn create_component(
    tenant_slug: &str,
    project_slug: &str,
    environment_slug: &str,
    new_component: NewComponent,
) -> Result<&'static Component, String> {
    let (name, slug) = normalized_name(&new_component.name, "component")?;
    let kind = normalized_component_kind(&new_component.kind)?;
    let visibility = normalized_visibility(&new_component.visibility)?;
    let branch = source_branch(&new_component.source_kind, kind)?;
    let source = if new_component.source.trim().is_empty() {
        match kind {
            "Managed PostgreSQL" => "Managed PostgreSQL".to_owned(),
            "Managed Valkey" => "Managed Valkey".to_owned(),
            _ => return Err(format!("source cannot be empty for a {kind} component")),
        }
    } else {
        new_component.source.trim().to_owned()
    };
    if new_component.port == Some(0) {
        return Err("component port must be between 1 and 65535".to_owned());
    }
    let mut domain = new_component
        .domain
        .filter(|domain| !domain.trim().is_empty())
        .map(|domain| normalize_domain(&domain))
        .transpose()?;
    if visibility == "Public" && domain.is_none() {
        domain = Some(managed_component_domain(
            tenant_slug,
            project_slug,
            environment_slug,
            &slug,
        ));
    }
    let variables = normalized_variables(new_component.variables)?;
    let custom_settings = normalized_settings(new_component.settings)?;

    let mut state = mock_state()
        .write()
        .unwrap_or_else(|poisoned| poisoned.into_inner());
    let current = state
        .get(tenant_slug)
        .copied()
        .ok_or_else(|| tenant_not_found(tenant_slug))?;
    let project = current
        .projects
        .iter()
        .find(|project| project.slug == project_slug)
        .ok_or_else(|| project_not_found(tenant_slug, project_slug))?;
    let environment = project
        .environments
        .iter()
        .find(|environment| environment.slug == environment_slug)
        .ok_or_else(|| environment_not_found(tenant_slug, project_slug, environment_slug))?;
    if environment
        .components
        .iter()
        .any(|component| component.slug == slug)
    {
        return Err(format!(
            "component slug '{slug}' already exists in environment '{environment_slug}'"
        ));
    }

    let slug = leak_string(slug.clone());
    let mut common_settings = vec![
        ("Source".to_owned(), source),
        ("Branch".to_owned(), branch.to_owned()),
        ("Replicas".to_owned(), "1".to_owned()),
        (
            "Port".to_owned(),
            new_component
                .port
                .map_or_else(String::new, |port| port.to_string()),
        ),
        ("Domain".to_owned(), domain.unwrap_or_default()),
        ("Visibility".to_owned(), visibility.to_owned()),
    ];
    for (label, value) in custom_settings {
        if let Some((_, existing_value)) = common_settings
            .iter_mut()
            .find(|(existing_label, _)| existing_label == &label)
        {
            *existing_value = value;
        } else {
            common_settings.push((label, value));
        }
    }
    let settings = merge_settings(&[], common_settings);
    let variables = leak_slice(
        variables
            .into_iter()
            .map(leak_string)
            .collect::<Vec<&'static str>>(),
    );
    let component = Component {
        slug,
        name: leak_string(name.clone()),
        kind,
        state: "Provisioning",
        summary: leak_string(format!("New {kind} component")),
        url: component_url(
            settings,
            Some(managed_component_domain(
                tenant_slug,
                project_slug,
                environment_slug,
                slug,
            )),
        ),
        observability: None,
        settings,
        variables,
        changes: &[],
    };
    let mut components = environment.components.to_vec();
    components.push(component);
    let mut volumes = environment.volumes.to_vec();
    if let Some(volume) = new_component.volume {
        volumes.push(new_volume(environment, slug, &name, volume)?);
    }
    let next_environment = Environment {
        components: leak_slice(components),
        volumes: leak_slice(volumes),
        ..*environment
    };
    let next = replace_environment(current, project_slug, environment_slug, next_environment)?;
    let installed = install_tenant(&mut state, next);

    installed_component(installed, project_slug, environment_slug, slug)
}

pub fn update_component_settings(
    tenant_slug: &str,
    project_slug: &str,
    environment_slug: &str,
    component_slug: &str,
    update: ComponentSettingsUpdate,
) -> Result<&'static Component, String> {
    let replacement_settings = normalized_settings(update.settings)?;
    let replacement_variables = update.variables.map(normalized_variables).transpose()?;

    let mut state = mock_state()
        .write()
        .unwrap_or_else(|poisoned| poisoned.into_inner());
    let current = state
        .get(tenant_slug)
        .copied()
        .ok_or_else(|| tenant_not_found(tenant_slug))?;
    let project = current
        .projects
        .iter()
        .find(|project| project.slug == project_slug)
        .ok_or_else(|| project_not_found(tenant_slug, project_slug))?;
    let environment = project
        .environments
        .iter()
        .find(|environment| environment.slug == environment_slug)
        .ok_or_else(|| environment_not_found(tenant_slug, project_slug, environment_slug))?;
    let component_index = environment
        .components
        .iter()
        .position(|component| component.slug == component_slug)
        .ok_or_else(|| {
            component_not_found(tenant_slug, project_slug, environment_slug, component_slug)
        })?;
    let current_component = environment.components[component_index];
    let settings = merge_settings(current_component.settings, replacement_settings);
    let variables = replacement_variables.map_or(current_component.variables, |variables| {
        leak_slice(
            variables
                .into_iter()
                .map(leak_string)
                .collect::<Vec<&'static str>>(),
        )
    });
    let mut components = environment.components.to_vec();
    components[component_index] = Component {
        settings,
        variables,
        url: component_url(
            settings,
            Some(managed_component_domain(
                tenant_slug,
                project_slug,
                environment_slug,
                component_slug,
            )),
        ),
        ..current_component
    };

    let mut volumes = environment.volumes.to_vec();
    if let Some(volume_update) = update.volume {
        let capacity_gib = validated_capacity(volume_update.capacity_gib)?;
        let mount_path = validated_mount_path(&volume_update.mount_path)?;
        let backup_policy = volume_update.backup_policy.trim();
        if backup_policy.is_empty() {
            return Err("volume backup policy cannot be empty".to_owned());
        }

        if let Some(volume_index) = volumes.iter().position(|volume| {
            volume
                .binding
                .is_some_and(|binding| binding.component_slug == component_slug)
        }) {
            let current_volume = volumes[volume_index];
            if capacity_gib < current_volume.capacity_gib {
                return Err(format!(
                    "volume '{}' cannot shrink below its current {} GiB capacity",
                    current_volume.slug, current_volume.capacity_gib
                ));
            }
            volumes[volume_index] = Volume {
                capacity_gib,
                backup_policy: leak_string(backup_policy.to_owned()),
                binding: Some(VolumeBinding {
                    component_slug: current_component.slug,
                    mount_path: leak_string(mount_path),
                }),
                ..current_volume
            };
        } else {
            volumes.push(new_volume(
                environment,
                current_component.slug,
                current_component.name,
                NewVolume {
                    capacity_gib,
                    mount_path,
                    backup_policy: backup_policy.to_owned(),
                },
            )?);
        }
    }

    let next_environment = Environment {
        components: leak_slice(components),
        volumes: leak_slice(volumes),
        ..*environment
    };
    let next = replace_environment(current, project_slug, environment_slug, next_environment)?;
    let installed = install_tenant(&mut state, next);

    installed_component(installed, project_slug, environment_slug, component_slug)
}

pub fn remove_volume(
    tenant_slug: &str,
    project_slug: &str,
    environment_slug: &str,
    volume_slug: &str,
) -> Result<(), String> {
    let mut state = mock_state()
        .write()
        .unwrap_or_else(|poisoned| poisoned.into_inner());
    let current = state
        .get(tenant_slug)
        .copied()
        .ok_or_else(|| tenant_not_found(tenant_slug))?;
    let project = current
        .projects
        .iter()
        .find(|project| project.slug == project_slug)
        .ok_or_else(|| project_not_found(tenant_slug, project_slug))?;
    let environment = project
        .environments
        .iter()
        .find(|environment| environment.slug == environment_slug)
        .ok_or_else(|| environment_not_found(tenant_slug, project_slug, environment_slug))?;
    let Some(volume_index) = environment
        .volumes
        .iter()
        .position(|volume| volume.slug == volume_slug)
    else {
        return Err(format!(
            "volume '{volume_slug}' does not exist in environment '{environment_slug}'"
        ));
    };

    let mut volumes = environment.volumes.to_vec();
    volumes.remove(volume_index);
    let next_environment = Environment {
        volumes: leak_slice(volumes),
        ..*environment
    };
    let next = replace_environment(current, project_slug, environment_slug, next_environment)?;
    install_tenant(&mut state, next);

    Ok(())
}

pub fn delete_component(
    tenant_slug: &str,
    project_slug: &str,
    environment_slug: &str,
    component_slug: &str,
) -> Result<(), String> {
    let mut state = mock_state()
        .write()
        .unwrap_or_else(|poisoned| poisoned.into_inner());
    let current = state
        .get(tenant_slug)
        .copied()
        .ok_or_else(|| tenant_not_found(tenant_slug))?;
    let project = current
        .projects
        .iter()
        .find(|project| project.slug == project_slug)
        .ok_or_else(|| project_not_found(tenant_slug, project_slug))?;
    let environment = project
        .environments
        .iter()
        .find(|environment| environment.slug == environment_slug)
        .ok_or_else(|| environment_not_found(tenant_slug, project_slug, environment_slug))?;
    let Some(component_index) = environment
        .components
        .iter()
        .position(|component| component.slug == component_slug)
    else {
        return Err(component_not_found(
            tenant_slug,
            project_slug,
            environment_slug,
            component_slug,
        ));
    };

    if let Some(volume) = environment.volumes.iter().find(|volume| {
        volume
            .binding
            .is_some_and(|binding| binding.component_slug == component_slug)
    }) {
        return Err(format!(
            "cannot delete component '{component_slug}': remove attached volume '{}' first",
            volume.slug
        ));
    }

    let dependent_names = environment
        .components
        .iter()
        .filter(|component| {
            dependency_slugs(project_slug, component.slug).contains(&component_slug)
        })
        .map(|component| component.slug)
        .collect::<Vec<_>>();
    if !dependent_names.is_empty() {
        return Err(format!(
            "cannot delete component '{component_slug}': required by {}",
            dependent_names.join(", ")
        ));
    }

    let mut components = environment.components.to_vec();
    components.remove(component_index);
    let next_environment = Environment {
        components: leak_slice(components),
        ..*environment
    };
    let next = replace_environment(current, project_slug, environment_slug, next_environment)?;
    install_tenant(&mut state, next);

    Ok(())
}

fn change_sha(component: &Component, summary: &str) -> String {
    let hash = component
        .slug
        .bytes()
        .chain(summary.bytes())
        .chain(component.changes.len().to_string().bytes())
        .fold(2_166_136_261_u32, |hash, byte| {
            (hash ^ u32::from(byte)).wrapping_mul(16_777_619)
        });
    format!("{:07x}", hash & 0x0fff_ffff)
}

pub fn record_component_change(
    tenant_slug: &str,
    project_slug: &str,
    environment_slug: &str,
    component_slug: &str,
    summary: &str,
) -> Result<&'static Component, String> {
    let summary = summary.trim();
    if summary.is_empty() {
        return Err("change summary cannot be empty".to_owned());
    }

    let mut state = mock_state()
        .write()
        .unwrap_or_else(|poisoned| poisoned.into_inner());
    let current = state
        .get(tenant_slug)
        .copied()
        .ok_or_else(|| tenant_not_found(tenant_slug))?;
    let project = current
        .projects
        .iter()
        .find(|project| project.slug == project_slug)
        .ok_or_else(|| project_not_found(tenant_slug, project_slug))?;
    let environment = project
        .environments
        .iter()
        .find(|environment| environment.slug == environment_slug)
        .ok_or_else(|| environment_not_found(tenant_slug, project_slug, environment_slug))?;
    let component_index = environment
        .components
        .iter()
        .position(|component| component.slug == component_slug)
        .ok_or_else(|| {
            component_not_found(tenant_slug, project_slug, environment_slug, component_slug)
        })?;
    let current_component = environment.components[component_index];
    let change = Change {
        sha: leak_string(change_sha(&current_component, summary)),
        summary: leak_string(summary.to_owned()),
        author: "You",
        time: "Just now",
        target: ChangeTarget {
            project_slug: project.slug,
            environment_slug: environment.slug,
            component_slug: current_component.slug,
        },
    };

    let mut component_changes = current_component.changes.to_vec();
    component_changes.insert(0, change);
    let mut components = environment.components.to_vec();
    components[component_index] = Component {
        changes: leak_slice(component_changes),
        ..current_component
    };
    let next_environment = Environment {
        components: leak_slice(components),
        ..*environment
    };
    let mut next = replace_environment(current, project_slug, environment_slug, next_environment)?;
    let mut tenant_changes = next.changes.to_vec();
    tenant_changes.insert(0, change);
    next.changes = leak_slice(tenant_changes);
    let installed = install_tenant(&mut state, next);

    installed_component(installed, project_slug, environment_slug, component_slug)
}

#[cfg(test)]
mod tests {
    use super::*;

    fn setting<'a>(component: &'a Component, label: &str) -> Option<&'a str> {
        component
            .settings
            .iter()
            .find(|setting| setting.label == label)
            .map(|setting| setting.value)
    }

    #[test]
    fn exposes_the_ordered_existing_tenants() {
        let slugs = tenants()
            .iter()
            .map(|tenant| tenant.slug)
            .collect::<Vec<_>>();
        let khuedoan_projects = tenant("khuedoan")
            .unwrap()
            .projects
            .iter()
            .map(|project| project.slug)
            .collect::<Vec<_>>();
        let test_projects = tenant("test")
            .unwrap()
            .projects
            .iter()
            .map(|project| project.slug)
            .collect::<Vec<_>>();

        assert_eq!(slugs, ["khuedoan", "netamos", "test"]);
        assert_eq!(
            khuedoan_projects,
            [
                "blog",
                "hn4e",
                "homelab",
                "excalidraw",
                "homepage",
                "media",
                "matrix",
                "ollama",
                "paperless",
                "speedtest",
                "wireguard",
                "finance",
            ]
        );
        assert_eq!(test_projects, ["example", "example-service"]);
        assert_eq!(tenant("netamos").unwrap().projects.len(), 0);
    }

    #[test]
    fn isolates_resources_by_tenant() {
        assert!(project("khuedoan", "blog").is_some());
        assert!(project("test", "blog").is_none());
        assert!(project("khuedoan", "example").is_none());
        assert!(project("netamos", "example").is_none());
        assert!(component("test", "example", "production", "example").is_some());
        assert!(component("khuedoan", "example", "production", "example").is_none());
    }

    #[test]
    fn preserves_project_component_coupling() {
        let media = environment("khuedoan", "media", "production").unwrap();
        let matrix = environment("khuedoan", "matrix", "production").unwrap();
        let ollama = environment("khuedoan", "ollama", "production").unwrap();
        let paperless = environment("khuedoan", "paperless", "production").unwrap();

        assert_eq!(media.components.len(), 6);
        assert_eq!(matrix.components.len(), 3);
        assert_eq!(ollama.components.len(), 2);
        assert_eq!(paperless.components.len(), 2);
        assert!(component("khuedoan", "finance", "production", "importer").is_none());
    }

    #[test]
    fn keeps_current_and_legacy_regions_distinct() {
        for slug in ["blog", "hn4e", "homelab", "finance"] {
            let project = project("khuedoan", slug).unwrap();
            assert_eq!(project.environments.len(), 1);
            assert_eq!(project.environments[0].region, "Helsinki");
        }

        for slug in [
            "excalidraw",
            "homepage",
            "media",
            "matrix",
            "ollama",
            "paperless",
            "speedtest",
            "wireguard",
        ] {
            let project = project("khuedoan", slug).unwrap();
            assert_eq!(project.environments.len(), 1);
            assert_eq!(project.environments[0].region, "Saigon");
        }
    }

    #[test]
    fn represents_both_test_projects_faithfully() {
        let example = project("test", "example").unwrap();
        let example_service = project("test", "example-service").unwrap();

        assert_eq!(
            example
                .environments
                .iter()
                .map(|environment| environment.slug)
                .collect::<Vec<_>>(),
            ["production", "staging"]
        );
        assert_eq!(example.environments[0].components[0].slug, "example");
        assert_eq!(example_service.environments.len(), 1);
        assert_eq!(example_service.environments[0].components[0].slug, "server");
    }

    #[test]
    fn keeps_manifest_values_exact() {
        let blog = component("khuedoan", "blog", "production", "blog").unwrap();
        let sonarr = component("khuedoan", "media", "production", "sonarr").unwrap();
        let wireguard = component("khuedoan", "wireguard", "production", "wireguard").unwrap();
        let test_server = component("test", "example-service", "production", "server").unwrap();

        assert_eq!(setting(blog, "Source"), Some("khuedoan/blog"));
        assert_eq!(
            setting(blog, "Image"),
            Some(
                "registry.registry.svc.cluster.local/apps/khuedoan/blog:3186aa0cc22649ce1dd897f80c1ac10e50d7e3af"
            )
        );
        assert_eq!(
            setting(sonarr, "Image"),
            Some("lscr.io/linuxserver/sonarr:4.0.2")
        );
        assert_eq!(setting(sonarr, "Port"), Some("8989"));
        assert_eq!(setting(sonarr, "Domain"), Some("sonarr.khuedoan.com"));
        assert_eq!(setting(wireguard, "Protocol"), Some("UDP"));
        assert_eq!(
            setting(wireguard, "Network"),
            Some("LoadBalancer · UDP 51820")
        );
        assert_eq!(setting(wireguard, "Capabilities"), Some("NET_ADMIN"));
        assert_eq!(setting(test_server, "Port"), Some("8009"));
    }

    #[test]
    fn provides_usage_and_resolvable_activity_targets() {
        let blog = project("khuedoan", "blog").unwrap();
        assert_eq!(blog.usage.compute_vcpu_hours, 8.4);
        assert_eq!(blog.usage.memory_gib_hours, 16.2);
        assert_eq!(blog.usage.egress_gb, 2.8);

        for change in tenant("khuedoan").unwrap().changes {
            assert!(
                component(
                    "khuedoan",
                    change.target.project_slug,
                    change.target.environment_slug,
                    change.target.component_slug,
                )
                .is_some(),
                "unresolvable target for {}",
                change.sha
            );
        }
    }

    #[test]
    fn represents_persistent_storage_and_mounts() {
        let actualbudget = volume("khuedoan", "finance", "production", "data").unwrap();
        let postgresql = volume("khuedoan", "matrix", "production", "postgresql-data").unwrap();
        let ollama = volume("khuedoan", "ollama", "production", "data").unwrap();
        let paperless = volume("khuedoan", "paperless", "production", "data").unwrap();

        assert_eq!(actualbudget.capacity_gib, 1);
        assert_eq!(actualbudget.binding.as_ref().unwrap().mount_path, "/data");
        assert!(
            environment("khuedoan", "media", "production")
                .unwrap()
                .volumes
                .is_empty()
        );
        assert_eq!(
            postgresql.binding.as_ref().unwrap().component_slug,
            "postgresql"
        );
        assert_eq!(ollama.capacity_gib, 10);
        assert_eq!(ollama.binding.as_ref().unwrap().mount_path, "/root/.ollama");
        assert_eq!(paperless.capacity_gib, 10);
        assert_eq!(
            paperless.binding.as_ref().unwrap().mount_path,
            "/usr/src/paperless/data"
        );
        assert_eq!(
            storage_for_component("khuedoan", "finance", "production", "actualbudget")
                .unwrap()
                .slug,
            "data"
        );
        assert!(storage_for_component("khuedoan", "media", "production", "jellyfin").is_none());
    }

    #[test]
    fn every_volume_binding_resolves_to_a_component() {
        for tenant in tenants() {
            for project in tenant.projects {
                for environment in project.environments {
                    for volume in environment.volumes {
                        if let Some(binding) = &volume.binding {
                            assert!(
                                component(
                                    tenant.slug,
                                    project.slug,
                                    environment.slug,
                                    binding.component_slug,
                                )
                                .is_some(),
                                "unresolvable binding {} on volume {}",
                                binding.component_slug,
                                volume.slug
                            );
                        }
                    }
                }
            }
        }
    }

    #[test]
    fn no_component_is_bound_to_more_than_one_volume() {
        for tenant in tenants() {
            for project in tenant.projects {
                for environment in project.environments {
                    for component in environment.components {
                        let volume_count = environment
                            .volumes
                            .iter()
                            .filter(|volume| {
                                volume
                                    .binding
                                    .as_ref()
                                    .is_some_and(|binding| binding.component_slug == component.slug)
                            })
                            .count();

                        assert!(
                            volume_count <= 1,
                            "{} is bound to {volume_count} volumes",
                            component.slug
                        );
                    }
                }
            }
        }
    }

    #[test]
    fn isolates_volumes_by_tenant_project_and_environment() {
        assert_eq!(
            volume("khuedoan", "finance", "production", "data")
                .unwrap()
                .name,
            "Actual Budget data"
        );
        assert_eq!(
            volume("khuedoan", "ollama", "production", "data")
                .unwrap()
                .name,
            "Ollama models"
        );
        assert!(volume("test", "finance", "production", "data").is_none());
        assert!(volume("khuedoan", "blog", "production", "data").is_none());
        assert!(volume("khuedoan", "finance", "staging", "data").is_none());
    }

    #[test]
    fn normalizes_mutable_resource_identifiers() {
        assert_eq!(normalize_slug("  New API / Worker  "), "new-api-worker");
        assert_eq!(
            normalize_domain("Example.COM."),
            Ok("example.com".to_owned())
        );
        assert!(normalize_domain("https://example.com/path").is_err());
        assert_eq!(
            managed_component_domain("test", "example", "production", "web"),
            managed_component_domain("test", "example", "production", "web")
        );
    }

    #[test]
    fn exposes_registered_base_domains_and_dependency_edges() {
        assert_eq!(tenant("khuedoan").unwrap().domains, ["khuedoan.com"]);
        assert_eq!(tenant("netamos").unwrap().domains, ["netamos.io"]);
        assert_eq!(tenant("test").unwrap().domains, ["khuedoan.com"]);
        assert_eq!(
            dependencies("khuedoan", "ollama", "production", "open-webui")[0].slug,
            "ollama"
        );
        assert_eq!(
            dependents("khuedoan", "paperless", "production", "redis")[0].slug,
            "paperless"
        );
        assert_eq!(
            dependencies("khuedoan", "matrix", "production", "dendrite")[0].slug,
            "postgresql"
        );
    }

    #[test]
    fn keeps_environment_slugs_unique_within_each_project() {
        for tenant in tenants() {
            for project in tenant.projects {
                let mut slugs = project
                    .environments
                    .iter()
                    .map(|environment| environment.slug)
                    .collect::<Vec<_>>();
                let count = slugs.len();
                slugs.sort_unstable();
                slugs.dedup();
                assert_eq!(
                    slugs.len(),
                    count,
                    "duplicate environment slug in {}/{}",
                    tenant.slug,
                    project.slug,
                );
            }
        }
    }

    #[test]
    fn rejects_unknown_resources() {
        assert!(tenant("default").is_none());
        assert!(tenant("missing").is_none());
        assert!(project("khuedoan", "missing").is_none());
        assert!(environment("khuedoan", "blog", "missing").is_none());
        assert!(component("khuedoan", "blog", "production", "missing").is_none());
        assert!(volume("khuedoan", "finance", "production", "missing").is_none());
    }
}
