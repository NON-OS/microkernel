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

//! The IOC's request frame size: anything from the SCSI request with one
//! SGE up to the request region of the grant, the emulated SAS1068's 128
//! words among them, and at bring-up as well as by the rule alone.

use super::util::grant;
use crate::frame::{served, LEAST, MOST};
use crate::ioc;
use crate::model::ioc as model;

#[test]
fn the_edges_of_the_rule() {
    assert_eq!((LEAST, MOST), (56, 0x400));
    assert!(!served(13), "52 bytes is short of the request and its SGE");
    assert!(served(14) && served(32) && served(128) && served(256));
    assert!(!served(257), "past the request region");
    assert!(!served(0) && !served(u16::MAX));
}

#[test]
fn bring_up_with_each_frame_size() {
    // One grant: the shim answers every map with the same device address.
    let g = grant();
    let sizes = [(13, false), (14, true), (32, true), (128, true), (256, true), (257, false)];
    for (dwords, up) in sizes {
        let m = model(vec![]);
        m.0.borrow_mut().frame_dwords = dwords;
        let r = ioc::start(&m, g.user_va, g.device_addr);
        assert_eq!(r.is_ok(), up, "{dwords} words: {r:?}");
        if !up {
            assert_eq!(r, Err("IOC request frames not served"));
        }
    }
}
