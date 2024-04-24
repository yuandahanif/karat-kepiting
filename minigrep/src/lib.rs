pub mod config {
    use std::env;

    #[derive(Debug)]
    pub struct Config {
        query: String,
        file_path: String,
    }

    impl Config {
        pub fn new(args: &[String]) -> Config {
            Config {
                query: args[1].clone(),
                file_path: args[2].clone(),
            }
        }
    }

    pub fn parse_config() -> Config {
        let args: Vec<String> = env::args().collect();
        Config::new(&args)
    }
}
