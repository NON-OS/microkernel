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

//! Assemble the firmware DRAM image map the gen3 boot ROM reads from peripheral
//! scratch. The runtime image is a flat section list split by two separators
//! into LMAC, UMAC and paged images, the way `iwl_pcie_init_fw_sec` does it in
//! Linux iwlwifi. `classify` is the pure partition; `stage` copies each section
//! into the DMA region and reports each chunk's page-aligned device address.

// The single-region staging and its helpers are built by the proofs only
// (see `gen3`): the capsule lays firmware out across grants with `layout`.
#[cfg(test)]
mod align;
mod classify;
mod markers;
#[cfg(test)]
mod placement;
#[cfg(test)]
mod stage;
mod types;

pub use classify::classify;
#[cfg(test)]
pub use placement::DramPlacement;
#[cfg(test)]
pub use stage::stage;
pub use types::FwLayout;
