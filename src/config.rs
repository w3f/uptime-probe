extern crate serde_yaml;
extern crate tempfile;

use std::path::Path;

#[derive(Debug, Deserialize)]
pub struct Config {
    period: String,
}

impl Config {
    pub fn new(args: &[String]) -> Result<Config, &'static str> {
        if args.len() == 0 {
            return Err("Please provide a config path.");
        }
        if ! Path::new(&args[0]).exists() {
            return Err("Please, provide a valid file path.");
        }
        if let Ok(f) = std::fs::File::open(&args[0]) {
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
        let args = vec!["cfg.yaml".to_string()];
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
        let args = vec![path.to_str().unwrap().to_string()];

        let result = Config::new(&args[..]);
        assert!(result.is_err());

        if let Err(msg) = result {
            assert_eq!(msg, "Please, provide a valid YAML file.")
        }
    }

    #[test]
    fn valid_yaml_succeeds() {
        let mut tmpfile = tempfile::NamedTempFile::new().unwrap();
        write!(tmpfile, "period: 5m").unwrap();

        let path = tmpfile.path();
        let args = vec![path.to_str().unwrap().to_string()];

        let result = Config::new(&args[..]);
        assert!(result.is_ok());

        if let Ok(cfg) = result {
            assert_eq!(cfg.period, "5m")
        }
    }
}
