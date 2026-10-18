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

//! The identities, records and little clock the model sweep shares.

use crate::faulty::Faulty;
use crate::fixture::public;
use nonos_bt_bond::{BondRecord, Store};
use nonos_bt_wire::addr::{AddrType, BdAddr};

/// Three distinct public identities.
pub(super) fn ident(i: usize) -> [u8; 6] {
    [i as u8 + 1, 2, 3, 4, 5, 6]
}

/// A record for identity `i` at the given level (public type, no IRK).
pub(super) fn record(i: usize, authenticated: bool) -> BondRecord {
    BondRecord { authenticated, identity: ident(i), ..public() }
}

/// xorshift64, the same each run.
pub(super) struct Rng(pub u64);

impl Rng {
    pub(super) fn next(&mut self) -> u64 {
        let mut x = self.0;
        x ^= x << 13;
        x ^= x >> 7;
        x ^= x << 17;
        self.0 = x;
        x
    }
    pub(super) fn below(&mut self, n: usize) -> usize {
        (self.next() % n as u64) as usize
    }
}

/// The level the store resolves for identity `i`, checking it is not another
/// identity's record (two identities keyed alike would show here).
pub(super) fn level(s: &Store<Faulty>, i: usize) -> Option<bool> {
    s.find(AddrType::Public, &BdAddr(ident(i))).map(|r| {
        assert_eq!(r.identity, ident(i), "identity {i} resolved to another's record");
        r.authenticated
    })
}

pub(super) const SEEDS: [u64; 4] = [0x9e37_79b9_7f4a_7c15, 0x2545_f491_4f6c_dd1d, 7, 0xdead_beef];
