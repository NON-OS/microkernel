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

//! Where the host's DMA regions lie for its address width. ADMA2 in 32-bit
//! mode names nothing past 4 GiB, so the open asks the broker for DMA32 on a
//! host without 64-bit addressing (Capabilities bit 28), and only there. The
//! stand-in broker puts a map without DMA32 above 4 GiB, so each test holds
//! only if the driver asks on the right host and the broker honours it. A
//! 64-bit host then reads through 64-bit ADMA2: the table's address in
//! ADMA_ADDRESS_HI as well, and each descriptor's whole data address.

use nonos_libc::live_dma;

use super::open_tests::{found, DEAD_TABLE, SLOT};
use crate::emmc::pci::Kind;
use crate::emmc::platform::open;
use crate::model::{with, Armed, CardCfg, HostCfg, GLK_CAPS};

/// ADMA2 Act Tran: a descriptor that moves data.
const TRAN: u16 = 2;

#[test]
fn a_host_without_64_bit_addressing_has_its_regions_below_4_gib() {
    let cfg = HostCfg { caps: GLK_CAPS & !(1 << 28), ..HostCfg::default() };
    Armed::left_running(cfg, CardCfg::default(), DEAD_TABLE);
    let opened = open(&found(Kind::IntelEmmc, SLOT)).expect("the host opens, its card up");
    let live = live_dma();
    assert_eq!(live.len(), 2, "the table and the data buffer");
    assert!(live.iter().all(|&(at, _, n)| at + n as u64 <= 1 << 32), "past 4 GiB: {live:x?}");
    let violations = with(|h| h.sim.0.borrow().violations.clone());
    assert!(violations.is_empty(), "{violations:?}");
    drop(opened);
}

#[test]
fn a_64_bit_host_reads_its_regions_above_4_gib_through_64_bit_adma2() {
    Armed::left_running(HostCfg::default(), CardCfg::default(), DEAD_TABLE);
    let opened = open(&found(Kind::IntelEmmc, SLOT)).expect("the host opens, its card up");
    let live = live_dma();
    assert!(live.iter().all(|&(at, _, _)| at >> 32 != 0), "below 4 GiB: {live:x?}");
    let (violations, data) = with(|h| {
        let sim = h.sim.0.borrow();
        let tran = sim.tables.iter().flatten().filter(|d| d.attr >> 4 & 3 == TRAN);
        (sim.violations.clone(), tran.map(|d| d.addr).collect::<Vec<_>>())
    });
    assert!(violations.is_empty(), "{violations:?}");
    assert!(!data.is_empty() && data.iter().all(|a| a >> 32 != 0), "data at {data:x?}");
    drop(opened);
}
