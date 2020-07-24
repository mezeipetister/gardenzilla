use crate::prelude::*;

pub struct TaxNumber([u32; 11]);

impl std::fmt::Display for TaxNumber {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}{}{}{}{}{}{}{}-{}-{}{}",
            self.0[0],
            self.0[1],
            self.0[2],
            self.0[3],
            self.0[4],
            self.0[5],
            self.0[6],
            self.0[7],
            self.0[8],
            self.0[9],
            self.0[10],
        )
    }
}

impl TaxNumber {
    pub fn new(tax_number: &str) -> AppResult<Self> {
        // Create an own copy of the input string
        let s = clean_characters(tax_number)
            .chars()
            .into_iter()
            .map(|c| c.to_digit(10).unwrap())
            .collect::<Vec<u32>>();
        if s.len() != 11 {
            return Err(Error::BadRequest(String::from(
                "Formai hiba! Az adószám 11 db számot kell, hogy tartalmazzon.",
            )));
        }
        let slice: [u32; 8] = [s[0], s[1], s[2], s[3], s[4], s[5], s[6], s[7]];
        let result: TaxNumber = Self([
            s[0], s[1], s[2], s[3], s[4], s[5], s[6], s[7], s[8], s[9], s[10],
        ]);
        if !is_valid_checksum(&slice) {
            return Err(Error::BadRequest(String::from(
                "A megadott adószám formailag megfelelő, de a beírt számok hibásak.",
            )));
        }
        if s[8] == 0 || s[8] > 5 {
            return Err(Error::BadRequest(String::from(
                "A megadott adószám 9. karaktere nem megfelelő 1, 2, 3, 4, vagy 5 lehet.",
            )));
        }
        Ok(result)
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
    #[test]
    fn test_taxnumber_display() {
        assert_eq!(
            format!("{}", TaxNumber::new("23127182-2-15").unwrap()),
            String::from("23127182-2-15")
        );
        assert_eq!(
            format!("{}", TaxNumber::new("231271822-15").unwrap()),
            String::from("23127182-2-15")
        );
        assert_eq!(
            format!("{}", TaxNumber::new("23127182215").unwrap()),
            String::from("23127182-2-15")
        );
        assert_eq!(
            format!("{}", TaxNumber::new("15731979-1-15").unwrap()),
            String::from("15731979-1-15")
        );
    }
}
