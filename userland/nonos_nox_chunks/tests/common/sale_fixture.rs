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

//! A sale and a buyer for the order tests, at NOW.

use nonos_nox_chunks::{Buyer, Sale};

use crate::k::kec;

pub const NOW: u64 = 1_800_000_000;
pub const NOX: [u8; 20] = [6; 20];
pub const KEY: [u8; 32] = [0x42; 32];
pub const CR: [u8; 32] = [0xC1; 32];

pub fn sale() -> Sale {
    Sale {
        listing_id: [1; 32],
        release_id: [2; 32],
        bond_id: [5; 32],
        payee: [4; 20],
        amount: 1_000,
        chunk_root: [3; 32],
        content_len: 3000,
        cipher_root: CR,
        key_commit: kec(&[&KEY]),
    }
}

pub fn buyer(deadline: u64) -> Buyer {
    Buyer { deadline, refund_to: [9; 20], refund_salt: [10; 32], salt: [11; 32] }
}
