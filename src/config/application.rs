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
        let data = envy::prefixed("APPLICATION_")
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
        serde_json::from_str::<Application>("{}").expect("unable to initialize default values")
    }
}

impl From<Application> for Url {
    fn from(value: Application) -> Self {
        value.url.parse().expect("invalid url provided")
    }
}

//region test-cases for Application config
#[cfg(test)]
mod application_env_tests {
    #[allow(unused_imports)]
    use super::super::*;
    use crate::config::test_envs::TestEnvs;
    use std::sync::Mutex;
    use url::Url;

    const ENV_APPLICATION_URL: &str = "APPLICATION_URL";
    const ENV_APPLICATION_PORT: &str = "APPLICATION_PORT";
    const ENV_APPLICATION_BIND: &str = "APPLICATION_BIND";
    const ENV_APPLICATION_ALLOWED_ORIGIN: &str = "APPLICATION_ALLOWED_ORIGIN";

    const ENV_LIST: [&str; 4] = [
        ENV_APPLICATION_URL,
        ENV_APPLICATION_PORT,
        ENV_APPLICATION_BIND,
        ENV_APPLICATION_ALLOWED_ORIGIN,
    ];

    // due to globally managed environments, tests cannot run in parallel
    // therefore Mutex is used to maintain the sequence
    static LOCK: Mutex<bool> = Mutex::new(true);

    #[test]
    fn parse_defaults() {
        let c = {
            let _l = LOCK.lock().unwrap();
            let _e = TestEnvs::new(ENV_LIST.to_vec());

            Application::default()
        };
        let url = Url::from(c);
        let u = url.as_str().trim_end_matches('/');
        assert_eq!(u, "http://localhost");
    }

    #[test]
    fn parse_application_envs() {
        let c = {
            let _l = LOCK.lock().unwrap();
            let _e = TestEnvs::new(ENV_LIST.to_vec());

            std::env::set_var(ENV_APPLICATION_URL, "http://my-host-name.domain");
            std::env::set_var(ENV_APPLICATION_PORT, "12345");
            std::env::set_var(ENV_APPLICATION_BIND, "1.2.3.4");
            std::env::set_var(ENV_APPLICATION_ALLOWED_ORIGIN, "http://my_origin:8000");

            Application::from_env()
        };

        assert_eq!(c.url, "http://my-host-name.domain".to_string());
        assert_eq!(c.port, 12345);
        assert_eq!(c.bind, "1.2.3.4".to_string());
        assert_eq!(c.allowed_origin, "http://my_origin:8000".to_string());
    }
}
//endregion
