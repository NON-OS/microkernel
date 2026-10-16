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

//! ADMA2 descriptor tables (SDHCI 3.0 ADMA2, as recalled, unverified). Pure: the table is built in
//! a byte slice the caller then copies into the descriptor page.
//!
//! Every descriptor is a transfer descriptor (Act = 10b, Valid). The last
//! one carries End. No Nop descriptor is ever written, so a host that
//! mishandles End in a Nop descriptor (Intel eMMC hosts, as observed,
//! unverified against Intel's documents) never meets one, and every host
//! gets the plain form.

mod build;
mod error;
mod layout;
mod width_for;

pub use build::build;
pub use error::AdmaError;
pub use layout::{Width, ACT_TRAN, ATTR_END, ATTR_VALID, MAX_CHUNK, TABLE_BYTES};
pub use width_for::width_for;
