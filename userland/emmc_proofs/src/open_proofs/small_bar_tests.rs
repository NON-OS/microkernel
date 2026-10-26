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

//! A slot BAR under a page. QEMU's sdhci-pci has one of 0x100, a single
//! slot's registers; the broker maps the page it sits in and hands back its
//! first byte, so the driver asks for the BAR as it is.

use nonos_libc::{seen, Bar, Call};

use super::open_tests::{found, DEAD_TABLE, SLOT};
use crate::emmc::error::EmmcError;
use crate::emmc::pci::Kind;
use crate::emmc::platform::open;
use crate::model::{Armed, CardCfg, HostCfg};

#[test]
fn a_bar_of_one_slot_opens_the_host_and_its_card() {
    let _host = Armed::left_running(HostCfg::default(), CardCfg::default(), DEAD_TABLE);
    let opened = open(&found(Kind::Generic, Bar { size: 0x100, ..SLOT })).expect("it opens");
    let disk = opened.slot.disk.as_ref().expect("the card is up");
    assert!(disk.capacity_sectors() > 0);
    drop(opened);
    assert!(!seen().is_empty());
}

#[test]
fn a_bar_short_of_one_slot_is_refused_before_any_map() {
    for size in [0, 0x80, 0xFF] {
        let _host = Armed::left_running(HostCfg::default(), CardCfg::default(), DEAD_TABLE);
        let bar = Bar { size, ..SLOT };
        assert_eq!(open(&found(Kind::Generic, bar)).err(), Some(EmmcError::Window), "{size:#x}");
        assert!(seen().iter().all(|s| s.call != Call::MmioMap), "nothing mapped: {size:#x}");
    }
}

#[test]
fn a_bar_past_a_page_by_less_than_one_still_opens() {
    let _host = Armed::left_running(HostCfg::default(), CardCfg::default(), DEAD_TABLE);
    assert!(open(&found(Kind::Generic, Bar { size: 0x1100, ..SLOT })).is_ok());
}
