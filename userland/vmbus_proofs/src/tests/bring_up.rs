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

//! The claim, the grant, what the contact names, and every failure giving
//! back what was taken.

use nonos_libc::{calls, script, script_dma, set_devices, DeviceRecord, Script, BUS_KIND_PCI};

use super::util::{host, root, up, usual, GRANT, ROOT};
use crate::discover::{find, found};
use crate::setup;

fn u64_at(p: &[u8], at: usize) -> u64 {
    u64::from_le_bytes(p[at..at + 8].try_into().unwrap())
}

#[test]
fn the_root_is_the_virt_record_alone() {
    root();
    assert_eq!(find(), Some(ROOT));
    let rec = |bus, vendor, device| DeviceRecord { device_id: 9, bus_kind: bus, vendor, device };
    assert_eq!(found(&rec(BUS_KIND_PCI, 0x1414, 0x5642)), None, "a PCI function");
    assert_eq!(found(&rec(3, 0x1414, 0x5643)), None);
    assert_eq!(found(&rec(3, 0x1AF4, 0x5642)), None);
    set_devices(vec![]);
    assert_eq!(find(), None);
}

#[test]
fn claim_grant_contact_offers_in_order() {
    let (d, m) = up(usual());
    assert_eq!(calls(), [("claim", ROOT), ("dma_map", 1 << 2)]);
    let s = m.0.borrow();
    assert_eq!(s.log, [(4, 14), (4, 3)], "the contact, then the offers, on connection 4");
    let contact = &s.contacts[0];
    assert_eq!(u64_at(contact, 24), GRANT + 4096, "monitor page 1");
    assert_eq!(u64_at(contact, 32), GRANT + 8192, "monitor page 2");
    assert_eq!((d.bus.version, d.bus.connection), (0x5_0003, 4));
    let relids: Vec<u32> = d.offers.iter().map(|o| o.relid).collect();
    assert_eq!(relids, [1, 2, 3, 4, 5]);
}

#[test]
fn claim_refused() {
    root();
    host(usual());
    script(Script { claim: -16 });
    assert_eq!(setup::run(ROOT).err(), Some("claim failed"));
    assert_eq!(calls(), [("claim", ROOT)]);
}

#[test]
fn a_grant_refused_or_short_gives_the_claim_back() {
    host(usual());
    script_dma(Err(-12));
    assert_eq!(setup::run(ROOT).err(), Some("dma map failed"));
    assert_eq!(calls().last(), Some(&("device_release", ROOT)));
    script_dma(Ok((GRANT, 4096)));
    assert_eq!(setup::run(ROOT).err(), Some("dma grant short"));
    assert_eq!(calls()[calls().len() - 2..], [("dma_unmap", 70), ("device_release", ROOT)]);
}

#[test]
fn a_silent_host_is_given_back_untouched() {
    let m = host(usual());
    m.0.borrow_mut().silent = true;
    assert_eq!(setup::run(ROOT).err(), Some("no answer to the contact"));
    assert_eq!(calls()[calls().len() - 2..], [("dma_unmap", 70), ("device_release", ROOT)]);
    assert!(!m.0.borrow().unloaded, "never contacted, nothing to unload");
}

#[test]
fn offers_never_finished_unload_the_contact() {
    let m = host(usual());
    m.0.borrow_mut().deliver_all = false;
    assert_eq!(setup::run(ROOT).err(), Some("offers not all delivered"));
    let s = m.0.borrow();
    assert!(s.unloaded);
    assert_eq!(s.log.last(), Some(&(4, 16)));
    assert_eq!(calls()[calls().len() - 2..], [("dma_unmap", 70), ("device_release", ROOT)]);
}

#[test]
fn a_busy_host_is_waited_for() {
    let m = host(usual());
    m.0.borrow_mut().busy = 30;
    assert!(setup::run(ROOT).is_ok());
}

#[test]
fn a_host_busy_too_long_is_given_up_on() {
    let m = host(usual());
    m.0.borrow_mut().busy = u32::MAX;
    assert_eq!(setup::run(ROOT).err(), Some("host had no room for a message"));
}
