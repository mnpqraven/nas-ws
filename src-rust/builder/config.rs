use serde::{Deserialize, Serialize};
use std::fs;
use std::io::Error as IoError;

#[derive(Serialize, Deserialize, Debug)]
struct ConfigToml {
    database: Option<ConfigTomlDatabase>,
}

#[derive(Serialize, Deserialize, Debug)]
struct ConfigTomlDatabase {
    url: Option<String>,
    auth_token: Option<String>,
}

#[derive(Debug)]
pub struct EnvConfig {
    pub db_url: String,
    pub db_auth_token: String,
}

impl Default for EnvConfig {
    fn default() -> Self {
        Self::new()
    }
}

impl EnvConfig {
    pub fn new() -> Self {
        let filepaths: [&str; 2] = ["./config.toml", "./Config.toml"];
        let mut content: String = "".to_owned();

        for filepath in filepaths {
            let result: Result<String, IoError> = fs::read_to_string(filepath);

            if let Ok(inner) = result {
                content = inner;
                break;
            }
        }

        let config_toml: ConfigToml = toml::from_str(&content).unwrap_or_else(|_| {
            println!("Failed to create ConfigToml Object out of config file.");
            ConfigToml { database: None }
        });

        let (url, auth_token): (String, String) = match config_toml.database {
            Some(database) => {
                let db_url: String = database.url.unwrap_or_else(|| {
                    println!("Missing field `url` in table `database`.");
                    "unknown".to_owned()
                });

                let db_auth_token: String = database.auth_token.unwrap_or_else(|| {
                    println!("Missing field `auth_token` in table `database`.");
                    "unknown".to_owned()
                });

                (db_url, db_auth_token)
            }
            None => {
                println!("Missing table `database`.");
                ("unknown".to_owned(), "unknown".to_owned())
            }
        };

        EnvConfig {
            db_url: url,
            db_auth_token: auth_token,
        }
    }
}
