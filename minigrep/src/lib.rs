pub mod config {
    use std::env;

    #[derive(Debug)]
    pub struct Config {
        pub query: String,
        pub file_path: String,
        pub ignore_case: bool,
    }

    impl Config {
        fn build(args: &[String]) -> Result<Config, &'static str> {
            if args.len() < 3 {
                return Err("not enough arguments!");
            }

            let ignore_case = env::var("IGNORE_CASE").is_ok();

            Ok(Config {
                query: args[1].clone(),
                file_path: args[2].clone(),
                ignore_case,
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

    fn search_content<'a>(query: &str, content: &'a str) -> Vec<&'a str> {
        let mut results: Vec<&str> = vec![];

        for line in content.lines() {
            if line.contains(query) {
                results.push(line);
            }
        }

        results
    }

    fn search_case_insensitive<'a>(query: &str, content: &'a str) -> Vec<&'a str> {
        let mut results: Vec<&str> = vec![];

        for line in content.lines() {
            if line
                .to_ascii_lowercase()
                .contains(&query.to_ascii_lowercase())
            {
                results.push(line);
            }
        }

        results
    }

    pub fn run(cfg: &super::config::Config, contents: &str) -> Result<(), Box<dyn Error>> {
        let results = if cfg.ignore_case {
            search_case_insensitive(&cfg.query, &contents)
        } else {
            search_content(&cfg.query, &contents)
        };

        for line in results {
            println!("{line}");
        }

        Ok(())
    }

    #[test]
    fn case_sensitive() {
        let query = "duct";
        let content = "\
        Rust:
        safe, fast, productive.
        Pick three.
        Duct tape.";

        assert_eq!(
            vec!["        safe, fast, productive."],
            search_content(query, &content)
        );
    }

    #[test]
    fn case_insensitive() {
        let query = "rUsT";
        let contents = "\
        Rust:
        safe, fast, productive.
        Pick three.
        Trust me.";

        assert_eq!(
            vec!["Rust:", "        Trust me."],
            search_case_insensitive(query, &contents)
        );
    }
}
