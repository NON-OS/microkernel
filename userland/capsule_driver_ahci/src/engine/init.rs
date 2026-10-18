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

use super::port::Port;
use crate::constants::regs::{PORT_BASE, PORT_SIG, PORT_STRIDE};
use crate::constants::timing::DEVICE_READY_MS;
use crate::error::{AhciError, AhciResult};
use crate::regs::Regs;

/// Bring up port `index` of the HBA whose CAP reads `cap`, and keep it if it
/// holds an ATA disk that IDENTIFY describes as one the driver serves. Every
/// implemented port is tried: COMRESET decides whether something is there
/// (`NoDisk` when not), not a link state read before the port was spun up.
pub fn init_port(
    device_id: u64,
    claim_epoch: u64,
    regs: Regs,
    cap: u32,
    index: u8,
) -> AhciResult<Port> {
    let base = PORT_BASE + index as u32 * PORT_STRIDE;
    /*
     * Stop whatever the firmware left running first: while it runs, the HBA
     * owns the firmware's command list and FIS region, and nothing of ours
     * is programmed yet.
     */
    super::stop::stop(regs, base)?;
    let mut port = Port::map(device_id, claim_epoch, base, cap)?;
    super::program::program(regs, &port);
    /*
     * program() set FRE with the FIS region's address; from here every
     * failure parks the port, which stops it before the regions go, so the
     * HBA never DMAs into freed memory.
     */
    if let Err(e) = super::link::link_up(regs, base, DEVICE_READY_MS) {
        super::park::park(port, regs);
        return Err(e);
    }
    /*
     * With the link up and the device ready, its D2H FIS has arrived and
     * PxSIG names the device. Only an ATA disk is served: an ATAPI drive
     * answers IDENTIFY PACKET DEVICE, and the other kinds no IDENTIFY at all.
     */
    // SAFETY: the walk reaches only ports whose registers lie inside the
    // mapped ABAR.
    if !super::link::is_ata_disk(unsafe { regs.r32(base + PORT_SIG) }) {
        super::park::park(port, regs);
        return Err(AhciError::NoDisk);
    }
    if let Err(e) = super::start::start(regs, base) {
        super::park::park(port, regs);
        return Err(e);
    }
    if let Err(e) = super::identify::identify(&mut port, regs) {
        super::park::park(port, regs);
        return Err(e);
    }
    Ok(port)
}
