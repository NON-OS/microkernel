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

//! The M/N values a DisplayPort transcoder sends (DisplayPort 1.4a
//! section 2.2.5.2, Mvid/Nvid; SKL PRM Vol 2c, PIPE_DATA_M1, PIPE_DATA_N1,
//! PIPE_LINK_M1, PIPE_LINK_N1). Data M/N is the share of the link the
//! pixels fill; link M/N is the pixel clock over the link symbol clock.
//! With 8b/10b a lane's symbol clock is its bit rate over ten.

use super::access::Mmio;
use super::error::Error;
use super::pipe::Transcoder;
use super::regs::trans_base;

/// One N for both pairs: large enough for precision, small enough that
/// M fits 24 bits for any ratio up to 32.
pub const N: u32 = 0x8_0000;
/// PIPE_DATA_M1 TU Size, 30:25, stored less one: 64.
pub const TU_64: u32 = 63 << 25;

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct Mn {
    pub data_m: u32,
    pub link_m: u32,
}

/// For `pixel_khz` at `bpp` bits per pixel on `lanes` lanes of `mbps`.
pub fn compute(pixel_khz: u32, bpp: u32, lanes: u32, mbps: u32) -> Result<Mn, Error> {
    let symbol_khz = mbps as u64 * 100;
    if pixel_khz == 0 || bpp == 0 || lanes == 0 || symbol_khz == 0 {
        return Err(Error::Argument);
    }
    let data_num = pixel_khz as u64 * bpp as u64;
    let data_den = symbol_khz * lanes as u64 * 8;
    if data_num > data_den {
        return Err(Error::Argument);
    }
    let data_m = (data_num * N as u64 + data_den / 2) / data_den;
    let link_m = (pixel_khz as u64 * N as u64 + symbol_khz / 2) / symbol_khz;
    if link_m >= 1 << 24 {
        return Err(Error::Argument);
    }
    Ok(Mn { data_m: data_m as u32, link_m: link_m as u32 })
}

pub fn write(m: &mut dyn Mmio, t: Transcoder, mn: Mn) {
    let b = trans_base(t);
    m.write(b + 0x30, TU_64 | mn.data_m);
    m.write(b + 0x34, N);
    m.write(b + 0x40, mn.link_m);
    m.write(b + 0x44, N);
}
