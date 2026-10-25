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

//! What a sink says it can do: the receiver capability fields at the
//! start of the DPCD (DisplayPort 1.4a section 2.9.3.1).

use super::aux::{Aux, AuxError};
use super::dpcd::{rate_mbps, DPCD_REV};
use super::native::read;

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct Caps {
    /// 0x10 for 1.0 up to 0x14 for 1.4.
    pub revision: u8,
    pub max_rate_mbps: u32,
    pub max_lanes: u32,
    pub tps3: bool,
    pub tps4: bool,
    pub enhanced_framing: bool,
    /// The channel equalisation read interval, in microseconds.
    pub eq_interval_us: u32,
}

impl Caps {
    /// Decode the first 16 DPCD bytes. A rate code the standard does not
    /// define, or a lane count other than 1, 2 or 4, is no usable sink.
    pub fn decode(b: &[u8; 16]) -> Option<Self> {
        let max_lanes = (b[2] & 0x1F) as u32;
        if !matches!(max_lanes, 1 | 2 | 4) {
            return None;
        }
        // 0x00E bits 6:0: 0 is 400 us, n from 1 to 4 is n x 4 ms.
        let eq_interval_us = match b[0x0E] & 0x7F {
            0 => 400,
            n @ 1..=4 => n as u32 * 4000,
            _ => return None,
        };
        Some(Caps {
            revision: b[0],
            max_rate_mbps: rate_mbps(b[1])?,
            max_lanes,
            tps3: b[2] & 0x40 != 0,
            tps4: b[3] & 0x80 != 0,
            enhanced_framing: b[2] & 0x80 != 0,
            eq_interval_us,
        })
    }

    pub fn read(aux: &mut dyn Aux) -> Result<Option<Self>, AuxError> {
        let mut b = [0u8; 16];
        read(aux, DPCD_REV, &mut b)?;
        Ok(Self::decode(&b))
    }
}
