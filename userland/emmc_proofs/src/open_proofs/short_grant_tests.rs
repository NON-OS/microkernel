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

//! A DMA grant shorter than the region asked for is refused: the ADMA2
//! table and the data buffer are sized for what was asked, and a host given
//! less is never turned on to master the bus, its claim given back.

use nonos_libc::{grant_short, refused_without_bus_master, released_last, seen, Call};

use super::open_tests::{found, DEAD_TABLE, SLOT};
use crate::emmc::error::EmmcError;
use crate::emmc::pci::Kind;
use crate::emmc::platform::open;
use crate::model::{Armed, CardCfg, HostCfg};

#[test]
fn a_grant_short_of_the_region_asked_is_refused_without_bus_master() {
    for bytes in [1, 512, 4096] {
        Armed::left_running(HostCfg::default(), CardCfg::default(), DEAD_TABLE);
        grant_short(bytes);
        let got = open(&found(Kind::IntelEmmc, SLOT)).err();
        assert_eq!(got, Some(EmmcError::Broker(-12)), "{bytes} short");
        let seen = seen();
        assert!(seen.iter().any(|s| s.call == Call::DmaMap), "a region was asked for");
        refused_without_bus_master(&seen);
        released_last(&seen);
    }
}

#[test]
fn a_whole_grant_opens_the_host() {
    Armed::left_running(HostCfg::default(), CardCfg::default(), DEAD_TABLE);
    grant_short(0);
    assert!(open(&found(Kind::IntelEmmc, SLOT)).is_ok(), "the host opens, its card up");
}
