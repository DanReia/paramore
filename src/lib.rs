use serde::de::DeserializeOwned;
use serde::Serialize;
use std::fs;
use std::io::Read;
use std::path::Path;

fn read_file_to_string(path: &Path) -> String {
    let mut f = fs::File::open(path).expect("Unable to open file");
    let mut contents = String::new();
    f.read_to_string(&mut contents).unwrap();
    contents
}

pub fn read_config<T>(path: &Path) -> T
where
    T: DeserializeOwned,
{
    let file_string = read_file_to_string(path);
    let deserialized: T = serde_json::from_str(&file_string).unwrap();
    deserialized
}

pub fn write_config<T>(path: &Path, structure: &T)
where
    T: Serialize,
{
    let serialized = serde_json::to_string(structure).unwrap();
    fs::write(path, serialized).expect("Unable to write file");
}

#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn it_works() {
        use serde::{Deserialize, Serialize};

        #[derive(Serialize, Deserialize, Debug, PartialEq)]
        struct Config {
            a: String,
        }

        let path = Path::new("test.json");

        let config = Config {
            a: String::from("abc"),
        };

        write_config(path, &config);

        let config_new: Config = read_config(path);

        assert_eq!(config, config_new);

        fs::remove_file("test.json").unwrap();
    }
}
