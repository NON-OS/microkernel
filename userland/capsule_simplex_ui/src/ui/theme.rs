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

//! The window's colours, from the theme the toolkit holds for this process,
//! which the skeleton keeps in step with the system's theme setting.

use nonos_toolkit::paint::mixer::mix;
use nonos_toolkit::theme::snapshot;

const OPAQUE: u32 = 0xFF00_0000;
/// The refusal red the toolkit's palette uses.
const FAIL: u32 = 0xFFC8_3A2B;

pub struct Colors {
    pub ground: u32,
    pub pane: u32,
    pub text: u32,
    pub muted: u32,
    pub quiet: u32,
    pub accent: u32,
    pub rule: u32,
    /// The fill of a bubble we sent.
    pub sent: u32,
    /// The fill of the selected row in the list.
    pub selected: u32,
    /// Text on an accent fill.
    pub on_accent: u32,
    pub fail: u32,
}

fn tint(base: u32, accent: u32, a: u32) -> u32 {
    OPAQUE | mix(base, accent, a)
}

pub fn colors() -> Colors {
    let t = snapshot();
    Colors {
        ground: t.background_argb,
        pane: t.surface_argb,
        text: t.text_argb,
        muted: t.muted_argb,
        quiet: t.quiet_argb,
        accent: t.accent_argb,
        rule: t.border_argb,
        sent: tint(t.surface_argb, t.accent_argb, 48),
        selected: tint(t.surface_argb, t.accent_argb, 28),
        on_accent: t.background_argb,
        fail: FAIL,
    }
}
