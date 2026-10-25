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

//! A flight's two private keys, wiped where they lie in their boxes.

use crate::flight::ClientFlight;

fn read(p: *const u8, n: usize) -> Vec<u8> {
    // SAFETY: the flight still owns the boxes `p` points into.
    (0..n).map(|i| unsafe { core::ptr::read_volatile(p.add(i)) }).collect()
}

#[test]
fn a_flights_private_keys_are_wiped_where_they_lie() {
    let mut f = ClientFlight {
        record: Vec::new(),
        handshake: Vec::new(),
        private: Box::new([0xAA; 32]),
        p256_private: Box::new([0xBB; 32]),
    };
    let (x, p) = (f.private.as_ptr(), f.p256_private.as_ptr());
    f.wipe_private();
    assert!(read(x, 32).iter().chain(read(p, 32).iter()).all(|b| *b == 0));
}
