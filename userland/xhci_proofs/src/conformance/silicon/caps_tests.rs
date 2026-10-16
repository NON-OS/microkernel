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

//! What real controllers lay out: an extended capability list far into BAR0
//! and separate USB 2 and USB 3 port ranges.

use super::super::model::{controller, LEGACY, MAPPED, XECP_WORDS};
use super::caps::{lay_out_caps, supported_protocol};
use crate::regs::cap::PortProtocols;

#[test]
fn port_protocols_split_usb2_and_usb3_ranges_as_an_intel_pch_does() {
    // GLK-like: USB 2 on ports 1 to 8, USB 3 on 9 to 15.
    let bar = controller(XECP_WORDS);
    lay_out_caps(
        &bar,
        &[
            (LEGACY, supported_protocol(2), 1 | (8 << 8)),
            (LEGACY + 0x20, supported_protocol(3), 9 | (7 << 8)),
        ],
    );
    let ports = PortProtocols::read(bar.base(), MAPPED);
    for port in 1..=8 {
        assert_eq!(ports.major(port), 2, "port {port}");
        assert!(!ports.is_usb3(port));
    }
    for port in 9..=15 {
        assert!(ports.is_usb3(port), "port {port}");
    }
    assert_eq!(ports.major(16), 0, "a port no capability names is unknown");
}

#[test]
fn the_capability_walk_stays_inside_the_mapped_window() {
    let hcc = (0x100u32 << 16) | 1;
    let reads = std::cell::RefCell::new(Vec::new());
    // A first capability inside, pointing at one that ends past the window.
    let window = 0x408;
    let read = |off: u64| {
        reads.borrow_mut().push(off);
        match off {
            0x400 => supported_protocol(2) | (1 << 8),
            0x408 => 1 | (2 << 8),
            _ => 0xDEAD_BEEF,
        }
    };
    let ports = PortProtocols::parse(hcc, window + 12, read);
    assert_eq!(ports.major(1), 2);
    assert!(reads.borrow().iter().all(|&o| o + 4 <= window + 12), "{:x?}", reads.borrow());
    // The same list with the window ending inside the first capability
    // reads nothing at all.
    reads.borrow_mut().clear();
    let ports = PortProtocols::parse(hcc, 0x404, read);
    assert_eq!(ports.major(1), 0);
    assert!(reads.borrow().is_empty());
}
