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
        assert_eq!(result, search(query, content).unwrap())
    }

    #[test]
    fn case_insensitive() {
        let query = "Name";
        let content = r#"<string name="string_name">text_string</string>"#;
        assert_eq!(
            vec!["string_name"],
            search_case_insensitive(query, content).unwrap()
        )
    }
}

pub fn search<'a>(query: &str, content: &'a str) -> Result<Vec<&'a str>, Box<dyn Error>> {
    let mut results = Vec::new();
    let re = Regex::new(r#"<string name="(.+?)">"#)?;
    for line in content.lines() {
        if let Some(caps) = re.captures(line) {
            let key = caps.get(1).map_or("", |m| m.as_str());
            if key.contains(&query) {
                results.push(key);
            }
        }
    }
    Ok(results)
}

pub fn search_case_insensitive<'a>(
    query: &str,
    content: &'a str,
) -> Result<Vec<&'a str>, Box<dyn Error>> {
    let query = query.to_lowercase();
    let mut results = Vec::new();
    let re = Regex::new(r#"<string name="(.+?)">"#)?;
    for line in content.lines() {
        if let Some(caps) = re.captures(line) {
            let key = caps.get(1).map_or("", |m| m.as_str());
            if key.to_lowercase().contains(&query) {
                results.push(key);
            }
        }
    }
    Ok(results)
}
