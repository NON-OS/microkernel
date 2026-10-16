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

//! nonos_blk_client's wire, proved on the host against the block drivers'
//! own sources. Each `#[path]` pulls in the shipping file, so a driver that
//! renumbers an op, or changes its magic or header, fails these tests
//! rather than having the client send a write where a capacity query was
//! meant. scripts/check_blk_wire.py holds the same table to every driver
//! the client names, by service name, without a build.

extern crate alloc;

#[path = "../../nonos_blk_client/src/driver/mod.rs"]
pub mod driver;
pub mod drivers;
#[path = "../../nonos_blk_client/src/error.rs"]
pub mod error;
#[path = "../../nonos_blk_client/src/device/identity_parse.rs"]
pub mod identity_parse;
#[path = "../../nonos_blk_client/src/device/span.rs"]
pub mod span;
#[path = "../../nonos_blk_client/src/status_text.rs"]
pub mod status_text;
pub mod wire;

#[cfg(test)]
mod tests;
