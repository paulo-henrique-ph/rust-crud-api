use std::fmt::Debug;

#[derive(Debug, Deserialize)]
pub struct Env {
    pub database_url: String,

    pub keycloak_url: String,
    pub keycloak_username: String,
    pub keycloak_password: String,

    pub new_relic_key: String,

    pub port: i16,
    pub hostname: String,
    pub is_dev: bool,
}

impl Env {
    pub fn load() -> Self {
        let profile = std::env::var("PROFILE").unwrap_or_default();

        if profile.is_empty() {
            dotenv::dotenv().expect("failed to load .env values");
        }

        envy::from_env().expect("failed to parse env vars to Env struct")
    }
}
