pub fn search<'a>(query: &str, file_text: &'a str) -> impl Iterator<Item = &'a str> {
    file_text.lines().filter(move |line| line.contains(query))
}

pub fn search_case_insensitive<'a>(
    query: &str,
    file_text: &'a str,
) -> impl Iterator<Item = &'a str> {
    let query = query.to_lowercase();
    file_text
        .lines()
        .filter(move |line| line.to_lowercase().contains(&query))
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

        let actual: Vec<_> = search(query, file_text).collect();
        assert_eq!(vec!["safe, fast, productive."], actual);
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

        let actual: Vec<_> = search(query, file_text).collect();
        assert_eq!(
            vec![
                "Let's build a new spaceship",
                "A spaceship to fly to the",
                "spaceship that can help us",
            ],
            actual
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
        let actual: Vec<_> = search(query, file_text).collect();
        assert_eq!(expected, actual);
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

        let actual: Vec<_> = search(query, file_text).collect();
        assert_eq!(vec!["Let's build a new spaceship"], actual);
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

        let actual: Vec<_> = search_case_insensitive(query, file_text).collect();
        assert_eq!(
            vec![
                "Hate? No hate!",
                "I'm no hater!",
                "What do you mean?",
                "Hatching, cracking..."
            ],
            actual
        );
    }
}
