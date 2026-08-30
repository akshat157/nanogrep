pub fn search<'a>(query: &str, file_text: &'a str) -> Vec<&'a str> {
    for line in file_text.lines() {
        // TODO: implement the matching logic here
    }
    vec![]
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn one_result() {
        let query = "duct";
        let file_text = "\
        Rust:
        safe, fast, productive.
        Pick three.";

        assert_eq!(vec!["safe, fast, productive"], search(query, file_text));
    }
}
