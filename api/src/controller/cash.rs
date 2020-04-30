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

use crate::guard::Login;
use crate::prelude::*;
use crate::DataLoad;
use chrono::prelude::*;
use core_lib::model::*;
use rocket::http::RawStr;
use rocket::request::FromParam;
use rocket::State;

#[get("/cash_register/all")]
pub fn cash_register_all_get(
    _user: Login,
    data: State<DataLoad>,
) -> Result<StatusOk<Vec<Transaction>>, ApiError> {
    let res = data
        .inner()
        .cash_register
        .lock()?
        .unpack()
        .get_transactions_all()
        .clone();
    Ok(StatusOk(res))
}

#[get("/cash_register/last/<n>")]
pub fn cash_register_last_n_get(
    _user: Login,
    data: State<DataLoad>,
    n: usize,
) -> Result<StatusOk<Vec<Transaction>>, ApiError> {
    let res = data
        .inner()
        .cash_register
        .lock()?
        .unpack()
        .get_last_n_transactions(n)
        .to_vec();
    Ok(StatusOk(res))
}

pub trait ToDateTimeUtc {
    fn to_datetime_utc(&self) -> Result<DateTime<Utc>, ApiError>;
}

impl ToDateTimeUtc for &str {
    #[inline(always)]
    fn to_datetime_utc(&self) -> Result<DateTime<Utc>, ApiError> {
        match DateTime::parse_from_rfc3339(&self) {
            Ok(dtfixed) => Ok(dtfixed.with_timezone(&Utc)),
            Err(_) => Err(ApiError::BadRequest(format!(
                "Wrong date format: {}. Please use proper rfc3339.",
                self
            ))),
        }
    }
}

#[get("/cash_register/daterange/<from>/<till>")]
pub fn cash_register_daterange_get(
    _user: Login,
    data: State<DataLoad>,
    from: String,
    till: String,
) -> Result<StatusOk<Vec<Transaction>>, ApiError> {
    let fromutc = from.as_str().to_datetime_utc()?;
    let tillutc: DateTime<Utc> = till.as_str().to_datetime_utc()?;
    let res = data
        .inner()
        .cash_register
        .lock()?
        .unpack()
        .get_transaction_between_datetime_utc(fromutc, tillutc)
        .iter()
        .map(|t| (**t).clone())
        .collect::<Vec<Transaction>>();
    Ok(StatusOk(res))
}

#[get("/cash_register/new/<amount>")]
pub fn cash_register_new_put(
    _user: Login,
    data: State<DataLoad>,
    amount: i32,
) -> Result<StatusOk<String>, ApiError> {
    let res = data
        .inner()
        .cash_register
        .lock()?
        .as_mut()
        .unpack()
        .add_new_transaction(
            amount,
            TransactionKind::new_purchase("0".to_string()),
            _user.userid().to_string(),
        )
        .to_string();
    Ok(StatusOk(res))
}
