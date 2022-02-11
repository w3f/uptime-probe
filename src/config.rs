extern crate serde_yaml;

use std::path::Path;

#[derive(Debug, Deserialize)]
pub struct Site {
    pub url: String,
}

#[derive(Debug, Deserialize)]
pub struct Config {
    pub period: u32,
    pub port: u32,
    pub allow_redirections: bool,
    pub prometheus_rule_scope: String,
    pub sites: Vec<Site>,
}

impl Config {
    pub fn new(args: &[String]) -> Result<Config, &'static str> {
        if args.len() < 2 {
            return Err("Please provide a config path.");
        }
        if ! Path::new(&args[1]).exists() {
            return Err("Please, provide a valid file path.");
        }
        if let Ok(f) = std::fs::File::open(&args[1]) {
            if let Ok(cfg) = serde_yaml::from_reader(f) {
                Ok(cfg)
            }
            else {
                Err("Please, provide a valid YAML file.")
            }
        } else {
            Err("Please, provide a valid file path.")
        }
    }
}

#[cfg(test)]
mod tests{
    extern crate tempfile;

    use super::*;
    use std::io::prelude::*;

    #[test]
    fn not_given_config_fails() {
        let args = Vec::new();

        let result = Config::new(&args);
        assert!(result.is_err());

        if let Err(msg) = result {
            assert_eq!(msg, "Please provide a config path.");
        }
    }

    #[test]
    fn not_given_file_fails() {
        let args = vec!["bin".to_string(), "cfg.yaml".to_string()];
        let result = Config::new(&args[..]);
        assert!(result.is_err());

        if let Err(msg) = result {
            assert_eq!(msg, "Please, provide a valid file path.")
        }
    }

    #[test]
    fn not_given_yaml_config_fails() {
        let mut tmpfile = tempfile::NamedTempFile::new().unwrap();
        write!(tmpfile, "not real YAML content...").unwrap();

        let path = tmpfile.path();
        let args = vec!["bin".to_string(), path.to_str().unwrap().to_string()];

        let result = Config::new(&args[..]);
        assert!(result.is_err());

        if let Err(msg) = result {
            assert_eq!(msg, "Please, provide a valid YAML file.")
        }
    }

    #[test]
    fn valid_yaml_succeeds() {
        let mut tmpfile = tempfile::NamedTempFile::new().unwrap();

        write!(tmpfile, r#"
period: 300
port: 8080
allow_redirections: false
prometheus_rule_scope: std
sites:
- url: "https://polkadot.network/"
- url: "https://web3.foundation/"
"#).unwrap();

        let path = tmpfile.path();
        let args = vec!["bin".to_string(), path.to_str().unwrap().to_string()];

        let result = Config::new(&args[..]);
        assert!(result.is_ok());

        if let Ok(cfg) = result {
            assert_eq!(cfg.period, 300);
            assert_eq!(cfg.port, 8080);
            assert_eq!(cfg.sites.len(), 2);
            assert_eq!(cfg.sites[0].url, "https://polkadot.network/");
            assert_eq!(cfg.sites[1].url, "https://web3.foundation/");
        }
    }
}
