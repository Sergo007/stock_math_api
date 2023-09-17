#[derive(Clone)]
struct Pair {
    key: String,
    value: Option<String>,
}

pub struct TestEnvs {
    envs: Vec<Pair>,
}

impl TestEnvs {
    #[inline]
    fn get_env(name: &str) -> Option<String> {
        std::env::var(name).map_or_else(|_| None, |v| Some(v))
    }

    fn backup_and_remove_envs(envs: Vec<&str>) -> Vec<Pair> {
        let mut list = vec![];
        for key in envs {
            let value = Self::get_env(key);
            std::env::remove_var(key);

            let pair = Pair {
                key: key.to_string(),
                value,
            };
            list.push(pair);
        }
        list
    }

    fn set_env(pair: &Pair) {
        if let Some(value) = pair.value.clone() {
            std::env::set_var(pair.key.as_str(), value);
        } else {
            std::env::remove_var(pair.key.as_str());
        }
    }
    fn restore_envs(&self) {
        for pair in &self.envs {
            Self::set_env(&pair);
        }
    }

    pub fn new(envs: Vec<&str>) -> Self {
        Self {
            envs: Self::backup_and_remove_envs(envs),
        }
    }
}

impl Drop for TestEnvs {
    fn drop(&mut self) {
        self.restore_envs();
    }
}
