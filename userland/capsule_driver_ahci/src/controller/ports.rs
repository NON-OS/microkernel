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

//! Which ports a controller has and which hold a device, from CAP, PI and
//! PxSSTS alone. Pure, so the host proofs hold the decode.

use crate::constants::regs::{
    CAP_CPD, CAP_NP_MASK, CMD_POD, CMD_SUD, IPM_ACTIVE, IPM_DEVSLEEP, IPM_PARTIAL, IPM_SLUMBER,
    SSTS_IPM_MASK, SSTS_IPM_SHIFT,
};
use crate::engine::link::established::link_established;

/// The ports the walk treats as implemented. PI is the authority (AHCI 1.3.1,
/// 3.1.4), and its bits may be sparse: a PCH that fuses off ports leaves
/// holes, and the highest implemented port can sit above CAP.NP. A PI of
/// zero is a firmware that never wrote it; Linux then takes the first
/// CAP.NP + 1 ports, and so does this.
pub const fn effective_pi(cap: u32, pi: u32) -> u32 {
    if pi != 0 {
        return pi;
    }
    let np = (cap & CAP_NP_MASK) + 1;
    if np >= 32 {
        u32::MAX
    } else {
        (1u32 << np) - 1
    }
}

/// How many port slots, from port 0, a walk over the controller covers: up
/// to CAP.NP + 1, and up to the highest bit of PI when that lies above it.
/// A walk bounded by CAP.NP alone never reached a sparse PI's high ports.
pub const fn port_count(cap: u32, pi: u32) -> u8 {
    let np = (cap & CAP_NP_MASK) + 1;
    let top = 32 - effective_pi(cap, pi).leading_zeros();
    if top > np {
        top as u8
    } else {
        np as u8
    }
}

/// A port holds a device that is talking: DET = 3, and the interface is in
/// a power state the HBA reports for a live link. Partial, Slumber and
/// DevSleep are such states (a drive the firmware left in link power
/// management reads them), not an absent device.
pub fn device_present(ssts: u32) -> bool {
    let ipm = (ssts >> SSTS_IPM_SHIFT) & SSTS_IPM_MASK;
    link_established(ssts) && matches!(ipm, IPM_ACTIVE | IPM_PARTIAL | IPM_SLUMBER | IPM_DEVSLEEP)
}

/// The PxCMD bits that power a port's link up: SUD always (read-only one on
/// an HBA without staggered spin-up, so harmless there), POD only when CAP.CPD
/// makes it writable.
pub const fn spin_up_bits(cap: u32) -> u32 {
    if cap & CAP_CPD != 0 {
        CMD_SUD | CMD_POD
    } else {
        CMD_SUD
    }
}
