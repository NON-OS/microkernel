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

//! The versions: newest first, the contact's form before and from 5.0,
//! and the connection the host names.

use super::util::{host, usual, GRANT, ROOT};
use crate::setup;

fn u64_at(p: &[u8], at: usize) -> u64 {
    u64::from_le_bytes(p[at..at + 8].try_into().unwrap())
}

fn version(p: &[u8]) -> u32 {
    u32::from_le_bytes(p[8..12].try_into().unwrap())
}

#[test]
fn an_older_host_is_walked_down_to() {
    let m = host(usual());
    m.0.borrow_mut().supported = vec![0x4_0000];
    let d = setup::run(ROOT).expect("up on 4.0");
    assert_eq!((d.bus.version, d.bus.connection), (0x4_0000, 1));
    let s = m.0.borrow();
    let asked: Vec<u32> = s.contacts.iter().map(|c| version(c)).collect();
    assert_eq!(asked, [0x5_0003, 0x5_0002, 0x5_0001, 0x5_0000, 0x4_0001, 0x4_0000]);
    let conns: Vec<u32> = s.log.iter().map(|l| l.0).collect();
    assert_eq!(conns, [4, 4, 4, 4, 1, 1, 1], "4 for 5.x, then 1, the offers on 1");
    let c = s.contacts.last().unwrap();
    assert_eq!(u64_at(c, 16), GRANT, "before 5.0 the interrupt page is named");
}

#[test]
fn windows_8_is_the_oldest_asked() {
    let m = host(usual());
    m.0.borrow_mut().supported = vec![0x1_0001, 0x0000_000D];
    assert_eq!(setup::run(ROOT).err(), Some("no VMBus version in common"));
    let s = m.0.borrow();
    assert_eq!(s.contacts.len(), 8);
    assert_eq!(version(s.contacts.last().unwrap()), 0x2_0004);
    assert!(!s.unloaded);
}

#[test]
fn the_connection_the_host_names() {
    for (named, want) in [(0, 4), (4, 4), (1, 1)] {
        let m = host(usual());
        m.0.borrow_mut().named = named;
        let d = setup::run(ROOT).expect("up");
        assert_eq!(d.bus.connection, want, "named {named}");
        assert_eq!(m.0.borrow().log.last(), Some(&(want, 3)), "offers asked there");
    }
}

#[test]
fn a_connection_the_kernel_refuses_ends_the_contact_attempt() {
    let m = host(usual());
    m.0.borrow_mut().named = 7;
    assert_eq!(setup::run(ROOT).err(), Some("host named a message connection the kernel refuses"));
}

#[test]
fn qemu_s_versions() {
    // QEMU's vmbus takes up to 5.0.
    let m = host(usual());
    m.0.borrow_mut().supported = vec![0x5_0000, 0x4_0001, 0x4_0000, 0x3_0000, 0x2_0004];
    let d = setup::run(ROOT).expect("up");
    assert_eq!((d.bus.version, d.bus.connection), (0x5_0000, 4));
}
