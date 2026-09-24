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

//! Text, at the weights the desktop reads it in.

use super::super::derive::{mix, opaque, with_alpha};
use super::super::store::snapshot;

/// Body text, titles, anything meant to be read.
pub fn ink() -> u32 {
    opaque(snapshot().text_argb)
}

/*
 * A value beside its label is not secondary: it is the answer to the row, and the
 * reason the row is on screen. It steps back from the label by a little so the two
 * read as label and value rather than one sentence, and no further.
 */
pub fn value_ink() -> u32 {
    let t = snapshot();
    opaque(mix(t.text_argb, t.background_argb, 36))
}

/*
 * Secondary and placeholder text come from the store already derived. They are not
 * a fixed step from the text colour: how far they can move depends on how much
 * contrast the scheme has to give, which `legible` works out once per theme.
 */
pub fn muted() -> u32 {
    opaque(snapshot().muted_argb)
}

pub fn quiet() -> u32 {
    opaque(snapshot().quiet_argb)
}

/// The single accent: selection, focus, the active tab, a link.
pub fn accent() -> u32 {
    opaque(snapshot().accent_argb)
}

/*
 * The label of the tab you are on. Lifted from the accent towards the text colour,
 * because the raw accent over its own wash is not legible in every scheme.
 *
 * Here rather than in each capsule: the settings panel, About and the process list
 * all draw the same sidebar, and three copies of this would be three chances for one
 * of them to end up with an unreadable active tab in one theme.
 */
pub fn nav_active_ink() -> u32 {
    let t = snapshot();
    mix(opaque(t.accent_argb), t.text_argb, 96)
}

/// The edge of the active tab's wash: stronger than the wash, weaker than a line,
/// so the tab reads as raised rather than boxed. Beside `nav_active_ink` because the
pub fn nav_active_edge() -> u32 {
    with_alpha(snapshot().accent_argb, 0x59)
}
