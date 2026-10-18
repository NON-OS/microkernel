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

//! A header that is not a configuration's, buffers too short to hold one,
//! and every record head after a valid one.

use crate::config_blob::{config, keyboard};
use crate::descriptors::hid_bindings;

#[test]
fn a_header_that_is_not_a_configuration_is_refused() {
    let raw = keyboard(0).build();
    for len in 0..9 {
        assert!(hid_bindings(&raw[..len]).is_err(), "{len} byte buffer");
    }
    for header_len in 0u8..9 {
        let mut d = raw.clone();
        d[0] = header_len;
        assert!(hid_bindings(&d).is_err(), "header bLength {header_len}");
    }
    for ty in (0..=u8::MAX).filter(|&t| t != 0x02) {
        let mut d = raw.clone();
        d[1] = ty;
        assert!(hid_bindings(&d).is_err(), "header type {ty:#x}");
    }
}

#[test]
fn every_short_buffer_is_refused() {
    assert!(hid_bindings(&[]).is_err());
    for a in 0..=u8::MAX {
        assert!(hid_bindings(&[a]).is_err());
        for b in 0..=u8::MAX {
            assert!(hid_bindings(&[a, b]).is_err());
        }
    }
}

#[test]
fn every_record_head_after_a_header_is_walked_without_a_panic() {
    // Every bLength and type for one record, padded with each of three fill
    // bytes, behind a valid header: the walk reads only what is there.
    for len in 0..=u8::MAX {
        for ty in 0..=u8::MAX {
            for fill in [0x00u8, 0x81, 0xff] {
                let mut rec = vec![len, ty];
                rec.resize(2 + (len as usize % 13), fill);
                let _ = hid_bindings(&config().raw(&rec).build());
            }
        }
    }
}
