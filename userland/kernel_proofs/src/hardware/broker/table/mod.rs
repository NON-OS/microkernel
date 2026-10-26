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

/*
 * The device table's storage and its removal, by path as the kernel builds
 * them, for the proof that an id is never handed to a second device; and the
 * fixed windows a PCI record gets as it is built, for the proof that the
 * inventory reads such a record as the kernel lists it.
 */

#[path = "../../../../../../src/hardware/broker/table/remove.rs"]
mod remove;
#[path = "../../../../../../src/hardware/broker/table/state.rs"]
mod state;

#[cfg(test)]
mod fixed_window_tests;
#[cfg(test)]
mod id_tests;
// The kernel's `legacy::attach` by path; the window rules it reads are the
// ones `legacy_ports` mounts. Only the test module places windows, so only a
// test build has them.
#[cfg(test)]
#[path = "../../../../../../src/hardware/broker/legacy/attach.rs"]
mod attach;
#[cfg(test)]
use crate::legacy_ports::{compat, dispi};
