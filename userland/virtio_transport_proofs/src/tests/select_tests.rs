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

//! Which transport a function gets, and where its register window lands.

use super::broker::FakeBroker;
use super::space::{
    qemu_modern, qemu_modern_bars, qemu_transitional_bars, Space, MODERN_NET, QEMU_MODERN_BAR,
    QEMU_MSIX_BAR, TRANSITIONAL_NET,
};
use crate::caps::{parse, ModernCaps};
use crate::map::{map_window, Need};
use crate::pci::ConfigSpace;
use crate::select::{choose, wants_probe, Kind};

#[test]
fn a_transitional_function_with_its_io_bar_stays_legacy_without_a_read() {
    let bars = qemu_transitional_bars();
    assert!(!wants_probe(TRANSITIONAL_NET, &bars), "no config read is made");
    let caps = parse(&qemu_modern(TRANSITIONAL_NET).cfg(), &bars);
    assert_eq!(choose(TRANSITIONAL_NET, &bars, &caps), Kind::Legacy);
}

#[test]
fn every_modern_only_id_is_probed_and_driven_modern() {
    for device in [0x1041u16, 0x1042, 0x1044, 0x1050] {
        let bars = qemu_modern_bars();
        assert!(wants_probe(device, &bars));
        let caps = parse(&qemu_modern(device).cfg(), &bars);
        assert_eq!(choose(device, &bars, &caps), Kind::Modern, "device {device:#x}");
    }
}

#[test]
fn a_modern_id_is_probed_even_beside_an_io_bar() {
    let bars = qemu_transitional_bars();
    assert!(wants_probe(MODERN_NET, &bars));
    let caps = parse(&qemu_modern(MODERN_NET).cfg(), &bars);
    assert_eq!(choose(MODERN_NET, &bars, &caps), Kind::Modern);
}

#[test]
fn a_transitional_id_without_an_io_bar_is_driven_modern() {
    let bars = qemu_modern_bars();
    assert!(wants_probe(TRANSITIONAL_NET, &bars));
    let caps = parse(&qemu_modern(TRANSITIONAL_NET).cfg(), &bars);
    assert_eq!(choose(TRANSITIONAL_NET, &bars, &caps), Kind::Modern);
}

#[test]
fn no_usable_common_cfg_leaves_the_legacy_path() {
    let bars = qemu_modern_bars();
    assert_eq!(choose(MODERN_NET, &bars, &ModernCaps::default()), Kind::Legacy);
    let s = Space::new(MODERN_NET, 0);
    assert_eq!(choose(MODERN_NET, &bars, &parse(&s.cfg(), &bars)), Kind::Legacy);
}

#[test]
fn the_snapshot_read_through_the_broker_is_the_config_space() {
    let space = qemu_modern(MODERN_NET);
    let mut broker = FakeBroker::new(&space, &qemu_modern_bars());
    let cfg = ConfigSpace::read(&mut broker).expect("every read answered");
    assert_eq!(broker.reads, 64, "one dword read per four bytes");
    assert_eq!(parse(&cfg, &qemu_modern_bars()), parse(&space.cfg(), &qemu_modern_bars()));
    // One refused read refuses the snapshot: a partial one would parse as
    // zeros and could hide the capability list.
    broker.refuse_read = Some(0x70);
    assert!(ConfigSpace::read(&mut broker).is_none());
}

/// The case found live: on a modern-only virtio-net the first MMIO BAR is
/// BAR1, the MSI-X table, which the broker refuses to map. The modern
/// window must come from the capabilities, all in BAR4, and no request may
/// name BAR1.
#[test]
fn the_window_of_a_qemu_modern_net_is_never_the_msix_bar() {
    let bars = qemu_modern_bars();
    let space = qemu_modern(MODERN_NET);
    let mut broker = FakeBroker::new(&space, &bars);
    let cfg = ConfigSpace::read(&mut broker).expect("snapshot");
    let caps = parse(&cfg, &bars);
    assert_eq!(choose(MODERN_NET, &bars, &caps), Kind::Modern);
    let window =
        map_window(&mut broker, &caps, Need { isr: true, device: true }).expect("maps in BAR4");
    assert!(!broker.requests.is_empty());
    for &(bar, _, _) in &broker.requests {
        assert_ne!(bar, QEMU_MSIX_BAR, "a request named the MSI-X BAR");
        assert_eq!(bar, QEMU_MODERN_BAR);
    }
    let lo = broker.bar_base(QEMU_MODERN_BAR);
    let hi = lo + broker.bar_len(QEMU_MODERN_BAR);
    let regions = [
        window.common.region,
        window.notify.region,
        window.isr.expect("isr").region,
        window.device.expect("device").region,
    ];
    for r in regions {
        let at = r.base() as usize;
        assert!(at >= lo && at + r.len() <= hi, "a register region outside BAR4");
    }
    assert_eq!(window.common.region.base() as usize, lo);
    assert_eq!(window.isr.expect("isr").region.base() as usize, lo + 0x1000);
    assert_eq!(window.device.expect("device").region.base() as usize, lo + 0x2000);
    assert_eq!(window.notify.region.base() as usize, lo + 0x3000);
}

#[test]
fn capabilities_placed_in_the_msix_bar_are_never_taken() {
    // A hostile layout: every structure named in BAR1, over the table.
    let bars = qemu_modern_bars();
    let mut s = Space::new(MODERN_NET, 0x98);
    s.msix(0x98, 0x40, 4, QEMU_MSIX_BAR, 0, QEMU_MSIX_BAR, 0x800);
    s.cap(0x40, 0x50, 1, QEMU_MSIX_BAR, 0, 0x1000);
    s.notify(0x50, 0x64, QEMU_MSIX_BAR, 0, 0x1000, 4);
    s.cap(0x64, 0x00, 4, QEMU_MSIX_BAR, 0x800, 0x100);
    let caps = parse(&s.cfg(), &bars);
    assert_eq!((caps.common, caps.notify, caps.device), (None, None, None));
    assert_eq!(choose(MODERN_NET, &bars, &caps), Kind::Legacy);
}
