pub enum _Unit {
    // Single
    Single,
    // Length
    Milimeter,
    Centimeter,
    Meter,
    Kilometer,
    // Weight
    Milligram,
    Gram,
    Dekagram,
    Kilogram,
    // Liquid
    Milliliter,
    Deciliter,
    Liter,
    Hectoliter,
}

pub struct Unit(String);

impl Unit {
    pub fn new(name: &str) -> Result<Self, String> {
        let name = name.trim().to_string();
        if name.chars().all(|c: char| c.is_alphabetic()) {
            return Ok(Unit(name));
        }
        Err(format!("Unit name must contains only alphanumeric"))
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_unit_new() {
        assert_eq!(Unit::new("mm").is_ok(), true);
        assert_eq!(Unit::new(" mm").is_ok(), true);
        assert_eq!(Unit::new(" mm ").is_ok(), true);
        assert_eq!(Unit::new("mm").is_ok(), true);
        assert_eq!(Unit::new("1kg").is_ok(), false);
        assert_eq!(Unit::new("m m").is_ok(), false);
        assert_eq!(Unit::new("kg").is_ok(), true);
        assert_eq!(Unit::new("l").is_ok(), true);
        assert_eq!(Unit::new("litre").is_ok(), true);
        assert_eq!(Unit::new("liter").is_ok(), true);
        assert_eq!(Unit::new("méter").is_ok(), true);
        assert_eq!(Unit::new("m").is_ok(), true);
        assert_eq!(Unit::new("g").is_ok(), true);
        assert_eq!(Unit::new("db").is_ok(), true);
    }
}
