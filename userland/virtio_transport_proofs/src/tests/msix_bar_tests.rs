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

//! The case found live: on a modern-only virtio-net the first MMIO BAR, BAR1,
//! is the MSI-X table, which the broker refuses to map. No register may be in it.

use super::broker::FakeBroker;
use super::space::{
    qemu_modern, qemu_modern_bars, Space, MODERN_NET, QEMU_MODERN_BAR, QEMU_MSIX_BAR,
};
use crate::caps::parse;
use crate::error::VirtioError::NoCommonCfg;
use crate::map::{map_window, Need};
use crate::select::modern_caps;

/// The modern window comes from the capabilities, all in BAR4; no request names BAR1.
#[test]
fn the_window_of_a_qemu_modern_net_is_never_the_msix_bar() {
    let bars = qemu_modern_bars();
    let space = qemu_modern(MODERN_NET);
    let mut broker = FakeBroker::new(&space, &bars);
    let need = Need { isr: true, device: true };
    let caps = modern_caps(&bars, &mut broker, need).expect("snapshot").expect("modern");
    let window = map_window(&mut broker, &caps, need).expect("maps in BAR4");
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
    // A modern-only id has no legacy registers to fall back on.
    let mut broker = FakeBroker::new(&s, &bars);
    assert_eq!(modern_caps(&bars, &mut broker, Need { isr: true, device: true }), Err(NoCommonCfg));
}
