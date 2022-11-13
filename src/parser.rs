#[allow(dead_code)]
pub struct Config {
    term : String,
    file_path : String,
    ignore_case : bool,
}

impl Config {

    pub fn new(
        mut args : impl Iterator<Item = String>,
    ) -> Result<Config, &'static str> {
        
        use std::env;

        args.next();
        let val1 = match args.next() {
            Some(x) => x,
            None => return Err("No search term"),
        };


        let val2 = match args.next() {
            Some(x) => x,
            None => return Err("No filepath"),
        };

        let ignore = env::var("IGNORE_CASE").is_ok();

        Ok(Config {
            term : val1,
            file_path : val2,
            ignore_case : ignore,
        })
    }

    pub fn term<'a>(&'a self) -> &'a String {
        &self.term
    }

    pub fn file_path<'a>(&'a self) -> &'a String {
        &self.file_path
    }

    pub fn ignore_case<'a>(&'a self) -> &'a bool {
        &self.ignore_case
    }
}




#[cfg(test)]
mod tests {
    #[test]
    fn config_test(){
        use crate::parser;

        let x : Vec<String> = vec!["boilerplate".to_string(), "napis".to_string(), "napis2".to_string()];

        let result = parser::Config::new(x.into_iter());
        assert_eq!(result.as_ref().unwrap().term, "napis");
        assert_eq!(result.as_ref().unwrap().file_path, "napis2");
    }
}