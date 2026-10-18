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

//! A host whose Software Reset For All reads clear while its ADMA engine
//! still runs: Present State shows the engine busy, by any of Command Inhibit
//! (DAT), DAT Line Active, Write Transfer Active or Read Transfer Active
//! (SDHCI 3.00, 2.2.9), and the host is refused without Bus Master.

use nonos_libc::{refused_without_bus_master, seen};

use super::open_tests::{found, DEAD_TABLE, SLOT};
use crate::emmc::error::EmmcError;
use crate::emmc::pci::Kind;
use crate::emmc::platform::open;
use crate::model::{with, Armed, CardCfg, HostCfg};

/// Open a host whose reset bit clears with ADMA running, showing `busy`.
fn busy_after_reset(busy: u32) {
    Armed::left_running(HostCfg::default(), CardCfg::default(), DEAD_TABLE);
    with(|h| (h.runs_through_reset, h.busy) = (true, busy));
    let got = open(&found(Kind::IntelEmmc, SLOT)).err();
    refused_without_bus_master(&seen());
    assert_eq!(got, Some(EmmcError::HostBusy), "Present State {busy:#x}");
}

#[test]
fn a_host_whose_reset_bit_clears_with_adma_running_is_refused_without_bus_master() {
    busy_after_reset(1 << 1 | 1 << 2);
}

#[test]
fn a_write_transfer_still_active_after_the_reset_is_refused_too() {
    busy_after_reset(1 << 8);
}

#[test]
fn a_read_transfer_still_active_after_the_reset_is_refused_too() {
    busy_after_reset(1 << 9);
}
