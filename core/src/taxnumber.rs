use crate::prelude::*;

pub struct TaxNumber([u32; 11]);

impl TaxNumber {
    pub fn new(tax_number: &str) -> AppResult<Self> {
        // Create an own copy of the input string
        let s = clean_characters(tax_number);
        if s.len() != 11 {
            return Err(Error::BadRequest(String::from(
                "Formai hiba! Az adószám 11 db számot kell, hogy tartalmazzon.",
            )));
        }
        // Check character numbers
        unimplemented!()
    }
}

// Hungarian tax number validation
// based on the algorythm found here:
// https://hu.wikipedia.org/wiki/Ad%C3%B3sz%C3%A1m
fn is_valid_checksum(s: &[u32; 8]) -> bool {
    let sum = s[0] * 9 + s[1] * 7 + s[2] * 3 + s[3] * 1 + s[4] * 9 + s[5] * 7 + s[6] * 3;
    if let Some(last) = sum.to_string().chars().into_iter().last() {
        if last == '0' {
            if s[7] != 0 {
                return false;
            }
        } else {
            if let Some(l) = last.to_digit(10) {
                if s[7] != (10 - l) {
                    return false;
                }
            } else {
                return false;
            }
        }
        return true;
    }
    false
}

fn clean_characters(s: &str) -> String {
    let mut result = s.to_string();
    // Remain only numbers
    result.retain(|c| c.is_numeric());
    result
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn taxnumber_test() {
        unimplemented!();
    }

    #[test]
    fn test_clean_format() {
        assert_eq!(clean_characters("123 "), String::from("123"));
        assert_eq!(clean_characters("1 2 3 "), String::from("123"));
        assert_eq!(clean_characters("123-"), String::from("123"));
        assert_eq!(clean_characters("1a2b3c - "), String::from("123"));
        assert_eq!(
            clean_characters("23127182-2-15 "),
            String::from("23127182215")
        );
    }

    #[test]
    fn test_checksum() {
        assert_eq!(is_valid_checksum(&[2, 3, 1, 2, 7, 1, 8, 2]), true); // Valid example
        assert_eq!(is_valid_checksum(&[2, 3, 1, 2, 7, 1, 8, 3]), false); // Wrong
        assert_eq!(is_valid_checksum(&[2, 3, 1, 2, 7, 1, 9, 2]), false); // Wrong
        assert_eq!(is_valid_checksum(&[2, 5, 5, 7, 2, 2, 0, 3]), true); // Valid example
        assert_eq!(is_valid_checksum(&[1, 5, 7, 3, 1, 9, 7, 9]), true); // Valid example
    }
}
