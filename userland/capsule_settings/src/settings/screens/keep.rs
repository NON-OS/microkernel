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

//! What a change on the displays' rows is kept as, so the shell puts it back
//! at the next start (its server/display_replay.rs): the arrangement as
//! DisplayLayout, a display's scale as DisplayScaleN. Named by id, as the
//! shell reads them; the store refuses them until it keeps them.

use super::arrange::{ScreenPage, OUTPUTS, SCALES};
use crate::settings::schema::access_knob::Knob;

/// DisplayLayout (0 mirror, 1 extend), then DisplayScale1 to 4, U8 each.
pub const DISPLAY_LAYOUT: u32 = 0x0126;
pub const DISPLAY_SCALE_1: u32 = 0x0127;

/// The field and value `knob`'s change on `page` is kept as; None for a row
/// that keeps nothing, or a scale the compositor would not take.
pub fn kept(page: &ScreenPage, knob: Knob) -> Option<(u32, u8)> {
    match knob {
        Knob::Arrange => Some((DISPLAY_LAYOUT, page.shown.extend as u8)),
        Knob::Scale => {
            let o = page.output.min(OUTPUTS - 1);
            let q = page.shown.chosen[o as usize];
            SCALES.contains(&q).then_some((DISPLAY_SCALE_1 + o, q as u8))
        }
        _ => None,
    }
}
