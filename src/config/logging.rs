use serde::Deserialize;

#[derive(Deserialize)]
pub struct Logging {
    #[serde(default = "default_format")]
    pub format: String,

    #[serde(default = "default_level")]
    pub level: String,
}

fn default_format() -> String {
    // "text".to_string()
    "json".to_string()
}

fn default_level() -> String {
    // "info".to_string()
    "debug".to_string()
}

impl Default for Logging {
    fn default() -> Self {
        serde_json::from_str::<Logging>("{}")
            .expect("unable to initialize default values for logging config")
    }
}

impl Logging {
    pub fn from_env() -> Self {
        envy::prefixed("LOG_")
            .from_env::<Logging>()
            .expect("Provide missing logging environment variables")
    }
}

//region test-cases for Application config
#[cfg(test)]
mod application_env_tests {
    #[allow(unused_imports)]
    use super::*;
    use crate::config::test_envs::TestEnvs;
    use std::sync::Mutex;

    const ENV_LOG_FORMAT: &str = "LOG_FORMAT";
    const ENV_LOG_LEVEL: &str = "LOG_LEVEL";
    const ENV_LIST: [&'static str; 2] = [ENV_LOG_FORMAT, ENV_LOG_LEVEL];

    // due to globally managed environments, tests cannot run in parallel
    // therefore Mutex is used to maintain the sequence
    static LOCK: Mutex<bool> = Mutex::new(true);

    #[test]
    fn validate_env_names() {
        assert_eq!("LOG_FORMAT", ENV_LOG_FORMAT);
        assert_eq!("LOG_LEVEL", ENV_LOG_LEVEL);
    }

    #[test]
    fn parse_default_log_format_env() {
        let _l = LOCK.lock().unwrap();
        let _e = TestEnvs::new(ENV_LIST.to_vec());

        let c = Logging::default();
        assert_eq!(c.format, "text");
    }

    #[test]
    fn parse_log_format_env() {
        let _l = LOCK.lock().unwrap();
        let _e = TestEnvs::new(ENV_LIST.to_vec());

        std::env::set_var(ENV_LOG_FORMAT, &"my_new_format");
        std::env::set_var(ENV_LOG_LEVEL, &"my_new_log_level");

        let c = Logging::from_env();

        assert_eq!(c.format, "my_new_format");
        assert_eq!(c.level, "my_new_log_level");
    }
}
//endregion
