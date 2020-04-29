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

pub use crate::model::version::cash_register::v1::*;
use storaget::*;

pub fn get_all_transaction(register: &Pack<CashRegister>) -> &Vec<Transaction> {
    register.unpack().get_transactions_all()
}

pub fn add_new_transaction(register: &mut Pack<CashRegister>, transaction: Transaction) {
    register.as_mut().unpack().add_transaction(transaction);
}

pub fn get_last_n_transactions(reg: &Pack<CashRegister>, mut n: usize) -> &[Transaction] {
    let trans = reg.unpack().get_transactions_all();
    if n > trans.len() {
        n = trans.len();
    }
    &trans[trans.len() - n..trans.len()]
}
