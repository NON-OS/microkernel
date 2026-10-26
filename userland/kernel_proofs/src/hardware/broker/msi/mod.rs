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

//! The MSI side table's rules (src/hardware/broker/pci_index/msi), gated here
//! as in the kernel, and its proofs.

crate::arch::msi_only! {
    #[path = "../../../../../../src/hardware/broker/pci_index/msi/set.rs"]
    pub mod set;
    #[path = "../../../../../../src/hardware/broker/pci_index/msi/table.rs"]
    pub mod table;
}

#[cfg(test)]
mod glue_tests;
#[cfg(test)]
mod tests;
