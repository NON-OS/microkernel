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

//! Where a purchase's funding quote stands, and an amount written exactly.

use alloc::format;
use alloc::string::String;

pub enum Fees {
    Unasked,
    Asking,
    /// The service's total fee, as it wrote it.
    Quoted(String),
    Refused(String),
}

/// `wei` as NOX: the whole part, then every significant decimal.
pub fn exact(wei: u128) -> String {
    let unit = 1_000_000_000_000_000_000u128;
    let frac = format!("{:018}", wei % unit);
    match frac.trim_end_matches('0') {
        "" => format!("{}", wei / unit),
        f => format!("{}.{f}", wei / unit),
    }
}
