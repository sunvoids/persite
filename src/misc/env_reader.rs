use std::{collections::HashMap, error::Error};

pub fn get_env(path: &str) -> Result<HashMap<String, String>, Box<dyn Error>> {
    let map = std::fs::read_to_string(path)?
        .lines()
        .filter(|line| !line.trim().is_empty() && !line.trim_start().starts_with('#'))
        .filter_map(|line| {
            let (key, value) = line.split_once('=')?;
            Some((key.trim().to_string(), value.trim().to_string()))
        })
        .collect::<HashMap<_, _>>();
    Ok(map)
}