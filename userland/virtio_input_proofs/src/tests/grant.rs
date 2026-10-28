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

//! The event queue's grant: one shorter than the ring is refused and given
//! back, as the ring and its buffers would run past it.

use nonos_libc::{grant_length, unmapped, GRANT};

use crate::constants::queue::RING_BYTES;
use crate::dma::map;

#[test]
fn a_whole_grant_is_kept() {
    grant_length(None);
    let out = map(1, 1).expect("the grant");
    assert_eq!(out.length, RING_BYTES as u64);
    assert!(unmapped().is_empty());
}

#[test]
fn a_short_grant_is_refused_and_given_back() {
    grant_length(Some(RING_BYTES as u64 - 1));
    assert_eq!(map(1, 1).err(), Some("dma grant short (event queue)"));
    assert_eq!(unmapped(), [GRANT]);
    grant_length(Some(RING_BYTES as u64 * 2));
    assert!(map(1, 1).is_ok(), "a longer one serves");
    assert!(unmapped().is_empty());
}
