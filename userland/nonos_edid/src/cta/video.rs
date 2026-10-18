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

//! Short video descriptors, one byte each in the video data block, tag 2
//! (CTA-861-H section 7.5.1).

/// The data block tag of the video data block.
pub const TAG_VIDEO: u8 = 2;

/// One SVD as (VIC, native). Codes 1-127 are the VIC itself; 129-192
/// are VICs 1-64 marked native, the layout of the older revisions kept
/// for those VICs; 193-253 are VICs again with no native bit. 0, 128,
/// 254 and 255 are reserved and name nothing.
pub fn svd(b: u8) -> Option<(u8, bool)> {
    match b {
        1..=127 => Some((b, false)),
        129..=192 => Some((b - 128, true)),
        193..=253 => Some((b, false)),
        _ => None,
    }
}
