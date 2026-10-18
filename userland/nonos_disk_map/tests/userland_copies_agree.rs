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

//! Two userland crates keep their own copies of the map. Each is read out
//! of its source here and compared.
//!
//! - The AHCI driver serves, of the SATA disks that come up, the one with
//!   the store's magic at the store's first sector or the plan's at the
//!   plan's (`userland/capsule_driver_ahci/src/choose/layout.rs`,
//!   `setup/probe.rs`). One that looks elsewhere takes a NONOS disk for a
//!   blank one and serves the lowest disk that came up instead.
//! - fs_proofs runs the vfs block layer over a stand-in for `nonos_libc`
//!   whose store calls answer as the kernel's do
//!   (`userland/fs_proofs/libc_shim/src/store.rs`). One whose window drifts
//!   from the map proves vfs against a window no kernel serves.

#[path = "lookup/defined.rs"]
mod defined;

use defined::{magic, rhs, source, value};
use nonos_disk_map::*;

const AHCI: &str = "../../userland/capsule_driver_ahci/src/choose/layout.rs";
const SHIM: &str = "../../userland/fs_proofs/libc_shim/src/store.rs";

#[test]
fn the_ahci_driver_picks_its_disk_by_the_same_sectors() {
    let layout = source(AHCI);
    assert_eq!(value(&layout, "STORE_LBA"), STORE_BASE_LBA);
    assert_eq!(rhs(&layout, "STORE_MAGIC"), magic(&STORE_MAGIC));
    assert_eq!(value(&layout, "PLAN_LBA"), PLAN_LBA);
    assert_eq!(rhs(&layout, "PLAN_MAGIC"), magic(&PLAN_MAGIC));
}

#[test]
fn the_proofs_stand_in_serves_the_store_window_the_map_names() {
    let store = source(SHIM);
    assert_eq!(value(&store, "STORE_BASE_LBA"), STORE_BASE_LBA);
    assert_eq!(value(&store, "PLAN_LBA"), PLAN_LBA);
}
