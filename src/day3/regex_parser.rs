use regex::Regex;

pub fn match_regex(regex_string: &str, value_string: &str) -> Option<Vec<String>> {
    let re = Regex::new(regex_string).ok()?;
    let captures = re.captures(value_string)?;

    let mut groups = vec![];
    for i in 1..captures.len() {
        if let Some(group) = captures.get(i) {
            groups.push(group.as_str().to_string());
        }
    }

    Some(groups)
}