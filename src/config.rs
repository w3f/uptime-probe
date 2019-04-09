extern crate serde_yaml;

use std::path::Path;

#[derive(Debug, Deserialize)]
pub struct Site {
    pub url: String,
    pub needles: Vec<String>,
}

#[derive(Debug, Deserialize)]
pub struct Config {
    pub period: u32,
    pub port: u32,
    pub sites: Vec<Site>,
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

        write!(tmpfile, r#"
period: 300
port: 8080
sites:
- url: "https://polkadot.network/"
  needles: ["multi-chain", "interoperability", "scalability"]
- url: "https://web3.foundation/"
  needles: ["decentralized", "protocols", "projects"]
"#).unwrap();

        let path = tmpfile.path();
        let args = vec![path.to_str().unwrap().to_string()];

        let result = Config::new(&args[..]);
        assert!(result.is_ok());

        if let Ok(cfg) = result {
            assert_eq!(cfg.period, 300);
            assert_eq!(cfg.port, 8080);
            assert_eq!(cfg.sites.len(), 2);
            assert_eq!(cfg.sites[0].url, "https://polkadot.network/");
            assert_eq!(cfg.sites[0].needles.len(), 3);
            assert_eq!(cfg.sites[0].needles[0], "multi-chain");
            assert_eq!(cfg.sites[0].needles[1], "interoperability");
            assert_eq!(cfg.sites[0].needles[2], "scalability");
            assert_eq!(cfg.sites[1].url, "https://web3.foundation/");
            assert_eq!(cfg.sites[1].needles.len(), 3);
            assert_eq!(cfg.sites[1].needles[0], "decentralized");
            assert_eq!(cfg.sites[1].needles[1], "protocols");
            assert_eq!(cfg.sites[1].needles[2], "projects");
        }
    }
}
