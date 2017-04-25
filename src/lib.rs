use std::fs::File;
use std::io::prelude::*;
use std::error::Error;
use std::env;

pub fn grep<'a> (search: &str, contents: &'a str) -> Vec<&'a str>{
    let mut result = Vec::new();
    for line in contents.lines() {
        if line.contains(search) {
            result.push(line);
        }
    }
    result
}

pub fn grep_case_insensitive<'a>(search: &str, contents: &'a str) -> Vec<&'a str>{
    let mut result = Vec::new();
    let search = search.to_lowercase();
    for line in contents.lines() {
        if line.to_lowercase().contains(&search){
            result.push(line);
        }
    }

    result
}

#[cfg(test)]
mod tests {
    use {grep,grep_case_insensitive};
    
    #[test]
    fn one_result() {
        let search = "fast";
        let contents = "\
Rust:
safe fast productive.
pick three";

        assert_eq!(vec!["safe fast productive."],grep(search,contents) );
    }

    #[test]
    fn case_insensitive() {
        let search = "rust";
        let content = "\
Rust:
safe fast, productive
Pick three.
Trust me.";
        assert_eq!(vec!["Rust:","Trust me."],
            grep_case_insensitive(search,content));

    }


}





pub struct Config{
    pub search: String,
    pub file: String,
    pub case_sensitive: bool,
}

impl Config{
    pub fn new(args: &[String]) -> Result<Config, &'static str>{

        if args.len() < 3 {
            return Err("not enough parameters");
        }

        let search = args[1].clone();
        let filename = args[2].clone();
        let mut case_sensitive = true;
        for (name,_) in env::vars() {
            if name == "CASE_INSENSITIVE" {
                case_sensitive = false;
            }
        }

        Ok(Config{
            search: search,
            file: filename,
            case_sensitive: case_sensitive,
        })
    }
}


pub fn run(config: Config) -> Result<(),Box<Error>>{
    let mut f = File::open(config.file)?;

    let mut contents = String::new();
    f.read_to_string(&mut contents)?;


    let results = if config.case_sensitive {
        grep(&config.search, &contents)
    } else {
        grep_case_insensitive(&config.search, &contents)
    };

    for line in results {
        println!("{}", line);
    }
    
    Ok(())
}