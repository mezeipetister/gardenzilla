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
pub struct UserId();
pub struct CustomerId();

pub enum Unit {
    Piece,
    Millimeter,
    Gram,
    Milliliter,
}

impl std::fmt::Display for Unit {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match &self {
            Unit::Piece => write!(f, "db"),
            Unit::Milliliter => write!(f, "ml"),
            Unit::Gram => write!(f, "g"),
            Unit::Millimeter => write!(f, "mm"),
        }
    }
}

pub enum Quantity {
    Simple(u32),
    Complex(u32, u32),
}

impl std::fmt::Display for Quantity {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match &self {
            Quantity::Simple(quantity) => write!(f, "{}", quantity),
            Quantity::Complex(multiplier, quantity) => write!(f, "{}x{}", multiplier, quantity),
        }
    }
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
    quantity: Quantity, // e.g.: Simple(u32) => 3 ml, or Complex(u32, u32) => 5x3 ml
    unit: Unit,         // e.g.: ml
    net_retail_price: Option<f32>, // e.g.: 1000 HUF
    vat_percentage: Option<u32>, // e.g.: 27 => 27%, 0 | 5 | 18 | 27 based on the Hungarian tax law 2020
    created_by: UserId,
    created_at: DateTime<Utc>,
}

impl std::fmt::Display for Product {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match &self.unit {
            _ => write!(f, "{} {} {}", self.name, self.quantity, self.unit),
        }
    }
}

enum UplLocation {
    Stock(StockId),
    //Delivery(DeliveryId),
    Cart(CartId),
    Purchase(CartId),
}

pub struct Upl {
    id: UplId,
    sku: SKU,
    procurement_id: ProcurementId, // todo: maybe ProcurementId?
    net_procurement_price: f32,    // todo: sure?
    net_retail_price_list: Option<f32>,
    net_retail_price: Option<f32>,      // todo: price change history?
    best_before: Option<DateTime<Utc>>, // todo: DateTime<Utc> or NaiveDate?
    quantity: Quantity, // todo: Should update all the times, when the SKU quantity updated
    unit: Unit,         // todo: Should be update all the times, when the SKU unit updated
    is_unopened: bool,
    location: UplLocation,
    history: Vec<UplHistoryItem>, // todo: + DateTime per event. Only location change?
}

pub struct UplHistoryItem {
    created_at: DateTime<Utc>,
    created_by: UserId,
    location: UplLocation,
}

pub struct UplPhdr {
    upl_id: u32,
    sku: SKU,
}

pub struct UplStore {
    upls: HashMap<UplId, Upl>,
}

pub trait UplManager {
    fn move_in(&mut self, upl_needle: UplPhdr) -> &Upl {
        unimplemented!()
    }
    fn move_out(&mut self) -> UplPhdr {
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

pub struct Stock {
    id: StockId,
    name: String,
    items: HashMap<SKU, Vec<UplPhdr>>,
}

pub struct Source {
    id: SourceId,
    name: String,
    items: HashMap<SKU, Vec<SourceItem>>,
}

pub struct SourceItem {
    net_wholesale_price: f32,
    kind: SourceKind,
    comment: Option<String>,
    created_by: UserId,
    created_at: DateTime<Utc>,
}

pub enum SourceKind {
    PriceList,
    Call,
    Procurement(ProcurementId),
}

pub struct PriceLog {
    sku: SKU,
    price_history: Vec<PriceLogHistoryItem>,
}

pub struct PriceLogHistoryItem {
    net_retail_price: f32,
    ppp_expected: (), // Profit Per Product
    ppp_pct_expected: (),
    reason: PriceLogReason,
    created_by: UserId,
    created_at: DateTime<Utc>,
}

pub enum PriceLogReason {
    Procurement(ProcurementId),
    Custom,
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
    upls: Vec<UplPhdr>,
}

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
    upls: Vec<UplPhdr>,
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
