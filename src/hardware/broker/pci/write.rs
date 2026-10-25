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

use crate::capabilities::Capability;
use crate::drivers::pci::config::ConfigSpace;
use crate::hardware::broker::claim;

use super::allowlist::validate;
use super::change::split;
use super::ident_read::ident;
use super::ownership::resolve;
use super::types::{PciWriteError, PciWriteRequest};

pub fn write(pid: u32, req: PciWriteRequest) -> Result<(), PciWriteError> {
    let (handle, _held) = resolve(pid, &req)?;
    let cfg = ConfigSpace::new(handle.address);
    let (register, _) = split(req.offset).ok_or(PciWriteError::OffsetNotAllowed)?;
    let current = cfg.read16(register).map_err(|_| PciWriteError::PlatformError)?;
    // The register, not the call's argument: a masked write's mask is above it.
    let ident = ident(&cfg, register as u32)?;
    // Bus Master is for DMA, Memory Space for MMIO: only a caller holding
    // Dma, or Mmio, may set each (K6, H-02).
    let has = |c: Capability| crate::process::caps::has(pid, c.bit());
    let may = (has(Capability::Dma), has(Capability::Mmio));
    let judge = |now| validate(&req, handle.msix.as_ref(), &ident, now, may);
    let action = judge(current)?;
    // Never after an exit moved the claim on and stopped the device (L69).
    let make = || super::apply::apply(&cfg, action, judge);
    let applied = claim::while_live(pid, req.device_id, req.claim_epoch, make);
    applied.unwrap_or(Err(PciWriteError::NotClaimed))
}
