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

//! Whether a link carries a timing at a colour depth.
//!
//! DisplayPort: each lane's bit rate, less the channel coding: 8b/10b up
//! to HBR3 (8.1 Gbps; DisplayPort 1.4a section 3.5), 128b/132b for the
//! UHBR rates from 10 Gbps (DisplayPort 2.0 section 3.5). Main stream
//! attribute packets and FEC take a little more; the margin is the
//! driver's. HDMI and DVI: the TMDS character rate is the pixel clock
//! scaled by bits per pixel over 24 (HDMI 1.4b section 6.5.2, deep colour)
//! and must not pass the sink's limit.

/// The link a mode is checked against.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Link {
    /// `lanes` of `mbps` each, as trained.
    Dp { lanes: u32, mbps: u32 },
    /// The largest TMDS character rate the sink and source take, in kHz.
    Tmds { max_khz: u32 },
}

/// The link carries `pixel_khz` at `bpp` bits per pixel.
pub fn carries(link: Link, pixel_khz: u32, bpp: u32) -> bool {
    let need = pixel_khz as u64 * bpp as u64;
    match link {
        Link::Dp { lanes, mbps } => {
            let raw = lanes as u64 * mbps as u64 * 1000;
            let payload = if mbps <= 8100 { raw * 8 / 10 } else { raw * 128 / 132 };
            need <= payload
        }
        Link::Tmds { max_khz } => bpp >= 24 && need / 24 <= max_khz as u64,
    }
}
