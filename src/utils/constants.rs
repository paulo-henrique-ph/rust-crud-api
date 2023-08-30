use std::env;

//DATABASE URL
const DB_URL: &str = env!("DATABASE_URL");

// Keycloak
const KEYCLOAK_URL: &str = env!("KEYCLOAK_URL");
const KEYCLOAK_USERNAME: &str = env!("KEYCLOAK_USERNAME");
const KEYCLOAK_PASSWORD: &str = env!("KEYCLOAK_PASSWORD");

// SERVER
pub const HOSTNAME: &str = env!("HOSTNAME");
pub const PORT: &str = env!("PORT");