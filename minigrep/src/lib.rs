pub mod config {
    use std::env;

    #[derive(Debug)]
    pub struct Config {
        pub query: String,
        pub file_path: String,
    }

    impl Config {
        fn build(args: &[String]) -> Result<Config, &'static str> {
            if args.len() < 3 {
                return Err("not enough arguments!");
            }

            Ok(Config {
                query: args[1].clone(),
                file_path: args[2].clone(),
            })
        }
    }

    pub fn parse_config() -> Result<Config, &'static str> {
        let args: Vec<String> = env::args().collect();

        Config::build(&args)
    }
}

pub mod file {
    use std::{error::Error, fs};

    pub fn get_contents(config: &super::config::Config) -> Result<String, Box<dyn Error>> {
        let contents = fs::read_to_string(&config.file_path)?;

        print!("With text:\n{contents}");

        Ok(contents)
    }
}
