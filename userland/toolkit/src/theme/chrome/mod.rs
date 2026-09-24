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

//! Every colour the desktop draws, derived from the five live theme roles.

mod control;
mod ground;
mod ink;
mod lines;

pub use control::{
    focus_ring, hover_wash, knob_off, knob_on, switch_off_bg, switch_off_border, track_bg,
    wash_strong,
};
pub use ground::{band, card_bg, header_bg, pill_bg, raised, sidebar_bg, window_bg};
pub use ink::{accent, ink, muted, nav_active_edge, nav_active_ink, quiet, value_ink};
pub use lines::{hairline, hairline_soft};

/*
 * Why derived rather than listed.
 *
 * The panel used to carry twenty nine constants, and the desktop as a whole carried
 * some six hundred across forty files. Every one of them was a decision made once
 * against one palette, which is why the theme setting could be offered for years
 * and change nothing: honouring it meant editing forty files, so nobody did.
 *
 * These read the live store instead, and each is one step from a role: a hairline
 * is the border, a muted label is the text moved towards the ground, a hover is the
 * text at low alpha. A theme then has five values to get right rather than six
 * hundred, and a new theme is five numbers rather than an audit.
 *
 * What is deliberately not here: the semantic colours. Success, warning and failure
 * mean something, and a reader who has learned that red failed should not have to
 * relearn it because they changed theme. Those stay in `palette`.
 */
