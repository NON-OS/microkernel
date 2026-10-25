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

/// The device call sites, with the casts each may keep.
pub(super) fn device_sites() -> Vec<(&'static str, &'static str, Vec<&'static str>)> {
    let mut sites = vec![
        (
            "dispatch/irq.rs",
            include_str!("../../../../src/syscall/microkernel/dispatch/irq.rs"),
            vec![],
        ),
        (
            "dispatch/dma.rs",
            include_str!("../../../../src/syscall/microkernel/dispatch/dma.rs"),
            vec![],
        ),
        (
            "dispatch/pio.rs",
            include_str!("../../../../src/syscall/microkernel/dispatch/pio.rs"),
            vec![],
        ),
        (
            "dispatch/debug.rs",
            include_str!("../../../../src/syscall/microkernel/dispatch/debug.rs"),
            vec![],
        ),
        (
            "pci/config.rs",
            include_str!("../../../../src/syscall/microkernel/pci/config.rs"),
            vec![],
        ),
        (
            "dispatch/device.rs",
            include_str!("../../../../src/syscall/microkernel/dispatch/device.rs"),
            vec![],
        ),
        (
            "router/admin/policy_push/entry.rs",
            include_str!("../../../../src/syscall/dispatch/router/admin/policy_push/entry.rs"),
            vec![],
        ),
    ];
    sites.extend(super::surface_sites::surface_sites());
    sites
}
