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

//! The codec's own reset, then what playback needs of it: the DAC, the
//! analog mixer and Vref ready, and the front DAC at the link's 48 kHz.

use crate::constants::mixer::{
    EXT_CTRL, EXT_VRA, PD_PLAYBACK_READY, POWERDOWN, RESET, VENDOR_ID1, VENDOR_ID2,
};
use crate::regs::Mixer;
use crate::wait::until;

/// Linux's ac97 layer gives the codec half a second to power up.
const POWER_READY_MS: u64 = 500;

/// The codec's AC'97 vendor id, in its two halves (QEMU's SigmaTel
/// STAC9700 is 8384:7600).
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct Ident {
    pub vendor: u16,
    pub device: u16,
}

pub fn reset(m: &Mixer) -> Result<Ident, &'static str> {
    m.write(RESET, 0);
    if !until(POWER_READY_MS, || m.read(POWERDOWN) & PD_PLAYBACK_READY == PD_PLAYBACK_READY) {
        return Err("codec DAC and analog section never came ready");
    }
    let id = Ident { vendor: m.read(VENDOR_ID1), device: m.read(VENDOR_ID2) };
    // A window the bridge does not decode reads all ones, ready bits too.
    if id.vendor == 0xFFFF && id.device == 0xFFFF {
        return Err("codec registers read all ones");
    }
    // Variable rate off: AC'97 2.3 then holds the DAC at the link's 48 kHz
    // whatever rate the firmware left. A codec older than variable rate has
    // neither register and plays at 48 kHz already.
    m.write(EXT_CTRL, m.read(EXT_CTRL) & !EXT_VRA);
    Ok(id)
}
