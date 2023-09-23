use serde::Deserialize;
use url::Url;

#[derive(Deserialize, Debug, Clone)]
pub struct Postgresql {
    #[serde(default = "default_url")]
    url: String,
}

fn default_url() -> String {
    "http://localhost".to_string()
}

// endregion7

impl Postgresql {
    pub fn from_env() -> Self {
        let data = envy::prefixed("DATABASE_")
            .from_env::<Postgresql>()
            .expect("Provide missing environment database variables");
        Self { ..data }
    }

    #[inline]
    pub fn url(&self) -> String {
        self.url.clone()
    }
}

impl Default for Postgresql {
    fn default() -> Self {
        serde_json::from_str::<Postgresql>("{}").expect("unable to initialize default values")
    }
}

impl From<Postgresql> for Url {
    fn from(value: Postgresql) -> Self {
        value.url.parse().expect("invalid url provided")
    }
}

//region test-cases for Application config
#[cfg(test)]
mod postgresql_env_tests {
    #[allow(unused_imports)]
    use super::super::*;
    use crate::config::test_envs::TestEnvs;
    use std::sync::Mutex;
    use url::Url;

    const DATABASE_URL: &str = "DATABASE_URL";

    const ENV_LIST: [&str; 1] = [DATABASE_URL];

    // due to globally managed environments, tests cannot run in parallel
    // therefore Mutex is used to maintain the sequence
    static LOCK: Mutex<bool> = Mutex::new(true);

    #[test]
    fn parse_defaults() {
        let c = {
            let _l = LOCK.lock().unwrap();
            let _e = TestEnvs::new(ENV_LIST.to_vec());

            Postgresql::default()
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

            std::env::set_var(DATABASE_URL, "http://my-host-name.domain");

            Postgresql::from_env()
        };

        assert_eq!(c.url, "http://my-host-name.domain".to_string());
    }
}
//endregion
