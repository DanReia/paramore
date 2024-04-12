use ::std::path::PathBuf;

// The purpose of the libary is to easily:
// * read a json configuration file from a specified location

struct Config {
    path: PathBuf,
}

impl Config {
    pub fn new(path: PathBuf) -> Config {
        Config { path }
    }

    pub fn exists(&self) {}
}

#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn it_works() {
        let path = PathBuf::from("config.json");
        let mut config: Config = Config::new(path);
    }
}
