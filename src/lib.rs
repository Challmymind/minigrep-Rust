use std::{fs, error::Error};
pub mod parser;

pub fn search<'a>(
    contents : &'a str, 
    term : &str,
) -> Vec<&'a str> {
    contents
        .lines()
        .filter(|x| x.contains(term))
        .collect()
}

pub fn search_insens<'a>(
    contents : &'a str,
    term : &str,
) -> Vec<&'a str> {

    let lowterm = term.to_lowercase();

    contents
        .lines()
        .filter(|x| x.to_lowercase().contains(&lowterm))
        .collect()

}

pub fn run(config : parser::Config) -> Result<(), Box<dyn Error>> {
    
    
    let contents = fs::read_to_string(config.file_path())?;
    
    let result = if *config.ignore_case() {
        search_insens(&contents, config.term())
    } else {
        search(&contents, config.term())
    };
    
    for item in result {
        println!("{}\n", item);
    }
    
    Ok(())
}

#[cfg(test)]
mod tests {
    #[test]
    fn search_test() {
        let term = "duct".to_string();
        let contents = {"\
contens, strings,
productivity, things.
Rust: "}.to_string();

        let result = crate::search(&contents, &term);
        assert_eq!(vec!["productivity, things."], *result);
    }

    #[test]
    fn search_false_test() {
        let term = "duct".to_string();
        let contents = {"\
contens, strings,
productivity, things.
Rust: "}.to_string();

        assert_ne!(vec![&contents], *crate::search(&contents, &term));
    }

    #[test]
    fn search_case_sensitive() {
        let term = "RuSt".to_string();
        let contents = {"\
contens, strings,
productivity, things.
Rust: "}.to_string();

        assert_eq!(Vec::<&str>::new(), *crate::search(&contents, &term));
    }

    #[test]
    fn search_case_insensitive() {
        let term = "RuSt".to_string();
        let contents = {"\
contens, strings,
productivity, things.
Rust: "}.to_string();

        assert_eq!(vec!["Rust: "], *crate::search_insens(&contents, &term));
    }
}