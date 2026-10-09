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
//! A verdict in words, for the boot log a photo is read from.

use super::verdict::Verdict;

/// The words for wire value `code` of `OP_OUTPUT_STATUS`.
pub const fn name(code: u32) -> &'static str {
    match code {
        c if c == Verdict::Ready as u32 => "playing",
        c if c == Verdict::NeedsSof as u32 => "needs Intel SOF, the speakers are behind the DSP",
        c if c == Verdict::NoCodec as u32 => "no codec on the link",
        c if c == Verdict::HdmiOnly as u32 => "HDMI or DisplayPort only",
        c if c == Verdict::NoOutputPath as u32 => "no usable analog output",
        c if c == Verdict::AmdAcp as u32 => "needs the AMD ACP, the speakers are behind it",
        _ => "unknown",
    }
}
