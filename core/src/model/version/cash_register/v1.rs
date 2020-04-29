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
    // Store balance
    balance: i32,
    transactions: Vec<Transaction>,
}

impl CashRegister {
    /// Add Transaction to CashRegister
    /// You need to know exactly the transaction type
    pub fn add_transaction(&mut self, transaction: Transaction) -> &str {
        // Update balance
        self.balance += transaction.get_amount();
        // Push transaction to store
        self.transactions.push(transaction);
        self.transactions[self.transactions.len()].get_id()
    }
    /// Get transactions as a ref vector
    pub fn get_transactions_all(&self) -> &Vec<Transaction> {
        // Return all the transactions as ref
        &self.transactions
    }
    /// Get transactions ref vector till a given date
    /// Please worth to check the date conversions:
    /// as we use DateTime<Utc> internally, but use NaiveDate in
    /// query
    pub fn get_transactions_till_date(&self, till: NaiveDate) -> Vec<&Transaction> {
        self.transactions
            .iter()
            .filter(|t| t.date_created.naive_utc().date() <= till)
            .collect::<Vec<&Transaction>>()
    }
    /// Get transactions ref vector till a given date
    pub fn get_transactions_till_date_utc(&self, till: Date<Utc>) -> Vec<&Transaction> {
        self.transactions
            .iter()
            .filter(|t| t.date_created.date() <= till)
            .collect::<Vec<&Transaction>>()
    }
    /// Get transactions ref vector till a given date
    pub fn get_transactions_till_datetime_utc(&self, till: DateTime<Utc>) -> Vec<&Transaction> {
        self.transactions
            .iter()
            .filter(|t| t.date_created <= till)
            .collect::<Vec<&Transaction>>()
    }
    /// Get transactions ref between date range
    /// Good to know that we manage dates as DateTime UTC,
    /// but in the query we use NaiveDate. So we convert
    /// DateTime<Utc> to NaiveDateTime and cenvert it to NaiveDate.
    pub fn get_transaction_between_date(
        &self,
        from: NaiveDate,
        till: NaiveDate,
    ) -> Vec<&Transaction> {
        self.transactions
            .iter()
            .filter(|t| {
                t.date_created.naive_utc().date() >= from
                    && t.date_created.naive_utc().date() <= till
            })
            .collect::<Vec<&Transaction>>()
    }
    /// Get transactions ref between DateTime<Utc> range
    pub fn get_transaction_between_datetime_utc(
        &self,
        from: DateTime<Utc>,
        till: DateTime<Utc>,
    ) -> Vec<&Transaction> {
        self.transactions
            .iter()
            .filter(|t| t.date_created >= from && t.date_created <= till)
            .collect::<Vec<&Transaction>>()
    }
    pub fn get_transaction_by_id(&self, id: &str) -> Option<&Transaction> {
        self.transactions.iter().find(|t| t.id == id)
    }
    pub fn get_balance(&self) -> i32 {
        self.balance
    }
    pub fn get_balance_till_datetime_utc(&self, till: DateTime<Utc>) -> i32 {
        self.transactions
            .iter()
            .filter(|t| t.date_created <= till)
            .map(|t| t.get_amount())
            .sum::<i32>()
    }
    pub fn get_turnover_between_datetime_utc(
        &self,
        from: DateTime<Utc>,
        till: DateTime<Utc>,
    ) -> i32 {
        self.transactions
            .iter()
            .filter(|t| t.date_created >= from && t.date_created <= till)
            .map(|t| t.get_amount())
            .sum::<i32>()
    }
    pub fn check_transaction_id_available(&self, id: &str) -> bool {
        self.transactions
            .iter()
            .find(|t| t.get_id() == id)
            .is_none()
    }
    pub fn add_new_transaction(
        &mut self,
        amount: i32,
        kind: TransactionKind,
        created_by: String,
    ) -> &str {
        let mut id = id::generate_alphanumeric(5);
        if !self.check_transaction_id_available(&id) {
            id = id::generate_alphanumeric(5);
        }
        let transaction = Transaction::new(id, amount, kind, created_by);
        self.add_transaction(transaction)
    }
    pub fn get_last_n_transactions(&self, mut n: usize) -> &[Transaction] {
        let trans = self.get_transactions_all();
        if n > trans.len() {
            n = trans.len();
        }
        &trans[trans.len() - n..trans.len()]
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
    fn new(id: String, amount: i32, kind: TransactionKind, created_by: String) -> Self {
        Transaction {
            id,
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
    MoneyIn {
        who: String,
        amount: i32,
        comment: String,
    },
    MoneyOut {
        who: String,
        amount: i32,
        comment: String,
    },
    Purchase {
        purchase_id: String,
    },
    Change {
        comment: String,
    },
}

impl TransactionKind {
    pub fn new_money_in(who: String, amount: i32, comment: String) -> TransactionKind {
        TransactionKind::MoneyIn {
            who,
            amount,
            comment,
        }
    }
    pub fn new_money_out(who: String, amount: i32, comment: String) -> TransactionKind {
        TransactionKind::MoneyOut {
            who,
            amount,
            comment,
        }
    }
    pub fn new_purchase(purchase_id: String) -> TransactionKind {
        TransactionKind::Purchase { purchase_id }
    }
    pub fn new_change(comment: String) -> TransactionKind {
        TransactionKind::Change { comment }
    }
}
