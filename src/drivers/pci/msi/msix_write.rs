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

//! Writing a prepared message into one MSI-X table entry. The vector control
//! word is left as it is: the bind keeps the function masked while it writes
//! and unmasks the entry afterwards (PCIe 6.0, 6.1.4.5; Linux
//! __pci_write_msi_msg writes MSI-X address and data the same way).

use super::super::error::Result;
use super::super::types::{MsiMessage, MsixInfo, PciBar};
use super::msix_entry::map_msix_table_entry;

pub fn write_msix_message(
    msix: &MsixInfo,
    bars: &[PciBar; 6],
    vector: u16,
    msg: MsiMessage,
) -> Result<()> {
    let entry = map_msix_table_entry(msix, bars, vector)?;
    crate::memory::mmio::mmio_w32(entry.addr, msg.address as u32);
    crate::memory::mmio::mmio_w32(entry.addr + 4u64, (msg.address >> 32) as u32);
    crate::memory::mmio::mmio_w32(entry.addr + 8u64, msg.data);
    Ok(())
}
