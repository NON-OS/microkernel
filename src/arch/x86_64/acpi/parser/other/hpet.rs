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

//! HPET read once at boot through the one pure decoder (`hpet_decode`).

#[path = "hpet_decode.rs"]
pub(super) mod hpet_decode;

use super::super::state::TableRegistry;
use super::raw_table::bytes;

pub fn parse_hpet(registry: &mut TableRegistry) {
    let Some(table) = bytes(registry, b"HPET") else { return };
    if let Some(base) = hpet_decode::hpet_base(table) {
        registry.data.hpet_address = Some(base);
    }
}
