use std::{error::Error, fs, env};

#[derive(PartialEq, Debug)]
pub struct Config {
    pub query: String,
    pub filename: String,
    pub case_sensitive: bool,
}

impl Config {
    pub fn new<I>(mut args: I) -> Result<Config, &'static str> 
    where 
    I: Iterator<Item = String>,
    {  
        args.next();

        let query = match args.next() {
            Some(arg) => arg,
            None => return Err("Didn't get an query string")
        };

        let filename = match args.next() {
            Some(arg) => arg,
            None => return Err("Didn't get a file name")
        };

        let case_sensitive = env::var("CASE_INSENSITIVE").is_err();

        Ok(Config { query, filename, case_sensitive })
    }
}

pub fn run(config: Config) -> Result<(), Box<dyn Error>> {
    let contents = fs::read_to_string(config.filename)?; // directly read the file content into a variable

    let results = if config.case_sensitive {
        search(&config.query, &contents)
    } else {
        search_case_insensitive(&config.query, &contents)
    };

    for res in results{
        println!("{}", res);
    }
    Ok(())
}

pub fn search<'a>(query: &str, contents: &'a str) -> Vec<&'a str> {
    contents
        .lines()
        .filter(|line| line.contains(query))
        .collect()
}

pub fn search_case_insensitive<'a>(query: &str, contents: &'a str) -> Vec<&'a str> {
    let query = query.to_lowercase();
    contents
        .lines()
        .filter(|line| line.to_lowercase().contains(&query))
        .collect()
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn not_enough_args(){
        let args: Vec<String> = vec!["hello".into(), "hi".into()];
        let err = Config::new(args.into_iter()).unwrap_err();
        assert_eq!(err, "Didn't get a file name");
    }

    #[test]
    fn enough_args(){
        let args = vec![
            "hello".into(),
            "Query".into(),
            "Filename".into(),
        ];

        let config = Config::new(args.into_iter()).expect("Expected valid config");

        let expected = Config {
            query: "Query".into(),
            filename: "Filename".into(),
            case_sensitive: true,
        };

        assert_eq!(config, expected);
    }

    #[test]
    fn case_sensitive() {
        let query = "duct";
        let contents = "
Rust:
This is quite the language
Duct
safe, fast, productive
        ";
        
        assert_eq!(
        vec!["safe, fast, productive"],
        search(query, contents)
    );
    }

    #[test]
    fn case_insensitive() {
        let query = "rUsT";
        let contents = "
Rust:
This is quite the language
safe, fast, productive
Trust me.
        ";
        assert_eq!(
            vec!["Rust:", "Trust me."],
            search_case_insensitive(query, contents)
        )
    }
}
