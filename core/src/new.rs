use chrono::prelude::*;
use std::collections::HashMap;

/*
   Backlog
       - Product
       - Unit
       - User
       - Customer
       - UPL Store
       - UPL Needle mechanism + traits
       - UPL movement traits + methods
       - Stock
       - Cart
       - Purchase process
       - Invoice
       - PettyCash
       - Source
       - Procurement
       - Delivery
       - ..
       - Order
       - Reservation
*/

pub type SKU = String;

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
    best_before: DateTime<Utc>, // todo: DateTime<Utc> or NaiveDate?
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
    fn move_in(&mut self, upl_needle: UplNeedle) -> &Upl {
        unimplemented!()
    }
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

pub struct User();
pub struct Customer();

pub struct Cart {
    id: u32,
    customer: Customer,
    items: HashMap<SKU, CartItem>,
    total_net_price: f32,
    total_vat: f32,
    total_gross_price: f32,
    created_by: User,
    created_at: DateTime<Utc>,
    is_closed: bool,
}

pub struct CartItem {
    sku: SKU,
    piece: f32,
    net_unit_price: f32,
    vat: f32,
    gross_unit_price: f32,
    upls: Vec<Upl>,
}
