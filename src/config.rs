use serde::de::DeserializeOwned;
use serde::Serialize;
use std::default::Default;
use std::error::Error;
use std::fs;
use std::io::BufReader;
use std::path::Path;

#[cfg(feature = "json")]
pub trait JSON {
    fn read(path: &Path) -> Result<Self, Box<dyn Error>>
    where
        Self: Sized + DeserializeOwned + Serialize + Default,
    {
        if !path.exists() {
            let deserialized: Self = self::Default::default();
            Ok(deserialized)
        } else {
            let f = fs::File::open(path)?;
            let reader = BufReader::new(f);
            let deserialized: Self = serde_json::from_reader(reader)?;
            Ok(deserialized)
        }
    }

    fn write(&self, path: &Path) -> Result<(), Box<dyn Error>>
    where
        Self: Sized + Serialize,
    {
        let serialized = serde_json::to_string(&self)?;
        fs::write(path, serialized)?;
        Ok(())
    }
}

#[cfg(feature = "yaml")]
pub trait YAML {
    fn read(path: &Path) -> Result<Self, Box<dyn Error>>
    where
        Self: Sized + DeserializeOwned + Serialize + Default,
    {
        if !path.exists() {
            let deserialized: Self = self::Default::default();
            Ok(deserialized)
        } else {
            let f = fs::File::open(path)?;
            let reader = BufReader::new(f);
            let deserialized: Self = serde_yaml::from_reader(reader)?;
            Ok(deserialized)
        }
    }

    fn write(&self, path: &Path) -> Result<(), Box<dyn Error>>
    where
        Self: Sized + Serialize,
    {
        let serialized = serde_yaml::to_string(&self)?;
        fs::write(path, serialized)?;
        Ok(())
    }
}

mod test_utils {
    use serde::{Deserialize, Serialize};

    #[derive(Serialize, Deserialize, Debug, PartialEq, Default, PartialOrd)]
    pub struct Config {
        pub a: String,
        pub b: bool,
    }
}

#[cfg(feature = "json")]
#[cfg(test)]
mod tests_json {

    use super::*;
    use test_utils::Config;

    impl JSON for Config {}

    #[test]
    fn test_first_read() {
        let path = Path::new("test_first.json");

        let config = <Config as JSON>::read(&path).unwrap();

        assert_eq!(Config::default(), config);
    }

    #[test]
    fn test_write_read() {
        let path = Path::new("test.json");

        let config = Config {
            a: String::from("abc"),
            b: true,
        };

        // config.write(&path).unwrap();
        JSON::write(&config, &path).unwrap();

        let config_new = <Config as JSON>::read(&path).unwrap();

        assert_eq!(config, config_new);

        fs::remove_file("test.json").unwrap();
    }
}

#[cfg(feature = "yaml")]
#[cfg(test)]
mod tests_yaml {

    use super::*;
    use test_utils::Config;

    use crate::YAML;
    impl YAML for Config {}

    #[test]
    fn test_first_read() {
        let path = Path::new("test_first.yaml");

        let config = <Config as YAML>::read(&path).unwrap();

        assert_eq!(Config::default(), config);
    }

    #[test]
    fn test_write_read() {
        let path = Path::new("test.yaml");

        let config = Config {
            a: String::from("abc"),
            b: true,
        };

        // config.write(&path).unwrap();
        YAML::write(&config, &path).unwrap();
        let config_new = <Config as YAML>::read(&path).unwrap();

        assert_eq!(config, config_new);

        fs::remove_file("test.yaml").unwrap();
    }
}
