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

//! The largest TMDS rate an HDMI sink accepts, from the blocks that carry
//! it: the HDMI 1.x vendor-specific block (HDMI 1.4b section 8.3.2, byte
//! 7: Max_TMDS_Clock), the HDMI Forum one and the HF-SCDB (HDMI 2.1
//! section 10.3.2, byte 5: Max_TMDS_Character_Rate). Each is in 5 MHz
//! steps; in the Forum blocks 0 means nothing above 340 MHz.

use super::blocks::{Cta, TAG_EXTENDED};
use crate::edid::Edid;

/// Data block tag 3, vendor-specific (CTA-861-H section 7.5.4).
pub const TAG_VENDOR: u8 = 3;
/// The IEEE OUIs, which a vendor block stores least significant first.
pub const OUI_HDMI: u32 = 0x00_0C03;
pub const OUI_HDMI_FORUM: u32 = 0xC4_5DD8;
/// Extended tag 0x79, the HDMI Forum sink capability data block.
pub const EXT_HF_SCDB: u8 = 0x79;

fn oui(body: &[u8]) -> Option<u32> {
    let o = body.get(..3)?;
    Some(o[0] as u32 | (o[1] as u32) << 8 | (o[2] as u32) << 16)
}

impl Edid<'_> {
    /// In kHz; 0 when no block names it, which leaves a driver to the
    /// DVI and HDMI 1.0 default of 165 MHz.
    pub fn max_tmds_khz(&self) -> u32 {
        let mut max = 0u32;
        for c in self.extensions().filter_map(Cta::new) {
            for b in c.data_blocks() {
                // Offsets below count from the byte after the header
                // (and after the extended tag in the HF-SCDB).
                let units = match (b.tag, b.extended, oui(b.body)) {
                    (TAG_VENDOR, None, Some(OUI_HDMI)) => b.body.get(6),
                    (TAG_VENDOR, None, Some(OUI_HDMI_FORUM)) => b.body.get(4),
                    (TAG_EXTENDED, Some(EXT_HF_SCDB), _) => b.body.get(3),
                    _ => None,
                };
                max = max.max(units.copied().unwrap_or(0) as u32 * 5_000);
            }
        }
        max
    }
}
