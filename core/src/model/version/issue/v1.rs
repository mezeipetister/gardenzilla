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

impl TryFrom for Issue {
    type TryFrom = Issue;
}

#[derive(Serialize, Deserialize, Clone, Debug, Default)]
pub struct Issue {
    id: String,
}
