pub fn search<'a>(query: &str, file_text: &'a str) -> Vec<&'a str> {
    let mut results = Vec::new();
    for line in file_text.lines() {
        if line.contains(query) {
            results.push(line);
        }
    }
    results
}

pub fn search_case_insensitive<'a>(query: &str, file_text: &'a str) -> Vec<&'a str> {
    let query = query.to_lowercase();
    let mut results = Vec::new();

    for line in file_text.lines() {
        if line.to_lowercase().contains(&query) {
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
Spaceship that travels at
the speed of light,
spaceship that can help us
find other beings.
On the quest of finding the
answer to life, universe
and everything.";

        assert_eq!(
            vec![
                "Let's build a new spaceship",
                "A spaceship to fly to the",
                "spaceship that can help us",
            ],
            search(query, file_text)
        );
    }

    #[test]
    fn no_lines_match() {
        let query = "banana";
        let file_text = "\
Let's build a new spaceship
A spaceship to fly to the
other side of the universe.";

        let expected: Vec<&str> = Vec::new();
        assert_eq!(expected, search(query, file_text))
    }

    #[test]
    fn case_sensitive() {
        let query = "spaceship";
        let file_text = "\
Let's build a new spaceship
to travel to the other side
of the universe.
Spaceship that travels at
the speed of light.";

        assert_eq!(
            vec!["Let's build a new spaceship"],
            search(query, file_text)
        );
    }

    #[test]
    fn case_insensitive() {
        let query = "hAt";
        let file_text = "\
Hate? No hate!
I'm no hater!
Rust is great.
What do you mean?
Spaceship is due.
Hatching, cracking...";

        assert_eq!(
            vec![
                "Hate? No hate!",
                "I'm no hater!",
                "What do you mean?",
                "Hatching, cracking..."
            ],
            search_case_insensitive(query, file_text)
        );
    }
}
