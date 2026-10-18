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

//! Bonds the proofs share: one with an IRK and a static random identity,
//! one with a public identity and no IRK.

use nonos_bt_bond::BondRecord;

pub const IRK: [u8; 16] = *b"peer-irk-0123456";

pub fn with_irk() -> BondRecord {
    BondRecord {
        authenticated: true,
        identity_type: 1,
        identity: [0x10, 0x20, 0x30, 0x40, 0x50, 0xc0],
        ltk: *b"long-term-key-16",
        irk: Some(IRK),
        key_size: 16,
    }
}

pub fn public() -> BondRecord {
    BondRecord {
        authenticated: false,
        identity_type: 0,
        identity: [1, 2, 3, 4, 5, 6],
        ltk: *b"another-ltk-here",
        irk: None,
        key_size: 16,
    }
}
