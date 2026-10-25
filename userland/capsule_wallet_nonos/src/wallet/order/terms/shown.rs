// NONOS Operating System
// Copyright (C) 2026 NONOS Contributors
//
// This program is free software: you can redistribute it and/or modify
// it under the terms of the GNU Affero General Public License as published by
// the Free Software Foundation, either version 3 of the License, or
// (at your option) any later version.
//
// This program is distributed in the hope that it will be useful,
// but WITHOUT ANY WARRANTY; without even the implied warranty of
// MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE. See the
// GNU Affero General Public License for more details.
//
// You should have received a copy of the GNU Affero General Public License
// along with this program. If not, see <https://www.gnu.org/licenses/>.

/*
 * When a purchase the store named may show at all (B8): only once every
 * market call is answered and the terms read, for the names asked. Until
 * then the wallet shows nothing of it, so no half-read screen can be
 * clicked through.
 */

use super::arming::Guard;
use super::asked::Record;
use crate::wallet::order::fees_kind::Fees;
use crate::wallet::order::read::Named;
use crate::wallet::order::reading_step::Reading;

pub const READING: &str = "Reading the listing's terms from the market.";

/// A purchase's terms before they are read: none, the first call due.
pub struct Unread {
    pub read: Result<Named, &'static str>,
    pub reading: Option<Reading>,
    pub record: Option<Record>,
    pub fees: Fees,
    pub guard: Guard,
}

/// Where a purchase starts, from TAKE or again once focus was lost over
/// its consent: nothing read, nothing quoted, Buy unarmed.
pub fn unread() -> Unread {
    let (read, reading, record) = (Err(READING), Some(Reading::Commerce), None);
    Unread { read, reading, record, fees: Fees::Unasked, guard: Guard::default() }
}

/// Whether the purchase view may open: no call left to make, the terms
/// read, and the market's names for them in hand.
pub fn opens(
    reading: Option<&Reading>,
    read: &Result<Named, &'static str>,
    record: Option<&Record>,
) -> bool {
    reading.is_none() && read.is_ok() && record.is_some()
}
