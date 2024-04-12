use serde::{Deserialize, Serialize};
use std::fs;
use std::io::{BufWriter, Read, Write};
use std::path::Path;

#[derive(Serialize, Deserialize, Debug)]
struct Config {
    a: String,
}

fn struct_to_json<T>(path: &Path, structure: &T)
where
    T: Serialize,
{
    let serialized = serde_json::to_string(structure).unwrap();
    fs::write(path, serialized).expect("Unable to write file");
}

fn read_file_to_string(path: &Path) -> String {
    let mut f = fs::File::open(path).expect("Unable to open file");
    let mut contents = String::new();
    f.read_to_string(&mut contents).unwrap();
    contents
}

fn read_config<'a, T>(path: &Path) -> T
where
    T: Deserialize<'a> + std::fmt::Debug,
{
    let file_string = read_file_to_string(path);
    let deserialized: T = serde_json::from_str(&file_string).unwrap();
    deserialized
}

fn main() {
    let path = Path::new("test.json");
    let config: Config = read_config(path);
    println!("{:?}", config);
}
