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

//! Bus Master at open, on a host a driver that died without its Drop left
//! with ADMA2 running from its own descriptor table, a page the kernel has
//! since freed: it goes on only once the full reset is seen to finish, with
//! no DMA region mapped before that, never for a host refused, and off again
//! once the host is stopped; the release is the last call and frees the
//! regions.

use nonos_libc::{bus_master_after_reset, bus_master_off_after_stop, refused_without_bus_master};
use nonos_libc::{released_last, seen, Bar, BAR_KIND_MMIO};

use crate::emmc::error::EmmcError;
use crate::emmc::pci::Kind;
use crate::emmc::platform::{open, Found};
use crate::model::{Armed, CardCfg, HostCfg, GLK_CAPS};

/// The dead driver's descriptor table, on the bus.
pub const DEAD_TABLE: u64 = 0x7f00_0000;
/// Slot 0's BAR: 4 KiB of registers.
pub const SLOT: Bar =
    Bar { base: 0xfe00_0000, size: 0x1000, aux: 0, kind: BAR_KIND_MMIO, flags: 0, _pad: [0; 2] };

/// A host of `kind`, slot 0 behind `bar`.
pub fn found(kind: Kind, bar: Bar) -> Found {
    let mut bars = [Bar::empty(); 6];
    bars[0] = bar;
    Found { device_id: 5, vendor: 0x8086, device: 0x31cc, kind, bars, bar_count: 1, acpi: false }
}

#[test]
fn a_host_left_running_adma_is_reset_before_bus_master_goes_on() {
    let host = Armed::left_running(HostCfg::default(), CardCfg::default(), DEAD_TABLE);
    let opened = open(&found(Kind::IntelEmmc, SLOT)).expect("the host opens, its card up");
    bus_master_after_reset(&seen());
    let violations = host.borrow().sim.0.borrow().violations.clone();
    assert!(violations.is_empty(), "{violations:?}");
    let disk = opened.slot.disk.as_ref().expect("the card is up");
    assert!(disk.capacity_sectors() > 0, "sector 0 was read through the regions");
    drop(opened);
    let seen = seen();
    bus_master_off_after_stop(&seen);
    released_last(&seen);
}

#[test]
fn a_host_whose_reset_never_ends_is_refused_without_bus_master() {
    let cfg = HostCfg { reset_reads: u32::MAX, ..HostCfg::default() };
    let host = Armed::left_running(cfg, CardCfg::default(), DEAD_TABLE);
    assert_eq!(open(&found(Kind::IntelEmmc, SLOT)).err(), Some(EmmcError::HostReset));
    refused_without_bus_master(&seen());
    assert_eq!(host.borrow().from, Some(DEAD_TABLE), "a reset that never ended stopped nothing");
}

#[test]
fn a_host_without_adma2_is_refused_without_bus_master() {
    let cfg = HostCfg { caps: GLK_CAPS & !(1 << 19), ..HostCfg::default() };
    let _host = Armed::left_running(cfg, CardCfg::default(), DEAD_TABLE);
    assert_eq!(open(&found(Kind::IntelEmmc, SLOT)).err(), Some(EmmcError::NoAdma));
    refused_without_bus_master(&seen());
}
