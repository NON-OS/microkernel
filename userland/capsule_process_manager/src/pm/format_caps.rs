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

//! Capability bits and their short names, in the kernel's enum order.

/* Every bit the kernel defines, through LocalSign. A grant with no entry here
 * was counted in a process's authority but drawn as no chip at all. */
pub const CAP_TABLE: &[(u64, &[u8])] = &[
    (1 << 0, b"exec"),
    (1 << 1, b"io"),
    (1 << 2, b"net"),
    (1 << 3, b"ipc"),
    (1 << 4, b"mem"),
    (1 << 5, b"crypto"),
    (1 << 6, b"fs"),
    (1 << 7, b"hw"),
    (1 << 8, b"debug"),
    (1 << 9, b"admin"),
    (1 << 10, b"regsvc"),
    (1 << 11, b"gquery"),
    (1 << 12, b"gcreate"),
    (1 << 13, b"gmap"),
    (1 << 14, b"gpresent"),
    (1 << 15, b"devenum"),
    (1 << 16, b"driver"),
    (1 << 17, b"mmio"),
    (1 << 18, b"irq"),
    (1 << 19, b"dma"),
    (1 << 20, b"pio"),
    (1 << 21, b"input"),
    (1 << 22, b"time"),
    (1 << 23, b"spawnbroker"),
    (1 << 24, b"spawnwindow"),
    (1 << 25, b"procctl"),
    (1 << 26, b"storewrite"),
    (1 << 27, b"enrolroot"),
    (1 << 28, b"keyring"),
    (1 << 29, b"entropy"),
    (1 << 30, b"install"),
    (1 << 31, b"attest"),
    (1 << 32, b"foreign"),
    (1 << 33, b"localsign"),
];
