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

//! The PCI `_DSM` (PCI Firmware Specification, the `_DSM` chapter), read-only
//! and for three documented functions alone: whether the boot configuration
//! must be preserved (function 5, on a host bridge), the device readiness
//! durations (function 9), and a device's firmware label (function 7, for
//! the inventory). No other function is ever evaluated, whatever function 0
//! offers. The UUID, revisions, function numbers and return shapes are
//! recalled, not read from the specification, and are unverified.

mod call;
mod facts;
mod label;

pub use call::{offered, PCI_DSM_UUID};
pub use facts::{host_facts, HostFacts, Readiness};
pub use label::{device_label, Label, MAX_LABEL};
