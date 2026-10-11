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

//! What several tests need: the address the shim's entropy draws, a driver
//! brought up on a fresh bench, and a frame's bytes in a pool slot.

use crate::bench::{Bench, GRANTS};
use crate::constants::queue::BUFFER_LEN;
use crate::init::finish;
use crate::model::{take, Event, Part};
use crate::setup::Driver;

/// The station address the shim's pattern becomes once made local unicast.
pub fn drawn() -> [u8; 6] {
    let mut mac: [u8; 6] = core::array::from_fn(|i| 0xC3 ^ i as u8);
    nonos_mac::apply(&mut mac);
    mac
}

/// A frame to `dest` with `fill` after the address.
pub fn frame_to(dest: [u8; 6], len: usize, fill: u8) -> Vec<u8> {
    [&dest[..], &vec![fill; len - 6]].concat()
}

pub fn up(bench: &mut Bench) -> Driver {
    finish(bench.with(|_| {})).expect("bring-up on a well-behaved part")
}

pub fn slot(pool_va: u64, idx: u16) -> &'static mut [u8] {
    // SAFETY: the bench's pools outlive every test that asks, and a slot is
    // BUFFER_LEN bytes inside one.
    unsafe {
        std::slice::from_raw_parts_mut(
            (pool_va + idx as u64 * BUFFER_LEN as u64) as *mut u8,
            BUFFER_LEN,
        )
    }
}

/// Bring-up on a part tuned to fail, holding the driver to its cleanup: the
/// part stopped and reset before any grant goes back, then every grant and
/// the claim, last taken first.
pub fn refused(tune: impl FnOnce(&mut Part)) -> (&'static str, Vec<Event>) {
    let mut bench = Bench::new();
    let err = finish(bench.with(tune)).err().expect("bring-up must fail");
    let back = nonos_libc::given_back();
    let want = vec![
        ("dma_unmap", GRANTS[4]),
        ("dma_unmap", GRANTS[3]),
        ("dma_unmap", GRANTS[2]),
        ("mmio_unmap", GRANTS[1]),
        ("device_release", GRANTS[0]),
    ];
    assert_eq!(back, want, "every grant back, last taken first");
    let log = take().log;
    let n = log.len();
    assert_eq!(log[n - 3..n - 1], [Event::Csr(6, 0), Event::Csr(0, 1)]);
    assert_eq!(log[n - 1], Event::Reset { grants_back: 0 });
    (err, log)
}
