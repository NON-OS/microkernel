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

//! MkIrqBind's copy-out failure path: it undoes the run of grants the call
//! made and nothing older. Releasing by device, as it once did, tore down
//! vectors an earlier bind had handed the same capsule.

const BIND: &str = include_str!("../../../src/syscall/microkernel/irq/bind.rs");
const RUN: &str = include_str!("../../../src/hardware/broker/irq/bind/grant_run.rs");

#[test]
fn the_failure_path_releases_by_grant_not_by_device() {
    assert!(!BIND.contains("irq_release_for_device"));
    assert!(BIND.contains("roll_back(pid, r.grant_id, r.grants)"));
    assert!(BIND.contains("irq_unmap_grant(pid, id)"));
}

#[test]
fn an_msix_run_reports_its_length() {
    assert!(RUN.contains("grants: n as u32"));
}
