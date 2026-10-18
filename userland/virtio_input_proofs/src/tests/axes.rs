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

//! The axis ranges, read from a device configuration in host memory laid
//! out as struct virtio_input_config answers ABS_INFO.

use nonos_virtio::Mmio;

use crate::init::axes::read;
use crate::init::{name, Axis};

fn region(bytes: &mut [u8]) -> Mmio {
    // SAFETY: the slice outlives every use of the region in the test.
    unsafe { Mmio::new(bytes.as_mut_ptr(), bytes.len()) }
}

#[test]
fn an_answered_axis_gives_its_range_and_a_silent_one_none() {
    let mut cfg = [0u8; 136];
    cfg[2] = 20;
    cfg[8..12].copy_from_slice(&(-5i32).to_le_bytes());
    cfg[12..16].copy_from_slice(&4095i32.to_le_bytes());
    let mut common = [0u8; 0x38];
    let axes = read(&region(&mut common), region(&mut cfg)).expect("axes");
    assert_eq!(axes.x, Some(Axis { min: -5, max: 4095 }));
    assert_eq!(cfg[0], 0x12, "ABS_INFO selected");
    cfg[2] = 0;
    assert_eq!(read(&region(&mut common), region(&mut cfg)).unwrap().y, None);
    let mut short = [0u8; 16];
    assert!(read(&region(&mut common), region(&mut short)).is_err());
}

#[test]
fn a_name_stops_at_the_first_byte_that_is_not_printable() {
    let mut cfg = [0u8; 136];
    cfg[2] = 18;
    cfg[8..26].copy_from_slice(b"QEMU Virtio Tablet");
    assert_eq!(name(region(&mut cfg)).1, 18);
    cfg[13] = 0x07;
    let (n, len) = name(region(&mut cfg));
    assert_eq!(&n[..len], b"QEMU ");
}
