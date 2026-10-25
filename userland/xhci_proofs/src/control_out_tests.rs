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

//! A control request's data stage: an OUT request carries exactly wLength
//! bytes after its setup and they go out as sent; an IN request carries
//! none. The stage once went out as zeros, so a SET_CUR of a sampling rate
//! set 0 Hz.

use crate::dma::DmaPool;
use crate::server::handlers::control_transfer::region::{data_region, stage_fits};

const MAX: u16 = 512;
const OUT: u8 = 0x22;
const IN: u8 = 0xA2;

#[test]
fn an_out_request_carries_exactly_its_wlength() {
    assert!(stage_fits(OUT, 3, 3, MAX));
    assert!(stage_fits(OUT, 0, 0, MAX), "no data stage");
    assert!(!stage_fits(OUT, 3, 2, MAX), "short");
    assert!(!stage_fits(OUT, 3, 4, MAX), "long");
    assert!(!stage_fits(OUT, 600, 600, MAX), "past what the handler moves");
}

#[test]
fn an_in_request_carries_nothing_and_its_wlength_is_clamped_not_refused() {
    assert!(stage_fits(IN, 3, 0, MAX));
    assert!(stage_fits(IN, 4096, 0, MAX));
    assert!(!stage_fits(IN, 3, 3, MAX));
}

#[test]
fn the_out_bytes_are_the_bytes_the_device_is_sent() {
    let pool = DmaPool::new(1, 1);
    let rate = 48_000u32.to_le_bytes();
    let r = data_region(&pool, 3, &rate[..3]).unwrap().expect("a data stage");
    // SAFETY: the region is three bytes of host memory.
    let sent = unsafe { core::slice::from_raw_parts(r.as_mut_ptr::<u8>(), 3) };
    assert_eq!(sent, &[0x80, 0xBB, 0x00], "48000 Hz little-endian");
    let r = data_region(&pool, 8, &[]).unwrap().expect("an IN stage");
    // SAFETY: as above, eight bytes.
    assert!(unsafe { core::slice::from_raw_parts(r.as_mut_ptr::<u8>(), 8) }
        .iter()
        .all(|&b| b == 0));
    assert!(data_region(&pool, 0, &[]).unwrap().is_none());
}
