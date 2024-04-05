#![allow(dead_code)]
use regex::Regex;

pub fn validate_slug(slug: &str) -> bool {
    let re = Regex::new(r"^[a-z0-9]+(?:-[a-z0-9]+)*$").unwrap();

    re.is_match(slug)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn should_return_true_when_slug_is_valid() {
        let slug = "hello-world".to_string();
        let result = validate_slug(&slug);

        assert!(result == true);
    }

    #[test]
    fn should_return_false_when_slug_is_invalid() {
        let slug = "hello-world-".to_string();
        let result = validate_slug(&slug);

        assert!(result == false);
    }

    #[test]
    fn should_return_false_when_slug_is_empty() {
        let slug = "".to_string();
        let result = validate_slug(&slug);

        assert!(result == false);
    }
}

