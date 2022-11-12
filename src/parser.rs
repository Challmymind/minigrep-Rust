#[allow(dead_code)]
pub struct Config<'a> {
    term : &'a String,
    file_path : &'a String,
    ignore_case : bool,
}

impl Config<'_> {

    pub fn new<'a>(
        val1 : &'a String,
        val2 : &'a String
    ) -> Config<'a> {

        use std::env;
        let ignore = env::var("IGNORE_CASE").is_ok();

        Config {
            term : val1,
            file_path : val2,
            ignore_case : ignore,
        }
    }

    pub fn term<'a>(&'a self) -> &'a String {
        self.term
    }

    pub fn file_path<'a>(&'a self) -> &'a String {
        self.file_path
    }

    pub fn ignore_case<'a>(&'a self) -> &'a bool {
        &self.ignore_case
    }
}


#[allow(dead_code)]
pub fn config<'a>(input : &'a [String]) -> Result<Config, &'static str> {

    let result;
    if let (Option::Some(term), Option::Some(path)) = (input.get(1), input.get(2)) {
        result = Config::new(term, path);
        return  Ok(result);
    }
    return Err("Not enough arguments");
}


#[cfg(test)]
mod tests {
    #[test]
    fn config_test(){
        use crate::parser;

        let x : Vec<String> = vec!["boilerplate".to_string(), "napis".to_string(), "napis2".to_string()];

        let result = parser::config(&x);
        assert_eq!(result.as_ref().unwrap().term, "napis");
        assert_eq!(result.as_ref().unwrap().file_path, "napis2");
    }
}