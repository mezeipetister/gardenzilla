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

use crate::id;
use chrono::prelude::*;
use serde::{Deserialize, Serialize};
use storaget::*;

impl TryFrom for CashRegister {
    type TryFrom = CashRegister;
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct CashRegister {
    balance: i32,
    transactions: Vec<Transaction>,
}

impl CashRegister {
    pub fn add_transaction(&mut self, transaction: Transaction) {
        self.transactions.push(transaction);
    }
    pub fn get_transactions(&self) -> &Vec<Transaction> {
        &self.transactions
    }
}

// Default implementation for
// Cashregister
impl Default for CashRegister {
    fn default() -> Self {
        CashRegister {
            balance: 0,
            transactions: Vec::new(),
        }
    }
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct Transaction {
    id: String,
    amount: i32,
    kind: TransactionKind,
    created_by: String,
    date_created: DateTime<Utc>,
}

impl Transaction {
    pub fn new(amount: i32, kind: TransactionKind, created_by: String) -> Self {
        Transaction {
            id: id::generate_alphanumeric(5),
            amount,
            kind,
            created_by,
            date_created: Utc::now(),
        }
    }
    pub fn get_id(&self) -> &str {
        &self.id
    }
    pub fn get_amount(&self) -> i32 {
        self.amount
    }
    pub fn get_kind(&self) -> &TransactionKind {
        &self.kind
    }
    pub fn get_created_by(&self) -> &str {
        &self.created_by
    }
    pub fn get_date_created(&self) -> DateTime<Utc> {
        self.date_created
    }
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub enum TransactionKind {
    MoneyIn { who: String, comment: String },
    MoneyOut { who: String, amount: i32 },
    Purchase { pruchaseId: String },
    Change { comment: String },
}
