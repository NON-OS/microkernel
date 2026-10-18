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

use super::projects;
use super::types::Prefs;

#[path = "codec_decode.rs"]
mod codec_decode;
pub use codec_decode::decode;

pub const MAGIC: [u8; 4] = *b"NTP1";
pub const VERSION: u16 = 2;
pub const HEAD: usize = 12;
pub const LEN: usize = HEAD + projects::BYTES;

pub fn encode(p: &Prefs) -> [u8; LEN] {
    let mut out = [0u8; LEN];
    out[0] = MAGIC[0];
    out[1] = MAGIC[1];
    out[2] = MAGIC[2];
    out[3] = MAGIC[3];
    let ver = VERSION.to_le_bytes();
    out[4] = ver[0];
    out[5] = ver[1];
    let theme = p.theme.to_le_bytes();
    out[6] = theme[0];
    out[7] = theme[1];
    out[8] = p.font_scale;
    out[9] = p.cursor;
    out[10] = p.rails;
    out[11] = 0;
    projects::encode_into(p, &mut out[HEAD..]);
    out
}
