use std::{collections::HashMap, error::Error};

pub fn read_env(path: &str) -> Result<DatabaseInfo, Box<dyn Error>> {
    let mut map = std::fs::read_to_string(path)?
        .lines()
        .filter(|line| !line.trim().is_empty() && !line.trim_start().starts_with('#'))
        .filter_map(|line| {
            let (key, value) = line.split_once('=')?;
            Some((key.trim().to_string(), value.trim().to_string()))
        })
        .collect::<HashMap<_, _>>();

    // Removing here to take ownership.
    let username = map.remove("DB_USERNAME").unwrap_or("user".to_string());
    let password = map.remove("DB_PASSWORD").unwrap_or("password".to_string());
    let hostname = map.remove("DB_HOST").unwrap_or("localhost".to_string());
    let port = map.remove("DB_PORT").unwrap_or("5432".to_string());
    let host = format!("{}:{}", hostname, port);

    Ok(DatabaseInfo {
        username,
        password,
        host,
    })
}

pub struct DatabaseInfo {
    pub username: String,
    pub password: String,
    pub host: String,
}
