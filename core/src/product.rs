use crate::new::*;
use crate::unit::*;
use chrono::prelude::*;

/*
    todo: unit conversation?
*/

// todo: manufacturer, brand
// eg.:
// Product {
//     sku: 1,
//     name: "Demo",
//     quantity: 1.0,
//     unit: "db",
//     created_by: "demouser",
//     created_at: "2020-07-13 10:06:01T"
// }
// Product {
//     sku: 2,
//     name: "Actara 50 ml",
//     quantity: 50.0,
//     unit: "ml",
//     created_by: "demouser",
//     created_at: "2020-07-13 10:06:01T"
// }
// Product {
//     sku: 3,
//     name: "Billingo ajtó kilincs",
//     quantity: 1.0,
//     unit: "db",
//     created_by: "demouser",
//     created_at: "2020-07-13 10:06:01T"
// }
pub struct Product {
    sku: SKU,
    name: String,
    quantity: f32,
    unit: Unit,
    created_by: User,
    created_at: DateTime<Utc>,
}
