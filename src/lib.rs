use ::std::path::PathBuf;

struct Config;

impl Config {
    pub fn new(path: PathBuf) -> Config {
        Config
    }
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
