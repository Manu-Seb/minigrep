use std::env;

pub struct Config {
    string: String,
    filepath: String,
    ignore_option: bool,
}

impl Config {
    pub fn get_string(&self) -> &str {
        &self.string
    }
    pub fn get_filepath(&self) -> &str {
        &self.filepath
    }
    pub fn get_option(&self) -> &bool {
        &self.ignore_option
    }
    pub fn new(args: &[String]) -> Result<Config, &'static str> {
        if args.len() < 3 {
            return Err("Not enough paramenters");
        }
        let string = args[1].clone();
        let filepath = args[2].clone();
        let mut ignore_option = true;
        if args.len() > 3 {
            let opt: u32 = args[3].trim().parse().expect("could not read file");
            if opt == 1 {
                ignore_option = false;
            }
        }

        // let ignore_option = env::var("IGNORE_OPTION").is_ok();

        Ok(Config {
            string,
            filepath,
            ignore_option,
        })
    }
}

pub fn search<'a>(content: &'a str, string: &str) -> Vec<&'a str> {
    let mut arr = vec![];

    for line in content.lines() {
        if line.contains(string) {
            arr.push(line);
        }
    }

    return arr;
}
pub fn search_case_insensitive<'a>(content: &'a str, string: &str) -> Vec<&'a str> {
    let mut arr = vec![];

    for line in content.lines() {
        if line.to_lowercase().contains(&string.to_lowercase()) {
            arr.push(line);
        }
    }

    return arr;
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn search_get_value() {
        let string = "lmfao";
        let content = "\
        I dont know\nWhat to do\nWith my life\nLMfao and lmfao";

        assert_eq!(vec!["LMfao and lmfao"], search(content, string));
    }
    #[test]
    fn case_sensitive() {
        let query = "duct";
        let contents = "Rust:\nsafe, fast, productive.\nPick three.\nDuct tape.";

        assert_eq!(vec!["safe, fast, productive."], search(contents, query));
    }

    #[test]
    fn case_insensitive() {
        let query = "rUsT";
        let contents = "Rust:\nsafe, fast, productive.\nPick three.\nTrust me.";

        assert_eq!(
            vec!["Rust:", "Trust me."],
            search_case_insensitive(contents, query)
        );
    }
}
