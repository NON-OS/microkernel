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

//! A host refused before its reset, or given no DMA region: never Bus Master.

use nonos_libc::{refuse, refused_without_bus_master, seen, Bar, Call, BAR_KIND_PIO};

use super::open_tests::{found, DEAD_TABLE, SLOT};
use crate::emmc::error::EmmcError;
use crate::emmc::pci::Kind;
use crate::emmc::platform::open;
use crate::model::{Armed, CardCfg, HostCfg, GLK_CAPS};

fn left_running(cfg: HostCfg) {
    Armed::left_running(cfg, CardCfg::default(), DEAD_TABLE);
}

#[test]
fn a_generic_host_whose_slot_is_not_embedded_is_refused_without_bus_master() {
    // Slot Type 0: a removable slot, a card reader.
    left_running(HostCfg { caps: GLK_CAPS & !(3 << 30), ..HostCfg::default() });
    assert_eq!(open(&found(Kind::Generic, SLOT)).err(), Some(EmmcError::NotEmbedded));
    refused_without_bus_master(&seen());
}

#[test]
fn a_slot_bar_that_is_no_register_window_is_refused_without_bus_master() {
    left_running(HostCfg::default());
    for bar in [Bar { kind: BAR_KIND_PIO, ..SLOT }, Bar { size: 0x80, ..SLOT }] {
        assert_eq!(open(&found(Kind::IntelEmmc, bar)).err(), Some(EmmcError::Window));
    }
    refused_without_bus_master(&seen());
}

#[test]
fn a_window_the_broker_will_not_map_is_refused_without_bus_master() {
    left_running(HostCfg::default());
    refuse(Call::MmioMap);
    assert_eq!(open(&found(Kind::IntelEmmc, SLOT)).err(), Some(EmmcError::Broker(-12)));
    refused_without_bus_master(&seen());
}

#[test]
fn dma_regions_the_broker_will_not_give_are_asked_for_only_after_the_reset() {
    left_running(HostCfg::default());
    refuse(Call::DmaMap);
    assert_eq!(open(&found(Kind::IntelEmmc, SLOT)).err(), Some(EmmcError::Broker(-12)));
    let seen = seen();
    refused_without_bus_master(&seen);
    for s in seen.iter().filter(|s| s.call == Call::DmaMap) {
        assert!(s.device.resets > 0 && s.device.running == 0, "asked before the reset: {seen:?}");
    }
}
