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

//! Bus Master for the 8821C only once its power cycle shows the MAC off. A
//! driver that died without its Drop (a panic, a kill, an allocation failure)
//! can leave the card powered with its DMA engines armed at memory the
//! kernel freed. The claim cleared Bus Master, so nothing moves until it is
//! turned on again; it is turned on only after a cold card, or a warm one
//! whose power-off read REG_CR back as 0xEA (rtw88 gives up otherwise,
//! mac.c:394), was powered on (CARD_ENABLE) and answers its registers.

use super::{power_on, PowerOn, CR_UNPOWERED};
use crate::constants::regs::{mmio_dead, REG_SYS_CFG1};
use crate::regs::Mmio;

/// Why the chip may not master the bus.
#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub enum NoMaster {
    /// The power-on sequence never reached readiness.
    PowerFailed,
    /// A warm card's power-off did not take: REG_CR read this, not 0xEA.
    LeftOn { cr_after_off: u8 },
    /// The register window read back dead after power-on.
    DeadMmio,
    /// The broker refused the Bus Master write.
    Refused,
}

/// Power the MAC on (a warm one off first), see it answer, then `master_on`.
/// The chip-id readback and how the power-on went, or why not.
pub fn power_on_then_master<M: Mmio>(
    mmio: &M,
    master_on: impl FnOnce() -> bool,
) -> Result<(u32, PowerOn), NoMaster> {
    let p = power_on(mmio);
    match p {
        PowerOn::Failed => return Err(NoMaster::PowerFailed),
        PowerOn::WasLeftPowered { cr_after_off } if cr_after_off != CR_UNPOWERED => {
            return Err(NoMaster::LeftOn { cr_after_off });
        }
        PowerOn::WasLeftPowered { .. } | PowerOn::FromCold => {}
    }
    let id = mmio.read32(REG_SYS_CFG1);
    if mmio_dead(id) {
        return Err(NoMaster::DeadMmio);
    }
    if !master_on() {
        return Err(NoMaster::Refused);
    }
    Ok((id, p))
}
