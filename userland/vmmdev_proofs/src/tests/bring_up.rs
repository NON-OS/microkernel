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

//! The introduction, against a host as VirtualBox answers.

use nonos_libc::{calls, logged, MK_DMA_MAP_COHERENT, MK_DMA_MAP_DMA32};

use super::util::{up, DEVICE};
use crate::say;
use crate::setup::claim::COMMAND_BITS;

#[test]
fn the_grants_are_taken_in_order_and_kept() {
    let (_d, _host) = up();
    let flags = (MK_DMA_MAP_DMA32 | MK_DMA_MAP_COHERENT) as u64;
    let want = vec![
        ("claim", DEVICE),
        ("pci_command", COMMAND_BITS as u64),
        ("pio_grant", 0),
        ("mmio_map", 1),
        ("dma_map", flags),
    ];
    assert_eq!(calls(), want);
}

// Integration last: once it is on the host stops moving the PS/2 pointer.
#[test]
fn the_guest_is_introduced_and_integration_turned_on_last() {
    let (_d, host) = up();
    let host = host.borrow();
    assert_eq!(host.types(), vec![4, 50, 42, 56, 41, 2]);
    assert!(host.requests.iter().all(|r| r.1 == 0), "every request carried out");
    assert_eq!(host.info, Some((0x0001_0004, 0x100)), "no Guest Additions claimed");
    assert_eq!(host.filter, 1 << 9, "the mouse's event alone");
    assert_eq!(host.caps, 0, "no seamless, graphics or window mapping offered");
    assert_eq!(host.guest_mouse, 1 | (1 << 4));
}

#[test]
fn the_host_version_is_read_and_said() {
    let (d, _host) = up();
    assert_eq!((d.host.major, d.host.minor, d.host.build), (7, 0, 20));
    say::up(&d.host);
    let line = logged().pop().unwrap();
    assert_eq!(line, "driver.vmmdev: up, VirtualBox 7.0.20, mouse integration on\n");
}
