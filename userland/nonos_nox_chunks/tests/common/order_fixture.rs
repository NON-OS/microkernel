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

//! An order the seller's agent would deliver for, at NOW, and `check`,
//! which changes one thing and asks reveal_ok again.

use nonos_nox_chunks::{reveal_ok, Mine, OrderTerms, TermsWrong};
use nonos_nox_config::DISPUTE_WINDOW;

use crate::k::{kec, K};

pub const NOW: u64 = 1_800_000_000;
pub const KEY: [u8; 32] = [0x42; 32];
pub const CR: [u8; 32] = [0xC1; 32];
pub const MINE: Mine = Mine {
    listing_id: [1; 32],
    release_id: [2; 32],
    chunk_root: [3; 32],
    content_len: 3000,
    price: 1_000,
    payee: [4; 20],
    bond_id: [5; 32],
    nox: [6; 20],
};

pub fn terms() -> OrderTerms {
    OrderTerms {
        listing_id: [1; 32],
        release_id: [2; 32],
        bond_id: [5; 32],
        payee: [4; 20],
        token: [6; 20],
        amount: 1_000,
        delivery_deadline: NOW + 6 * 3_600,
        dispute_window: DISPUTE_WINDOW,
        chunk_root: [3; 32],
        content_len: 3000,
        cipher_root: CR,
        key_commit: kec(&[&KEY]),
        refund_commit: [7; 32],
        salt: [8; 32],
    }
}

pub fn check(f: fn(&mut OrderTerms)) -> Result<(), TermsWrong> {
    let mut t = terms();
    f(&mut t);
    reveal_ok(&K, &t, &MINE, &KEY, &CR, NOW)
}
