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

//! The installer's client for the block wire (`nonos_blk_client`'s
//! `Driver::Wire`), held to driver.sdhci0's own source: the dialect, the
//! controllers it serves, the geometry it builds, the IDENTIFY parse, and
//! the rows the survey shows for a host nothing serves.

#[path = "../../../nonos_blk_client/src/driver/bus.rs"]
mod bus;
#[path = "../../../nonos_blk_client/src/driver/dialect.rs"]
mod dialect;
#[path = "../../../nonos_blk_client/src/driver/names.rs"]
mod names;
#[path = "../../../nonos_blk_client/src/driver/table.rs"]
mod table;
#[path = "../../../nonos_blk_client/src/device/unserved.rs"]
mod unserved;
#[path = "../../../nonos_blk_client/src/device/wire_identity_parse.rs"]
mod wire_identity_parse;

#[path = "../../../capsule_driver_sdhci/src/protocol/mod.rs"]
#[allow(dead_code, unused_imports)]
mod sdhci;
#[path = "../../../capsule_driver_ufs/src/platform/hosts.rs"]
#[allow(dead_code)]
mod ufs_hosts;

// Mounted once each, by the tests that hold them on their own.
use crate::blk_pci_tests::{identity_parse, pci};
use crate::blk_span_tests::{geometry, transfer};

mod dialect_tests;
mod geometry_tests;
mod identity_tests;
mod ufs_tests;
mod unserved_tests;
