pub mod config {

    //! # config module
    //! `config module` deals with how to parse input form the user with the allowed predefine Config struct

    use std::env::{self};

    #[derive(Debug)]
    pub struct Config {
        pub query: String,
        pub file_path: String,
        pub ignore_case: bool,
    }

    impl Config {
        fn build(mut args: impl Iterator<Item = String>) -> Result<Config, &'static str> {
            args.next();

            let mut args = args.into_iter();

            let query = match args.next() {
                Some(q) => q,
                None => return Err("Didn't get a query string"),
            };

            let file_path = match args.next() {
                Some(f) => f,
                None => return Err("Didn't get a file path"),
            };

            let ignore_case = env::var("IGNORE_CASE").is_ok();

            Ok(Config {
                query,
                file_path,
                ignore_case,
            })
        }
    }

    /// Parse config form the cli and return the Config struct
    ///
    /// # Example
    /// ```
    /// use minigrep::config;
    /// let cfg = config::parse_config().unwrap_or_else(|err| {
    ///     config::Config {
    ///     query: "".to_string(),
    ///     file_path: "".to_string(),
    ///     ignore_case: true,
    ///     }
    /// });
    /// ```
    ///
    pub fn parse_config() -> Result<Config, &'static str> {
        Config::build(env::args())
    }
}

pub mod file {
    //! # file module
    //! this module is responsible for the file operation and search functionality
    //!

    use std::{error::Error, fs};

    pub fn get_contents(config: &super::config::Config) -> Result<String, Box<dyn Error>> {
        let contents = fs::read_to_string(&config.file_path)?;

        Ok(contents)
    }

    fn search_content<'a>(query: &str, content: &'a str) -> Vec<&'a str> {
        content
            .lines()
            .into_iter()
            .filter(|line| line.contains(query))
            .collect()
    }

    fn search_case_insensitive<'a>(query: &str, content: &'a str) -> Vec<&'a str> {
        content
            .lines()
            .into_iter()
            .filter(|line| {
                line.to_ascii_lowercase()
                    .contains(&query.to_ascii_lowercase())
            })
            .collect()
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
