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

        Ok(contents)
    }

    pub fn search_content<'a>(query: &str, content: &'a str) -> Vec<&'a str> {
        let mut results: Vec<&str> = vec![];

        for line in content.lines() {
            if line.contains(query) {
                results.push(line);
            }
        }

        results
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn one_result() {
        let query = "duct";
        let content = "\
        Rust:
        safe, fast, productive.
        Pick three.";

        assert_eq!(
            vec!["        safe, fast, productive."],
            file::search_content(query, &content)
        );
    }
}
