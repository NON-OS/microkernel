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

//! `MkIrqAck`: a capsule finished with an interrupt and the line may fire
//! again.

use super::super::grant::IrqGrantKind;
use super::super::records;
use super::super::types::IrqError;
use crate::arch::interrupt::ioapic;

pub fn ack_grant(pid: u32, grant_id: u64) -> Result<(), IrqError> {
    let g = records::lookup(grant_id).ok_or(IrqError::UnknownGrant)?;
    if g.pid != pid {
        return Err(IrqError::NotHolder);
    }
    match g.kind {
        IrqGrantKind::Intx => {
            let _ = ioapic::mask(g.irq_source, false);
        }
        IrqGrantKind::Msix | IrqGrantKind::Msi => {
            // MSI and MSI-X have no per-line IO-APIC mask. The dispatcher
            // leaves the per-vector mask alone, so an ack is a
            // no-op on the hardware side; capsules still call it
            // for symmetry with the INTx path.
        }
    }
    Ok(())
}
