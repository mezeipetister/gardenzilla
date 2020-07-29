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
use crate::new::*;
use crate::taxnumber::*;
use chrono::prelude::*;
use serde::{Deserialize, Serialize};
use storaget::*;

impl TryFrom for Customer {
    type TryFrom = Customer;
}

// Implement StorageObject for NotificationContainer
impl VecPackMember for Customer {
    fn get_id(&self) -> &str {
        &self.id.as_str()
    }
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Customer {
    /// ID for customer
    id: CustomerId,
    /// Vector of usernames
    related_users: Vec<UserId>,
    name: String,
    tax_number: TaxNumber,
    address: InvoiceAddress,
    phone: String, // todo: Phone?
    email: String, // todo: Email
    created_at: DateTime<Utc>,
    created_by: UserId,
}

impl Default for Customer {
    fn default() -> Self {
        Customer {
            id: CustomerId::default(),
            related_users: Vec::new(),
            name: String::new(),
            tax_number: TaxNumber::default(),
            address: InvoiceAddress::default(),
            phone: String::new(),
            email: String::new(),
            created_at: Utc::now(),
            created_by: UserId::default(),
        }
    }
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct InvoiceAddress {
    pub zip: u32,
    pub location: String,
    pub street: String,
}

impl Default for InvoiceAddress {
    fn default() -> Self {
        InvoiceAddress {
            zip: 0,
            location: String::new(),
            street: String::new(),
        }
    }
}

impl InvoiceAddress {
    pub fn new(zip: u32, location: String, street: String) -> Self {
        InvoiceAddress {
            zip,
            location,
            street,
        }
    }
}

impl Customer {
    pub fn new(
        id: CustomerId,
        name: String,
        email: String,
        phone: String,
        tax_number: TaxNumber,
        zip: u32,
        location: String,
        street: String,
        created_by: UserId,
    ) -> Self {
        Customer {
            id,
            related_users: Vec::new(),
            name,
            tax_number,
            address: InvoiceAddress::new(zip, location, street),
            email,
            phone,
            created_at: Utc::now(),
            created_by,
        }
    }
    pub fn get_id(&self) -> &str {
        &self.id.as_str()
    }
    pub fn get_name(&self) -> &str {
        &self.name
    }
    pub fn set_name(&mut self, name: String) {
        self.name = name;
    }
    pub fn has_user(&self) -> bool {
        self.related_users.len() > 0
    }
    pub fn get_users(&self) -> &Vec<UserId> {
        &self.related_users
    }
    pub fn remove_user(&mut self, username: &UserId) {
        self.related_users.retain(|u| u != username);
    }
    pub fn get_tax_number(&self) -> &TaxNumber {
        &self.tax_number
    }
    pub fn set_tax_number(&mut self, tax_number: TaxNumber) {
        self.tax_number = tax_number;
    }
    pub fn set_address(&mut self, zip: u32, location: String, street: String) {
        self.address.zip = zip;
        self.address.location = location;
        self.address.street = street;
    }
    pub fn get_address(&self) -> (u32, String, String) {
        (
            self.address.zip.clone(),
            self.address.location.clone(),
            self.address.street.clone(),
        )
    }
    pub fn get_phone(&self) -> String {
        self.phone.clone()
    }
    pub fn set_phone(&mut self, phone: String) {
        self.phone = phone;
    }
    pub fn get_email(&self) -> String {
        self.email.clone()
    }
    pub fn set_email(&mut self, email: String) {
        self.email = email;
    }
    pub fn get_date_created(&self) -> DateTime<Utc> {
        self.created_at
    }
    pub fn get_created_by(&self) -> &UserId {
        &self.created_by
    }
}

pub struct Email {
    user: String,
    provider: String,
}

fn clean_spaces(s: &str) -> String {
    let mut result = s.to_string();
    // Remove everything but numbers
    result.retain(|c| !c.is_whitespace());
    result
}

mod tests {
    use super::*;
    #[test]
    fn test_clean_spaces() {
        let d = "hello bello";
        assert_eq!(clean_spaces(d), String::from("hellobello"));
        assert_eq!(clean_spaces(d) == String::from("hello bello"), false);
    }
}
