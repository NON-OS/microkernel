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

//! The embedded controller (ACPI 6.5, chapter 12): its ports, read from the
//! ECDT or the device's _CRS; the byte-by-byte handshake for its address
//! space; and the queries it raises through its GPE.

mod crs;
mod drain;
mod ecdt;
mod health;
mod protocol;
mod queries;
mod query;
mod regs;
mod stale;

pub use crs::ec_ports;
pub use drain::{drain, Drained, MAX_QUERIES};
pub use ecdt::{parse_ecdt, Ecdt};
pub use health::{Health, Off, STRIKES};
pub use protocol::{read, write};
pub use queries::query_methods;
pub use query::{pending, query};
pub use regs::{EcError, EcPorts, BURST, IBF, OBF, QR_EC, RD_EC, SCI_EVT, WR_EC};
