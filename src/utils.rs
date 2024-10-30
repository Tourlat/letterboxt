use regex::Regex;

pub fn is_url(param: &str) -> bool {
    let re = Regex::new(r"^https://").unwrap();
    re.is_match(param)
}
