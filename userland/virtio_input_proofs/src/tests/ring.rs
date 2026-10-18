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

//! The event queue over host memory, with the test as the device: every
//! descriptor aimed at its own buffer and offered, each used element read
//! once, and the indices wrapping past 65535.

use super::ring_page::Page;
use crate::constants::queue::AVAIL_OFF;
use crate::queue::used::Used;
use crate::queue::EventRing;

const DEV: u64 = 0x4_0000_0000;

#[test]
fn priming_offers_every_buffer_and_a_used_one_is_read_once() {
    let mut page = Page::new();
    let mut ring = EventRing::new(page.va(), DEV, 64);
    ring.prime();
    assert_eq!((page.u16(AVAIL_OFF), page.u16(AVAIL_OFF + 2)), (1, 64), "no interrupt, 64 offered");
    assert_eq!(page.u16(AVAIL_OFF + 4 + 63 * 2), 63);
    assert_eq!(ring.take(), None);
    page.give(0, 64, 5, [3, 0, 0, 0, 7, 0, 0, 0]);
    assert_eq!(ring.take(), Some(Used { id: 5, len: 8 }));
    assert_eq!(ring.event(5), [3, 0, 0, 0, 7, 0, 0, 0]);
    assert_eq!(ring.take(), None, "read once");
    ring.offer(5);
    assert_eq!((page.u16(AVAIL_OFF + 2), page.u16(AVAIL_OFF + 4)), (65, 5));
}

#[test]
fn the_indices_wrap_past_65535() {
    let mut page = Page::new();
    let mut ring = EventRing::new(page.va(), DEV, 16);
    ring.prime();
    (ring.last_used, ring.avail_idx) = (65534, 65534);
    for n in 65534..=65537u32 {
        let n = n as u16;
        page.give(n, 16, (n % 16) as u32, [0; 8]);
        assert_eq!(ring.take().map(|u| u.id), Some((n % 16) as u32));
        ring.offer(n % 16);
    }
    assert_eq!((ring.last_used, ring.avail_idx), (2, 2));
}
