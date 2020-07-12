use chrono::prelude::*;
use std::collections::HashMap;

type SKU = String;

enum UplLocation {
    Cart(u32),
    Stock(u32),
    Delivery(u32),
}

pub struct Upl {
    id: u32,
    sku: SKU,
    procurement_id: u32,        // todo: maybe ProcurementId?
    net_procurement_price: f32, // todo: sure?
    best_before: NaiveDate,     // todo: DateTime<Utc>?
    is_divisible: bool,         // todo: Manage this here?
    quantity: f32,              // unit is the same as the SKU's
    location: UplLocation,      //
    history: Vec<UplLocation>,  // todo: + DateTime per event. Only location change?
}

pub struct UplNeedle {
    upl_id: u32,
    sku: SKU,
}

pub struct UplStore {
    upls: HashMap<u32, Upl>, // todo: maybe UplId instead of u32?
}

pub trait UplManager {
    fn move_in(&mut self, upl_needle: UplNeedle);
    fn move_out(&mut self) -> UplNeedle {
        unimplemented!()
    }
}

impl UplStore {
    pub fn find_upl(&self, upl_id: u32) -> Result<&Upl, String> {
        unimplemented!("Not implemented!")
    }
    pub fn move_upl<F, T>(&mut self, from: F, to: T) -> Result<(), String>
    where
        F: UplManager,
        T: UplManager,
    {
        unimplemented!()
    }
}

pub struct Product {
    sku: SKU,
    name: String,
}
