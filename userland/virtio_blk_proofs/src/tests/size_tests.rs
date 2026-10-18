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

//! Ring sizes. A legacy device dictates its own and every size the split
//! ring allows is laid out, in a grant sized for it: the driver held it to
//! 256, refusing QEMU's queue-size=512 and 1024, and took a size that is no
//! power of two. A modern device is told the largest power of two it
//! offers, up to the cap.

use nonos_devmodel::FakeBar;

use super::model::view;
use crate::constants::{LEG_QUEUE_NUM, LEG_STATUS, MODERN_QUEUE_SIZE, STATUS_FAILED};
use crate::init::queue_size;
use crate::queue::{avail_offset, fit, ring_bytes, used_offset};
use crate::regs::Regs;

fn offered(qmax: u16) -> (Result<u16, &'static str>, u8) {
    let bar = FakeBar::new(64);
    bar.present16(LEG_QUEUE_NUM, qmax);
    (queue_size(Regs::mmio(view(&bar))), bar.wrote8(LEG_STATUS))
}

#[test]
fn every_power_of_two_ring_is_laid_out_in_whole_pages_that_hold_it() {
    for size in (2..=15).map(|shift| 1u16 << shift) {
        let (bytes, s) = (ring_bytes(size).expect("laid out"), usize::from(size));
        assert_eq!((avail_offset(size), used_offset(size) % 4096, bytes % 4096), (16 * s, 0, 0));
        assert!(used_offset(size) >= avail_offset(size) + 6 + 2 * s, "{size}: rings overlap");
        assert!(bytes >= used_offset(size) + 6 + 8 * s, "{size}: used ring past the grant");
    }
    assert_eq!((ring_bytes(256), ring_bytes(32768)), (Some(12288), Some(860160)));
}

#[test]
fn a_legacy_ring_larger_than_256_is_taken_as_the_device_dictates() {
    for qmax in [4u16, 8, 256, 512, 1024, 32768] {
        assert_eq!(offered(qmax), (Ok(qmax), 0), "a legacy ring of {qmax} entries");
    }
}

#[test]
fn a_legacy_ring_the_split_ring_does_not_allow_is_refused_and_the_device_told() {
    for qmax in [0u16, 1, 2, 3, 100, 255, 257, 1000, u16::MAX] {
        assert_eq!(ring_bytes(qmax), None);
        let (taken, status) = offered(qmax);
        assert!(taken.is_err(), "a ring of {qmax} entries was taken");
        assert_ne!(status & STATUS_FAILED, 0, "the device was not told");
    }
}

#[test]
fn a_modern_device_is_told_the_largest_power_of_two_it_offers_up_to_the_cap() {
    let cap = MODERN_QUEUE_SIZE;
    for (offer, want) in [(0u16, None), (3, None), (4, Some(4)), (5, Some(4)), (255, Some(128))] {
        assert_eq!(fit(offer, cap), want, "offered {offer}");
    }
    for offer in [256u16, 257, 1024, u16::MAX] {
        assert_eq!(fit(offer, cap), Some(256), "offered {offer}");
    }
    assert_eq!(fit(u16::MAX, u16::MAX), Some(32768));
}
