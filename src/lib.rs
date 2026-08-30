pub fn search<'a>(query: &str, file_text: &'a str) -> Vec<&'a str> {
    let mut results = Vec::new();
    for line in file_text.lines() {
        if line.contains(query) {
            results.push(line);
        }
    }
    results
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn one_line_matches() {
        let query = "duct";
        let file_text = "\
Rust:
safe, fast, productive.
Pick three.";

        assert_eq!(vec!["safe, fast, productive."], search(query, file_text));
    }

    #[test]
    fn four_line_match() {
        let query = "spaceship";
        let file_text = "\
Let's build a new spaceship
A spaceship to fly to the
other side of the universe.
A spaceship that travels at
the speed of light.
A spaceship that can help us
find other beings.
On the quest of finding the
answer to life, universe
and everything.";

        assert_eq!(
            vec![
                "Let's build a new spaceship",
                "A spaceship to fly to the",
                "A spaceship that travels at",
                "A spaceship that can help us",
            ],
            search(query, file_text)
        );
    }
}
