use crate::prelude::*;
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
pub type CartId = u32;
pub type StockId = u32;
pub type DeliveryId = u32;
pub type UplId = u32;
pub type ProcurementId = u32;

pub enum Unit {
    Piece,
    Millimeter,
    Gram,
    Milliliter,
}

impl Unit {
    pub fn from_str(from: &str) -> AppResult<Unit> {
        let res = match from {
            "piece" => Unit::Piece,
            "millimeter" => Unit::Milliliter,
            "gram" => Unit::Gram,
            "milliliter" => Unit::Milliliter,
            _ => return Err(Error::BadRequest(format!("Wrong unit format: {}", from))),
        };
        Ok(res)
    }
}

pub struct Product {
    sku: SKU,
    name: String,
    quantity: u32, // 250
    unit: Unit,    // ml
    created_by: User,
    created_at: DateTime<Utc>,
}

enum UplLocation {
    Cart(CartId),
    Stock(StockId),
    Delivery(DeliveryId),
}

pub struct Upl {
    id: UplId,
    sku: SKU,
    procurement_id: ProcurementId,      // todo: maybe ProcurementId?
    net_procurement_price: f32,         // todo: sure?
    best_before: Option<DateTime<Utc>>, // todo: DateTime<Utc> or NaiveDate?
    quantity: u32,                      // unit is the same as the SKU's
    is_opened: bool,
    location: UplLocation,        //
    history: Vec<UplHistoryItem>, // todo: + DateTime per event. Only location change?
}

pub struct UplHistoryItem {
    created_at: DateTime<Utc>,
    created_by: User,
    location: UplLocation,
}

pub struct UplNeedle {
    upl_id: u32,
    sku: SKU,
}

pub struct UplStore {
    upls: HashMap<UplId, Upl>, // todo: somehow partitioning?
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
    pub fn find_upl(&self, upl_id: u32) -> AppResult<&Upl> {
        unimplemented!("Not implemented!")
    }
    pub fn move_upl<F, T>(&mut self, from: &mut F, to: &mut T) -> AppResult<()>
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
    upls: Vec<UplNeedle>,
}
