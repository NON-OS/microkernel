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

//! `MkPciConfigWrite` orchestration.

use crate::drivers::pci::config::ConfigSpace;
use crate::drivers::pci::constants::CFG_COMMAND;

use super::allowlist::validate;
use super::ownership::resolve;
use super::quirk_bits::Ident;
use super::types::{PciWriteError, PciWriteRequest, WriteAction};

pub fn write(pid: u32, req: PciWriteRequest) -> Result<(), PciWriteError> {
    let handle = resolve(pid, &req)?;
    let cfg = ConfigSpace::new(handle.address);
    let current = cfg.read16(req.offset as u16).map_err(|_| PciWriteError::PlatformError)?;
    let ident = ident(&cfg)?;
    let action = validate(&req, handle.msix.as_ref(), &ident, current)?;
    apply(&cfg, action)
}

fn apply(cfg: &ConfigSpace, action: WriteAction) -> Result<(), PciWriteError> {
    match action {
        WriteAction::Command(value) => {
            cfg.write16(CFG_COMMAND, value).map_err(|_| PciWriteError::PlatformError)
        }
        WriteAction::MsixControl { offset, value } | WriteAction::Bits { offset, value } => {
            cfg.write16(offset, value).map_err(|_| PciWriteError::PlatformError)
        }
    }
}

/// The function's vendor, class and PCI Express capability, read from its
/// own config space at the time of the write.
fn ident(cfg: &ConfigSpace) -> Result<Ident, PciWriteError> {
    let vendor = cfg.read16(0x00).map_err(|_| PciWriteError::PlatformError)?;
    let class = cfg.read32(0x08).map_err(|_| PciWriteError::PlatformError)?;
    /*
     * The capability walk reads segment 0. A function behind a VMD lives in
     * a domain of its own, so the same bus, device and function there name
     * another device: none of its capability bits are offered.
     */
    let at = cfg.address();
    let pcie_cap = (at.segment == 0)
        .then(|| {
            crate::drivers::pci::capabilities::find_capability(
                at.bus,
                at.device,
                at.function,
                crate::drivers::pci::constants::CAP_ID_PCIE,
            )
        })
        .flatten()
        .map(|c| c.offset as u16);
    Ok(Ident { vendor, class: (class >> 24) as u8, subclass: (class >> 16) as u8, pcie_cap })
}
