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

//! The VideoCore mailbox a Raspberry Pi's firmware answers on, and the bus
//! window its messages are addressed through.

use crate::arch::fdt::error::FdtError;
use crate::arch::fdt::node::DmaWindow;
use crate::arch::fdt::parser::Fdt;

/// The device tree binding's compatible for the BCM2835-family mailbox.
const NEEDLES: &[&[u8]] = &[b"brcm,bcm2835-mbox"];

/// The mailbox's registers in CPU terms, and where the VideoCore sees RAM:
/// a message names its buffer by bus address, which on a Pi 4 is not the
/// CPU's (the `/soc` bus puts RAM's first gigabyte at 0xc000_0000).
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct MailboxInfo {
    pub base: u64,
    pub size: u64,
    pub dma: DmaWindow,
}

/// The first enabled mailbox with registers and a DMA window. `None` on a
/// board without one, or when no bus maps its DMA: a buffer it cannot read
/// is no message at all.
pub fn find(fdt: &Fdt) -> Result<Option<MailboxInfo>, FdtError> {
    for found in fdt.devices(NEEDLES) {
        let (node, _) = found?;
        let Some((base, size)) = fdt.mmio(&node, 0)? else {
            continue;
        };
        let Some(dma) = fdt.dma_window(&node)? else {
            continue;
        };
        return Ok(Some(MailboxInfo { base, size, dma }));
    }
    Ok(None)
}
