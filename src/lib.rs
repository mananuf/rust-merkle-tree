pub struct Config {
    pub command: String,
    pub file_path: String,
}

impl Config {
    fn new(args: &[String]) -> Self {
        Self {
            command: args[1].clone(),
            file_path: args[2].clone()
        }
    }
}

pub fn parse_config(args: &[String]) -> Config {
    let config: Config = Config::new(&args);
    config
}