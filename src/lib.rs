use serde::de::DeserializeOwned;
use serde::Serialize;
use std::default::Default;
use std::error::Error;
use std::fs;
use std::io::BufReader;
use std::path::Path;

pub fn read_config<T>(path: &Path) -> Result<T, Box<dyn Error>>
where
    T: DeserializeOwned + Default + Serialize,
{
    if !path.exists() {
        write_config(path, &T::default())?
    }
    let f = fs::File::open(path)?;
    let reader = BufReader::new(f);
    let deserialized: T = serde_json::from_reader(reader)?;
    Ok(deserialized)
}

pub fn write_config<T>(path: &Path, structure: &T) -> Result<(), Box<dyn Error>>
where
    T: Serialize,
{
    let serialized = serde_json::to_string(structure)?;
    fs::write(path, serialized)?;
    Ok(())
}

#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn test_first_read() {
        use serde::{Deserialize, Serialize};

        #[derive(Serialize, Deserialize, Debug, PartialEq, Default)]
        struct Config {
            a: String,
            b: bool,
        }

        let path = Path::new("test_first.json");

        let config_new: Config = read_config(path).unwrap();

        assert_eq!(Config::default(), config_new);

        fs::remove_file("test_first.json").unwrap();
    }

    #[test]
    fn test_json() {
        use serde::{Deserialize, Serialize};

        #[derive(Serialize, Deserialize, Debug, PartialEq, Default)]
        struct Config {
            a: String,
            b: bool,
        }

        let path = Path::new("test.json");

        let config = Config {
            a: String::from("abc"),
            b: true,
        };

        write_config(path, &config).unwrap();

        let config_new: Config = read_config(path).unwrap();

        assert_eq!(config, config_new);

        fs::remove_file("test.json").unwrap();
    }
}
