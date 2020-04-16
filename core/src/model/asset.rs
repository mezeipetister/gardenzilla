// Copyright (C) 2020 Peter Mezei
//
// This file is part of BIT.
//
// BIT is free software: you can redistribute it and/or modify
// it under the terms of the GNU General Public License as published by
// the Free Software Foundation, either version 2 of the License, or
// (at your option) any later version.
//
// BIT is distributed in the hope that it will be useful,
// but WITHOUT ANY WARRANTY; without even the implied warranty of
// MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
// GNU General Public License for more details.
//
// You should have received a copy of the GNU General Public License
// along with BIT.  If not, see <http://www.gnu.org/licenses/>.

use crate::error::Error;
use crate::model::*;
use crate::prelude::*;
use chrono::prelude::*;

impl Asset {
    pub fn new(
        id: usize,
        name: String,
        description: String,
        account: String,
        account_clearing: String,
        value: u32,
        date_activated: NaiveDate,
        depreciation_key: f32,
        residual_value: u32,
        created_by: String,
    ) -> Self {
        Asset {
            id,
            name,
            description,
            account,
            account_clearing,
            value,
            date_activated,
            depreciation_key,
            residual_value,
            date_created: Utc::now(),
            created_by,
            is_active: true,
        }
    }
    pub fn get_id(&self) -> usize {
        self.id
    }
    pub fn get_name(&self) -> &str {
        &self.name
    }
    pub fn set_name(&mut self, name: String) {
        self.name = name;
    }
    pub fn get_description(&self) -> &str {
        &self.description
    }
    pub fn set_description(&mut self, description: String) {
        self.description = description;
    }
    pub fn get_account(&self) -> &str {
        &self.account
    }
    pub fn get_account_clearing(&self) -> &str {
        &self.account_clearing
    }
    pub fn set_account(&mut self, account: String) {
        self.account = account;
    }
    pub fn set_account_clearing(&mut self, account_clearing: String) {
        self.account_clearing = account_clearing;
    }
    pub fn remove(&mut self) {
        self.is_active = false;
    }
    pub fn restore(&mut self) {
        self.is_active = true;
    }
    pub fn get_is_active(&self) -> bool {
        self.is_active
    }
    pub fn get_value(&self) -> u32 {
        self.value
    }
    pub fn get_depreciation_key(&self) -> f32 {
        self.depreciation_key
    }
    pub fn get_date_created(&self) -> DateTime<Utc> {
        self.date_created
    }
    pub fn get_created_by(&self) -> &str {
        &self.created_by
    }
    pub fn get_residual_value(&self) -> u32 {
        self.residual_value
    }
    pub fn get_date_activated(&self) -> NaiveDate {
        self.date_activated
    }
    pub fn depreciation_value(&self) -> u32 {
        self.value - self.residual_value
    }
    /// We use static 365 as a number of days in a year
    /// We do not use leap years in this model
    pub fn depreciation_daily_value(&self) -> u32 {
        (((self.value - self.residual_value) as f32 * self.depreciation_key) / 100.0 / 365.0)
            .round() as u32
    }
    /// Days in u32 that we are going to use to depreciate asset
    /// Ceil up and count the remnant on the last date.
    /// Its value must be lower then the daily depreciation value.
    pub fn depreciation_days(&self) -> u32 {
        ((self.value - self.residual_value) as f32 / self.depreciation_daily_value() as f32).ceil()
            as u32
    }
    /// Compute the last date based on the asset detail
    /// Returns a naiveDate
    pub fn depreciation_last_day(&self) -> NaiveDate {
        // Minus 1 as we count depreciation for the first day as well
        self.date_activated + chrono::Duration::days(self.depreciation_days() as i64 - 1)
    }
    /// Last day value, so the remainin from the total repreciation value.
    /// Total value - previous depreciation values
    pub fn depreciation_last_day_value(&self) -> u32 {
        self.depreciation_value() - (self.depreciation_days() - 1) * self.depreciation_daily_value()
    }
    /// Monthly depreciation value
    pub fn depreciation_by_month(&self, year: i32, month: i32) -> u32 {
        let last_day = self.depreciation_last_day();
        let date = NaiveDate::from_ymd(year, month as u32, 1);
        let date_month_day_number = month_last_day(date).day();
        // If month is withing the current item depreciation interval
        if date >= (self.date_activated - chrono::Duration::days(self.date_activated.day() as i64))
            && date <= last_day
        {
            // if the month is the last month
            // and it might be not a full one
            if date.month() == last_day.month() && date.year() == last_day.year() {
                return (last_day.day() - 1) * self.depreciation_daily_value()
                    + self.depreciation_last_day_value();
            }
            // if the month if the first month
            // and it might be not a full one
            else if date.month() == self.date_activated.month()
                && date.year() == self.date_activated.year()
            {
                return (date_month_day_number - self.date_activated.day() + 1)
                    * self.depreciation_daily_value();
            } else {
                return date_month_day_number * self.depreciation_daily_value();
            }
        }
        0
    }
    /// return Vec<Date, Depreciation value, Cumulated depreciation>
    pub fn depreciation_monthly_vector(&self) -> Vec<(NaiveDate, u32, u32)> {
        let mut res: Vec<(NaiveDate, u32, u32)> = Vec::new();
        let mut date_next: NaiveDate = month_last_day(self.date_activated);
        for _ in 0.. {
            match self.depreciation_by_month(date_next.year(), date_next.month() as i32) {
                x if x != 0 => res.push((
                    date_next,
                    x,
                    match res.last() {
                        Some(t) => t.2 + x,
                        None => x,
                    },
                )),
                _ => break,
            }
            date_next = next_month_last_day(date_next);
        }
        res
    }
    pub fn depreciation_total_till_date(&self, date: NaiveDate) -> u32 {
        self.depreciation_monthly_vector()
            .into_iter()
            .filter(|i| i.0 <= date)
            .map(|i| i.1)
            .sum()
    }
}

