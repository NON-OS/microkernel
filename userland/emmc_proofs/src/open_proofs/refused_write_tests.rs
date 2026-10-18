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

//! A Command write the broker refuses, the claim's write of memory decoding
//! or the Bus Master write once the regions are mapped: the host is
//! released either way, Bus Master never on.

use nonos_libc::{refuse, refused_without_bus_master, seen, Call, Seen};
use nonos_libc::{MK_PCI_CMD_BUS_MASTER, MK_PCI_CMD_MEMORY_SPACE};

use super::open_tests::{found, DEAD_TABLE, SLOT};
use crate::emmc::error::EmmcError;
use crate::emmc::pci::Kind;
use crate::emmc::platform::open;
use crate::model::{Armed, CardCfg, HostCfg};

/// Open a host whose broker refuses every Command write carrying `bits`.
fn refused(bits: u16) -> Vec<Seen> {
    let _host = Armed::left_running(HostCfg::default(), CardCfg::default(), DEAD_TABLE);
    refuse(Call::Command(bits));
    assert_eq!(open(&found(Kind::IntelEmmc, SLOT)).err(), Some(EmmcError::Broker(-16)));
    seen()
}

#[test]
fn a_memory_space_write_the_broker_refuses_releases_the_claim() {
    let seen = refused(MK_PCI_CMD_MEMORY_SPACE);
    assert!(seen.iter().all(|s| s.call != Call::MmioMap), "mapped after the refusal: {seen:?}");
    refused_without_bus_master(&seen);
}

#[test]
fn a_bus_master_write_the_broker_refuses_releases_with_bus_master_never_on() {
    let seen = refused(MK_PCI_CMD_BUS_MASTER);
    let on = |s: &&Seen| matches!(s.call, Call::Command(w) if w & MK_PCI_CMD_BUS_MASTER != 0);
    let asked: Vec<_> = seen.iter().filter(on).collect();
    assert!(asked.len() == 1 && asked[0].refused, "asked once, and refused: {seen:?}");
    refused_without_bus_master(&seen);
}
