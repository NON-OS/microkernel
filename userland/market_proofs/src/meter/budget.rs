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
//! The owner's budget, whose spending() PAY and FUNDED both check.

#[path = "../../../nonos_nox_meter/src/budget/add.rs"]
mod add;
#[path = "../../../nonos_nox_meter/src/budget/caps.rs"]
mod caps;
#[path = "../../../nonos_nox_meter/src/budget/listing.rs"]
mod listing;

pub use caps::Budget;
pub use listing::{ListingCap, MAX_LISTING_CAPS};
