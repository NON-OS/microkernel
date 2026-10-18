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

//! A host given back once its DMA regions are mapped: stopped first, and Bus
//! Master written off if it went on, then the release, which frees the
//! regions. None is unmapped by hand before it: the host could still reach
//! it, and the broker ORs Command bits until K6, so the driver's own "off"
//! write cannot stop it first.

use nonos_libc::{bus_master_after_reset, bus_master_off_after_stop, refuse_after};
use nonos_libc::{refused_without_bus_master, released_last, seen, Call};

use super::open_tests::{found, DEAD_TABLE, SLOT};
use crate::emmc::error::EmmcError;
use crate::emmc::pci::Kind;
use crate::emmc::platform::open;
use crate::model::{Armed, CardCfg, HostCfg};

#[test]
fn a_host_whose_card_never_answers_is_stopped_before_bus_master_goes_off() {
    let card = CardCfg { absent: true, ..CardCfg::default() };
    let _host = Armed::left_running(HostCfg::default(), card, DEAD_TABLE);
    assert_eq!(open(&found(Kind::IntelEmmc, SLOT)).err(), Some(EmmcError::NoCard));
    let seen = seen();
    bus_master_after_reset(&seen);
    bus_master_off_after_stop(&seen);
    released_last(&seen);
}

#[test]
fn a_data_buffer_the_broker_will_not_give_leaves_the_table_to_the_release() {
    let _host = Armed::left_running(HostCfg::default(), CardCfg::default(), DEAD_TABLE);
    refuse_after(Call::DmaMap, 1);
    assert_eq!(open(&found(Kind::IntelEmmc, SLOT)).err(), Some(EmmcError::Broker(-12)));
    let seen = seen();
    let maps = seen.iter().filter(|s| s.call == Call::DmaMap);
    assert!(maps.clone().all(|s| s.device.resets > 0 && s.device.running == 0), "{seen:?}");
    assert_eq!(maps.count(), 2, "the table was mapped, the buffer refused: {seen:?}");
    refused_without_bus_master(&seen);
}
