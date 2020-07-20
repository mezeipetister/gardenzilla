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
pub type SourceId = u32;
pub type InvoiceId = String;

pub enum Unit {
    Piece,
    Millimeter,
    Gram,
    Milliliter,
}

pub enum Quantity {
    Simple(u32),
    Complex(u32, u32),
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
    quantity: Quantity, // 250 5x3 ml -> 15 ml => price / ml
    unit: Unit,         // ml
    created_by: UserId,
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
    quantity: Quantity,                 // unit is the same as the SKU's
    is_opened: bool,
    location: UplLocation,        //
    history: Vec<UplHistoryItem>, // todo: + DateTime per event. Only location change?
}

pub struct UplHistoryItem {
    created_at: DateTime<Utc>,
    created_by: UserId,
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

pub struct UserId();
pub struct CustomerId();

pub struct Cart {
    id: u32,
    customer: CustomerId,
    items: HashMap<SKU, CartItem>,
    total_net_price: f32,
    total_vat: f32,
    total_gross_price: f32,
    created_by: UserId,
    created_at: DateTime<Utc>,
    is_closed: bool,
}

pub struct CartItem {
    sku: SKU,
    piece: u32,
    net_unit_price: f32,
    vat: f32,
    gross_unit_price: f32,
    upls: Vec<UplNeedle>,
}

pub struct Purchase {
    id: u32,
    items: HashMap<SKU, CartItem>,
    customer: CustomerId,
    payment: (),
    is_payed: bool,
    document: DocumentKind,
    document_history: Vec<(DateTime<Utc>, DocumentKind)>,
    is_removed: bool,
}

pub enum DocumentKind {
    Receipt,
    Normal(InvoiceId),
    Storno(InvoiceId),
}

pub struct Invoice {
    id: String,
    customer: String,
}

pub struct Stock {
    id: StockId,
    name: String,
    items: HashMap<SKU, Vec<UplNeedle>>,
}

pub struct Source {
    id: SourceId,
    name: String,
    items: HashMap<SKU, Vec<SourceItemHistory>>,
}

pub struct Procurement {
    id: ProcurementId,
    description: String,
    source: SourceId,
    items: Vec<ProcurementItem>,
    expected_net_total: f32,
    created_by: UserId,
    created_at: DateTime<Utc>,
}

pub struct ProcurementItem {
    sku: SKU,
    quantity: u32,
    expected_net_unit_price: f32,
    description: String,
}
