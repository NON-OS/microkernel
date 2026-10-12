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

//! Copying the EDID blob out of BAR2: only a blob with the E-EDID header
//! (section 3.3), only as many blocks as it counts, never past the window.

use crate::constants::EDID_WINDOW;
use crate::edid::read;
use crate::model::{blob, Adapter};

fn read_all(a: &Adapter) -> ([u8; EDID_WINDOW], usize) {
    let mut out = [0u8; EDID_WINDOW];
    // SAFETY: the window is BAR2, longer than EDID_WINDOW.
    let n = unsafe { read(a.bar.base(), &mut out) };
    (out, n)
}

#[test]
fn a_window_without_the_header_holds_no_blob() {
    let a = Adapter::new();
    a.bar.present8(0, 0xAA);
    assert_eq!(read_all(&a).1, 0);
}

#[test]
fn a_base_block_alone_is_one_block() {
    let a = Adapter::new();
    let b = blob(None, &[], 0);
    a.edid(&b);
    let (out, n) = read_all(&a);
    assert_eq!(n, 128);
    assert_eq!(&out[..128], &b[..]);
}

#[test]
fn the_extensions_the_base_block_counts_are_read_after_it() {
    let a = Adapter::new();
    a.edid(&blob(None, &[], 1));
    a.bar.present8(128, 0x02);
    a.bar.present8(255, 0x5A);
    let (out, n) = read_all(&a);
    assert_eq!(n, 256);
    assert_eq!((out[128], out[255]), (0x02, 0x5A));
}

#[test]
fn a_count_larger_than_the_window_stops_at_the_window() {
    /*
     * Byte 126 is the device's word; 255 extensions would be 32 KiB, and
     * the window QEMU gives the blob is 1 KiB.
     */
    let a = Adapter::new();
    a.edid(&blob(None, &[], 255));
    assert_eq!(read_all(&a).1, EDID_WINDOW);
}
