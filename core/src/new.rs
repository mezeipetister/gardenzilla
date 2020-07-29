// Copyright (C) 2020 Peter Mezei
//
// This file is part of Gardenzilla.
//
// Gardenzilla is free software: you can redistribute it and/or modify
// it under the terms of the GNU General Public License as published by
// the Free Software Foundation, either version 2 of the License, or
// (at your option) any later version.
//
// Gardenzilla is distributed in the hope that it will be useful,
// but WITHOUT ANY WARRANTY; without even the implied warranty of
// MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
// GNU General Public License for more details.
//
// You should have received a copy of the GNU General Public License
// along with Gardenzilla.  If not, see <http://www.gnu.org/licenses/>.

use crate::prelude::*;
use crate::taxnumber::*;
use chrono::prelude::*;
use serde::{Deserialize, Serialize};
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
#[derive(Serialize, Deserialize, Debug, Clone, Default)]
pub struct UserId(String);

impl PartialEq for UserId {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}

#[derive(Serialize, Deserialize, Debug, Clone, Default)]
pub struct CustomerId(String);

impl CustomerId {
    pub fn as_str(&self) -> &str {
        &self.0
    }
}

#[derive(Serialize, Deserialize, Debug, Clone)]
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
    pest_category: Option<PestCategory>,
    created_by: UserId,
    created_at: DateTime<Utc>,
}

pub enum PestCategory {
    I,
    II,
    III,
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
    net_retail_price: f32,
    net_retail_price_custom: Option<f32>,
    custom_price_history: Vec<UplCPH>,
    best_before: Option<DateTime<Utc>>, // todo: DateTime<Utc> or NaiveDate?
    quantity: Quantity, // todo: Should update all the times, when the SKU quantity updated
    unit: Unit,         // todo: Should be update all the times, when the SKU unit updated
    is_unopened: bool,
    location: UplLocation,
    history: Vec<UplHistoryItem>, // todo: + DateTime per event. Only location change?
    is_injured: bool,
}

pub struct UplCPH {
    created_at: DateTime<Utc>,
    created_by: UserId,
    net_retail_price_custom: f32,
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
    upls: Vec<UplPhdr>, // todo: manage it here? Or create UPLs when Procurement closed as a last step?
}

pub struct Cart {
    id: CartId,
    customer: Option<CustomerId>,
    payment_kind: PaymentKind,
    // delivery: (),
    document_kind: DocumentKind,
    items: HashMap<SKU, CartItem>,
    total_net_price: f32,
    total_vat: f32,
    total_gross_price: f32,
    created_by: UserId,
    created_at: DateTime<Utc>,
}

pub struct CartItem {
    sku: SKU,
    piece: u32,
    net_unit_price: f32,
    vat: f32,
    gross_unit_price: f32,
    upls: Vec<UplPhdr>,
    has_custom_priced: bool,
}

pub struct InvoiceData {
    customer_name: String,
    customer_address: Address,
    email: String,
    tax_number: TaxNumber,
    license_number: Option<String>,
}

pub struct Address {
    zip: u32,
    city: String,
    street: String,
}

pub enum PaymentKind {
    Cash,
    CreditCard,
    Transfer,
}

pub enum DocumentKind {
    Receipt,
    Invoice(InvoiceData),
}

pub enum DeliveryKind {
    OnSite, // todo: location?
    Delivery,
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

pub struct Invoice {
    id: String,
    customer: String,
}

pub struct Customer {
    id: CustomerId,
    name: String,
    phone: String,
    email: String,
}
