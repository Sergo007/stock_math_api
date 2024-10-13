use serde::Deserialize;
use url::Url;

#[derive(Deserialize, Debug, Clone)]
pub struct Application {
    #[serde(default = "default_bind")]
    bind: String,
    #[serde(default = "default_url")]
    url: String,
    #[serde(default = "default_port")]
    port: u16,
    #[serde(default = "default_workers")]
    workers: usize,
    #[serde(default = "default_allowed_origin")]
    allowed_origin: String,

    #[serde(default = "default_machine_id")]
    machine_id: i32,
    #[serde(default = "default_node_id")]
    node_id: i32,
}

// region default values for Application config
fn default_bind() -> String {
    "0.0.0.0".to_string()
}

fn default_url() -> String {
    "http://localhost".to_string()
}

fn default_port() -> u16 {
    8080
}

fn default_workers() -> usize {
    5
}

fn default_allowed_origin() -> String {
    "http://localhost:8000".to_string()
}

fn default_machine_id() -> i32 {
    1
}

fn default_node_id() -> i32 {
    1
}

// endregion7

impl Application {
    pub fn from_env() -> Self {
        let data = envy::prefixed("APP_")
            .from_env::<Application>()
            .expect("Provide missing environment application variables");
        Self { ..data }
    }

    #[inline]
    pub fn url(&self) -> String {
        self.url.clone()
    }

    #[inline]
    pub fn bind_address(&self) -> String {
        self.bind.clone()
    }

    #[inline]
    pub fn port(&self) -> u16 {
        self.port
    }

    #[inline]
    pub fn workers(&self) -> usize {
        self.workers
    }

    #[inline]
    pub fn allowed_origin(&self) -> String {
        self.allowed_origin.clone()
    }

    #[inline]
    pub fn machine_id(&self) -> i32 {
        self.machine_id
    }

    #[inline]
    pub fn node_id(&self) -> i32 {
        self.node_id
    }
}

impl Default for Application {
    fn default() -> Self {
        serde_json::from_str::<Application>("{}")
            .expect("unable to initialize default Application values")
    }
}

impl From<Application> for Url {
    fn from(value: Application) -> Self {
        value.url.parse().expect("invalid url provided")
    }
}
