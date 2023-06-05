use std::error::Error;

use regex::Regex;

// cargo test android_strgrep_tests
#[cfg(test)]
mod android_strgrep_tests {
    use super::*;

    #[test]
    fn one_result() {
        let query = "Name";
        let content = r#"<string name="string_name">text_string</string>"#;
        let result: Vec<&str> = vec![];
        assert_eq!(result, search(query, content))
    }

    #[test]
    fn case_insensitive() {
        let query = "Name";
        let content = r#"<string name="string_name">text_string</string>"#;
        assert_eq!(vec!["string_name"], search_case_insensitive(query, content))
    }
}

pub fn search<'a>(query: &str, content: &'a str) -> Vec<&'a str> {
    if let Ok(re) = Regex::new(r#"<string name="(.+?)">"#) {
        content
            .lines()
            .filter_map(|line| re.captures(line).and_then(|caps| caps.get(1)))
            .filter(|key| key.as_str().contains(query))
            .map(|key| key.as_str())
            .collect()
    } else {
        vec![]
    }
}

pub fn search_case_insensitive<'a>(query: &str, content: &'a str) -> Vec<&'a str> {
    let query = query.to_lowercase();
    if let Ok(re) = Regex::new(r#"<string name="(.+?)">"#) {
        content
            .lines()
            .filter_map(|line| re.captures(line).and_then(|caps| caps.get(1)))
            .filter(|key| key.as_str().to_lowercase().contains(&query))
            .map(|key| key.as_str())
            .collect()
    } else {
        vec![]
    }
}
