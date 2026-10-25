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

//! The host wire, as driver.xhci0 serves it and every class driver speaks
//! it: the same header, ops and replies, so a class driver needs nothing
//! but this capsule's name to use a device behind it.

mod header;
mod limits;
pub mod ops;

pub use header::{decode, encode_header, Request, HDR_LEN, STATUS_LEN};
pub use limits::*;
