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

//! Mapping the window: page-aligned requests, all or nothing.

use super::broker::FakeBroker;
use super::space::{qemu_modern, qemu_modern_bars, MODERN_NET};
use crate::caps::{parse, ModernCaps, Region};
use crate::error::VirtioError;
use crate::map::{map_window, plan, Need, NOTIFY_MAP_MAX, REGION_MAP_MAX};

const ALL: Need = Need { isr: true, device: true };

fn qemu() -> (FakeBroker, ModernCaps) {
    let space = qemu_modern(MODERN_NET);
    let caps = parse(&space.cfg(), &qemu_modern_bars());
    (FakeBroker::new(&space, &qemu_modern_bars()), caps)
}

#[test]
fn every_request_is_whole_pages_covering_the_region() {
    for (offset, length, cap) in [
        (0x0u32, 0x38u32, REGION_MAP_MAX),
        (0x100, 0x38, REGION_MAP_MAX),
        (0xFF8, 0x10, REGION_MAP_MAX),
        (0x3000, 0x1000, NOTIFY_MAP_MAX),
        (0x3002, 0x40_0000, NOTIFY_MAP_MAX),
    ] {
        let p = plan(Region { bar: 4, offset, length }, cap).expect("a plan");
        assert_eq!(p.map_offset % 4096, 0);
        assert_eq!(p.map_len % 4096, 0);
        assert_eq!(p.map_offset + p.in_page as u64, offset as u64);
        assert_eq!(p.usable as u32, length.min(cap));
        assert!(p.in_page as u64 + p.usable as u64 <= p.map_len, "the region fits its pages");
        assert!(p.map_len - (p.in_page as u64 + p.usable as u64) < 4096, "no extra page");
    }
    assert_eq!(plan(Region { bar: 4, offset: 0, length: 0 }, REGION_MAP_MAX), None);
}

#[test]
fn a_huge_notify_area_is_mapped_only_as_far_as_the_cap() {
    // One page per queue for 1024 queues: four megabytes of doorbells.
    let p = plan(Region { bar: 4, offset: 0, length: 0x40_0000 }, NOTIFY_MAP_MAX).expect("plan");
    assert_eq!(p.map_len, NOTIFY_MAP_MAX as u64);
}

#[test]
fn the_qemu_window_maps_four_regions_and_unmaps_them_all() {
    let (mut broker, caps) = qemu();
    let window = map_window(&mut broker, &caps, ALL).expect("window");
    assert_eq!(
        broker.requests,
        vec![(4, 0x0000, 0x1000), (4, 0x3000, 0x1000), (4, 0x1000, 0x1000), (4, 0x2000, 0x1000)]
    );
    assert_eq!(window.notify_area().multiplier, 4);
    assert_eq!(window.notify_area().len, 0x1000);
    assert_eq!(broker.live.len(), 4);
    assert!(window.unmap(&mut broker));
    assert!(broker.live.is_empty());
}

#[test]
fn only_the_needed_structures_are_mapped() {
    let (mut broker, caps) = qemu();
    let window =
        map_window(&mut broker, &caps, Need { isr: false, device: false }).expect("window");
    assert_eq!(broker.requests.len(), 2);
    assert!(window.isr.is_none() && window.device.is_none());
    assert_eq!(window.grant_ids().iter().flatten().count(), 2);
}

#[test]
fn a_refused_map_undoes_every_earlier_one() {
    for refuse in 0..4 {
        let (mut broker, caps) = qemu();
        broker.refuse_map = Some(refuse);
        assert_eq!(map_window(&mut broker, &caps, ALL).err(), Some(VirtioError::MapRefused));
        assert!(broker.live.is_empty(), "grants left behind after refusal {refuse}");
        assert_eq!(broker.unmapped.len(), refuse);
    }
}

#[test]
fn a_short_map_is_refused_and_undone() {
    // The kernel cuts a mapping short below an MSI-X table and says so in
    // the length; a window over the missing part would fault.
    for short in 0..4 {
        let (mut broker, caps) = qemu();
        broker.short_map = Some(short);
        assert_eq!(map_window(&mut broker, &caps, ALL).err(), Some(VirtioError::MapShort));
        assert!(broker.live.is_empty(), "grants left behind after short map {short}");
        assert_eq!(broker.unmapped.len(), short + 1, "the short grant is undone too");
    }
}

#[test]
fn a_missing_structure_maps_nothing() {
    let (mut broker, caps) = qemu();
    for (field, want) in [
        (0, VirtioError::NoCommonCfg),
        (1, VirtioError::NoNotifyCfg),
        (2, VirtioError::NoIsrCfg),
        (3, VirtioError::NoDeviceCfg),
    ] {
        let mut c = caps;
        match field {
            0 => c.common = None,
            1 => c.notify = None,
            2 => c.isr = None,
            _ => c.device = None,
        }
        assert_eq!(map_window(&mut broker, &c, ALL).err(), Some(want));
    }
    assert!(broker.requests.is_empty(), "nothing is asked for before every region is known");
}

#[test]
fn a_region_not_at_a_page_start_is_reached_at_its_own_offset() {
    let (mut broker, mut caps) = qemu();
    caps.common = Some(Region { bar: 4, offset: 0x140, length: 0x38 });
    let window = map_window(&mut broker, &caps, ALL).expect("window");
    assert_eq!(broker.requests[0], (4, 0, 0x1000));
    assert_eq!(window.common.region.base() as usize, broker.bar_base(4) + 0x140);
    assert_eq!(window.common.region.len(), 0x38);
}
