// #[cfg(test)]
// mod tests {
//     use super::*;
//     #[test]
//     fn test_unit_new() {
//         assert_eq!(Unit::new("mm").is_ok(), true);
//         assert_eq!(Unit::new(" mm").is_ok(), true);
//         assert_eq!(Unit::new(" mm ").is_ok(), true);
//         assert_eq!(Unit::new("mm").is_ok(), true);
//         assert_eq!(Unit::new("1kg").is_ok(), false);
//         assert_eq!(Unit::new("m m").is_ok(), false);
//         assert_eq!(Unit::new("kg").is_ok(), true);
//         assert_eq!(Unit::new("l").is_ok(), true);
//         assert_eq!(Unit::new("litre").is_ok(), true);
//         assert_eq!(Unit::new("liter").is_ok(), true);
//         assert_eq!(Unit::new("méter").is_ok(), true);
//         assert_eq!(Unit::new("m").is_ok(), true);
//         assert_eq!(Unit::new("g").is_ok(), true);
//         assert_eq!(Unit::new("db").is_ok(), true);
//     }
// }
