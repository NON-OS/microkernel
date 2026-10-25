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

//! A primary made: whole pages, every byte zero though the kernel's pages
//! came poisoned, so no primary ever shows what its pages held before; a
//! mapping refused is ENOMEM with nothing made; and a scrub zeroes all of it.

use nonos_gfx_serve::Platform;
use nonos_gfx_wire::errno::ENOMEM;
use nonos_libc::{maps, refuse_maps, reset};

use crate::Pool;

#[test]
fn a_primary_is_whole_pages_and_zero_over_poisoned_ones() {
    reset();
    let mut pool = Pool::default();
    for len in [1, 4095, 4096, 4097, 640 * 480 * 4, 1024 * 768 * 4 + 3] {
        let r = pool.alloc(len).expect("made");
        assert_eq!(r.len, len.next_multiple_of(4096), "{len}");
        assert!(maps().contains(&(r.base, r.len)), "{len}: its own mapping");
        assert!(pool.bytes(r).iter().all(|&b| b == 0), "{len}: a byte left as found");
    }
}

#[test]
fn a_mapping_refused_is_enomem_and_makes_nothing() {
    reset();
    let mut pool = Pool::default();
    refuse_maps(true);
    assert_eq!(pool.alloc(4096).err(), Some(ENOMEM));
    assert!(maps().is_empty());
}

#[test]
fn a_scrub_zeroes_every_byte_and_only_the_region() {
    reset();
    let mut pool = Pool::default();
    let (r, next) = (pool.alloc(8192).unwrap(), pool.alloc(4096).unwrap());
    // SAFETY: both regions are live mappings the pool made, this long.
    unsafe {
        core::ptr::write_bytes(r.base as *mut u8, 0x5C, r.len);
        core::ptr::write_bytes(next.base as *mut u8, 0x77, next.len);
    }
    pool.scrub(r);
    assert!(pool.bytes(r).iter().all(|&b| b == 0), "a composed byte kept");
    assert!(pool.bytes(next).iter().all(|&b| b == 0x77), "a scrub past its region");
}
