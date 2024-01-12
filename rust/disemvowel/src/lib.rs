use regex::Regex;

fn disemvowel(s: &str) -> String {
    let re = Regex::new(r"(?i)[aeiou]").unwrap();
    let result = re.replace_all(s, "");
    result.to_string()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example_test() {
        assert_eq!(disemvowel("This website is for losers LOL!"), "Ths wbst s fr lsrs LL!");
    }
}
