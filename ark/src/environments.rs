#[derive(PartialEq)]
pub enum Environment {
    Local,
    Staging,
    Production,
}

impl From<String> for Environment {
    fn from(value: String) -> Self {
        match value.as_str() {
            "local" => Environment::Local,
            "staging" => Environment::Staging,
            "production" => Environment::Production,
            _ => panic!("Invalid Environment"),
        }
    }
}

impl Environment {
    pub fn is_local(&self) -> bool {
        *self == Environment::Local
    }
    pub fn is_staging(&self) -> bool {
        *self == Environment::Staging
    }
    pub fn is_production(&self) -> bool {
        *self == Environment::Production
    }
}

pub fn current() -> Environment {
    dotenvy::dotenv().ok();

    std::env::var("ARK_ENV").expect("ARK_ENV must be set").into()
}