fn month_last_day(date: NaiveDate) -> NaiveDate {
    let given_first_day = NaiveDate::from_ymd(date.year(), date.month(), 1);
    let next_month_some = given_first_day + chrono::Duration::days(32);
    NaiveDate::from_ymd(next_month_some.year(), next_month_some.month(), 1)
        - chrono::Duration::days(1)
}

fn next_month_last_day(date: NaiveDate) -> NaiveDate {
    month_last_day(month_last_day(date) + chrono::Duration::days(1))
}

impl Repository {
    pub fn add_asset(
        &mut self,
        name: String,
        description: String,
        account: String,
        account_clearing: String,
        value: u32,
        date_activated: NaiveDate,
        depreciation_key: f32,
        residual_value: u32,
        created_by: String,
    ) -> AppResult<&Asset> {
        if !self.is_valid_account(&account) || !self.is_valid_account(&account_clearing) {
            return Err(Error::BadRequest(
                "A megadott számlaszám nem létezik vagy nem könyvelhető".to_string(),
            ));
        }
        if residual_value >= value {
            return Err(Error::BadRequest(
                "A maradványértéknek kisebbnek kell lennie, mint az eszköz bekerülési értéke."
                    .to_string(),
            ));
        }
        if depreciation_key < 0.0 || depreciation_key > 100.0 {
            return Err(Error::BadRequest(
                "A leírási kulcsnak 0-100 között kell lennie.".to_string(),
            ));
        }
        let new_asset = Asset::new(
            self.assets.len(),
            name,
            description,
            account,
            account_clearing,
            value,
            date_activated,
            depreciation_key,
            residual_value,
            created_by,
        );
        if new_asset.depreciation_daily_value() > 0 {
            self.assets.push(new_asset);
            return if let Some(v) = self.assets.last() {
                Ok(v)
            } else {
                Err(Error::InternalError(
                    "Failed to resolve the last inserted asset".to_string(),
                ))
            };
        }
        Err(Error::BadRequest(
            "Az eszköz napi ÉCS-je 0 Ft. Kisértékű eszköz.".to_string(),
        ))
    }
    pub fn remove_asset_by_id(&mut self, id: usize) -> AppResult<&Asset> {
        for asset in &mut self.assets {
            if asset.get_id() == id {
                asset.remove();
                return Ok(asset);
            }
        }
        Err(Error::BadRequest("Asset id not found".to_string()))
    }
    pub fn restore_asset_by_id(&mut self, id: usize) -> AppResult<&Asset> {
        for asset in &mut self.assets {
            if asset.get_id() == id {
                asset.restore();
                return Ok(asset);
            }
        }
        Err(Error::BadRequest("Asset id not found".to_string()))
    }
    pub fn asset_get_by_id(&mut self, id: usize) -> AppResult<&Asset> {
        for asset in &self.assets {
            if asset.get_id() == id {
                return Ok(asset);
            }
        }
        Err(Error::BadRequest("Asset id not found".to_string()))
    }
    pub fn update_asset_by_id(
        &mut self,
        id: usize,
        name: String,
        description: String,
        account: String,
        account_clearing: String,
    ) -> AppResult<&Asset> {
        for asset in &mut self.assets {
            if asset.get_id() == id {
                asset.set_name(name);
                asset.set_description(description);
                asset.set_account(account);
                asset.set_account_clearing(account_clearing);
                return Ok(asset);
            }
        }
        Err(Error::BadRequest("Asset id not found".to_string()))
    }
    pub fn get_assets(&self) -> &Vec<Asset> {
        &self.assets
    }
    pub fn get_asset_by_id(&self, id: usize) -> AppResult<&Asset> {
        for asset in &self.assets {
            if asset.get_id() == id {
                return Ok(asset);
            }
        }
        Err(Error::BadRequest(
            "A megadott eszköz ID nem létezik.".to_string(),
        ))
    }
    pub fn get_assets_by_account(&self, account: String) -> Vec<&Asset> {
        self.assets
            .iter()
            .filter(|a| a.get_account() == &account)
            .map(|a| a)
            .collect::<Vec<&Asset>>()
    }
    pub fn get_assets_by_account_clearing(&self, account_clearing: String) -> Vec<&Asset> {
        self.assets
            .iter()
            .filter(|a| a.get_account_clearing() == &account_clearing)
            .map(|a| a)
            .collect::<Vec<&Asset>>()
    }
    pub fn get_yearly_depreciation(&self, year: i32) -> u32 {
        self.assets
            .iter()
            .filter(|a| a.is_active)
            .fold(0, |sum, a| {
                sum + a
                    .depreciation_monthly_vector()
                    .iter()
                    .filter(|i| i.0.year() == year)
                    .fold(0, |sum, i| sum + i.1)
            })
    }
    pub fn get_monthly_depreciation(&self, year: i32, month: u32) -> u32 {
        self.assets
            .iter()
            .filter(|a| a.is_active)
            .fold(0, |sum, a| {
                sum + a
                    .depreciation_monthly_vector()
                    .iter()
                    .filter(|i| i.0.year() == year && i.0.month() == month)
                    .fold(0, |sum, i| sum + i.1)
            })
    }
    /// Return a vec of tuple
    /// tuple => (  clearing account: String,--------------------|
    ///             asset count: u32, ---------------------------|-----|
    ///             cummulated depreciation: u32,----------------|-----|----|
    ///             actual depreciation in this month: u32  -----|-----|----|----|
    ///          )                                               |     |    |    |
    pub fn get_statistics_by_account_clearings(&self) -> Vec<(String, u32, u32, u32)> {
        let mut res: Vec<(String, u32, u32, u32)> = Vec::new();
        for asset in &self.assets {
            if !asset.is_active {
                continue;
            }
            let vector = asset.depreciation_monthly_vector();
            let cummulated = vector
                .iter()
                .filter(|i| i.0 <= Utc::today().naive_utc())
                .fold(0, |sum, i| sum + i.1);
            let monthly_actual = vector
                .iter()
                .filter(|i| {
                    let year = i.0.year();
                    let month = i.0.month();
                    let day = i.0.day();
                    let today = Utc::today().naive_utc();
                    year == today.year() && month == today.month() && day > today.day()
                })
                .map(|i| i.1)
                .collect::<Vec<u32>>();
            if let Some(p) = res.iter().position(|r| r.0 == asset.account_clearing) {
                if let Some(i) = res.get_mut(p) {
                    i.1 += 1;
                    i.2 += cummulated;
                    i.3 += monthly_actual.get(0).unwrap_or(&0);
                }
            } else {
                res.push((
                    asset.account_clearing.clone(),
                    1,
                    cummulated,
                    *monthly_actual.get(0).unwrap_or(&0),
                ));
            }
        }
        res.sort_by(|a, b| a.0.cmp(&b.0));
        res
    }
}

#[cfg(test)]
mod tests {
    use crate::model::*;
    use chrono::prelude::*;
    #[test]
    fn test_monthly_vector() {
        let asset = Asset::new(
            0,
            "demo1".to_string(),
            "asd".to_string(),
            "11".to_string(),
            "119".to_string(),
            1000000,
            NaiveDate::from_ymd(2020, 03, 15),
            2.0,
            15,
            "mezeipetister".to_string(),
        );
        assert_eq!(asset.get_account(), "11");
        assert_eq!(asset.depreciation_monthly_vector().len() > 0, true);
    }
}
