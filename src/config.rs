use directories::BaseDirs;
use serde_derive::{Deserialize, Serialize};
use std::env;
use std::fs;
use std::path::PathBuf;
use std::string::String;

/// Holds the state of the app
///
/// stored in the $HOME/.ptl file
#[derive(Serialize, Deserialize, Debug)]
pub struct Config {
    email: String,
}

impl Config {
    /// Create a new default State object
    pub fn default() -> Config {
        Config {
            email: "anonymous".to_string(),
        }
    }


    /// Gather the config info
    /// 
    /// Gather the confing info from a toml formated file. The file is stored in the
    /// platform dependant directory computed by the directories crate.
    /// 
    /// # Panics
    /// 
    /// If the config file can't be found or created
    pub fn gather() -> Config {
        let mut config = Config::default();

        // Build the path for the users config dir.
        if let Some(mut cfg_path) = get_config_path() {
            
            cfg_path.push("config");

            // If we cant read the file we fill in defaults for State
            if cfg_path.is_file() {
                let toml_data: String = fs::read_to_string(&cfg_path).unwrap();
                config = match toml::from_str(&toml_data) {
                    Ok(s) => s,
                    _ => config,
                }
            }
        }

        config
    
    }

    /// Returns the email valueconfig
    pub fn email(&self) -> &String {
        &self.email
    }
}

/// Builds the path to the configuration directory and ensure that is exists
///
/// The users global configuration file location is BaseDirs::config_dir as
/// computed by the directories crate.
pub fn get_config_path() -> Option<PathBuf> {
    let dir_name = format!(".{}", env!("CARGO_BIN_NAME"));
 
    // Build the path for the users config dir.
    Some(BaseDirs::new().unwrap().config_dir().join(dir_name))

}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_state_default() {
        let cfg = super::Config::default();
        assert_eq!(cfg.email, "anonymous".to_string())
    }

    #[test]
    fn test_config_email() {
        let mut cfg = Config::default();
        cfg.email = "unknown@unknown.unk".to_string();
        assert_eq!(*cfg.email(), "unknown@unknown.unk".to_string());
    }

    #[test]
    fn test_config_gather() {
        let config = Config::gather();
        assert_ne!( *config.email(), String::from("anonymous"));
    }
}
