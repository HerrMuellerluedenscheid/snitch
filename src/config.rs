use serde::{Deserialize, Serialize};
use std::path::Path;

use crate::notifiers::NotificationConfig;

#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub struct Config {
    pub directories: Vec<String>,
    pub database_path: String,
    pub watch_authentication_logs: bool,
    pub notifications: NotificationConfig,
}

impl Config {
    pub fn database_path(&self) -> &Path {
        Path::new(&self.database_path)
    }

    pub fn directories(&self) -> Vec<&Path> {
        let mut paths = vec![];
        for dir in self.directories.iter() {
            paths.push(Path::new(dir));
        }
        paths
    }

    pub fn demo_config() -> Config {
        Config {
            directories: vec!["asdf/asdf".to_owned()],
            database_path: "/etc/nitro/db".to_owned(),
            watch_authentication_logs: false,
            notifications: NotificationConfig{
                enable_email: true,
                enable_telegram: true,
            },
        }
    }
}

pub fn load_config_from_file(path: &Path) -> Result<Config, serde_yaml::Error> {
    let reader = std::fs::File::open(path).unwrap();
    let config = serde_yaml::from_reader(reader)?;
    Ok(config)
}

pub fn print_basic_config() {
    let config = Config::demo_config();
    println!("{}", serde_yaml::to_string(&config).unwrap());
}

#[cfg(test)]
mod tests {
    use crate::config::Config;

    #[test]
    fn test_basic_config() {
        let x = Config::demo_config();
    }
}
