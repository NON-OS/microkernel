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

//! The surfaces things are drawn on.

use super::super::derive::{mix, opaque};
use super::super::store::snapshot;

/// The window ground.
pub fn window_bg() -> u32 {
    opaque(snapshot().background_argb)
}

/// A sidebar or rail: a shade below the ground, so the content beside it reads as
/// the thing in front.
pub fn sidebar_bg() -> u32 {
    let t = snapshot();
    opaque(mix(t.background_argb, 0xFF00_0000, 26))
}

/// A title bar. One step down from the ground rather than up, so the chrome
/// recedes and the content is where the eye lands.
pub fn header_bg() -> u32 {
    let t = snapshot();
    opaque(mix(t.background_argb, 0xFF00_0000, 40))
}

/// A card, panel or anything raised off the ground.
pub fn card_bg() -> u32 {
    opaque(snapshot().surface_argb)
}

/*
 * A pill sits on a card, so it cannot be the card's own colour and cannot be the
 * ground either: between the two is the only place it reads as a separate shape
 * without a border doing the work.
 */
pub fn pill_bg() -> u32 {
    let t = snapshot();
    opaque(mix(t.background_argb, t.surface_argb, 128))
}

/*
 * A strip raised off a card: a table's column headings, a toolbar inside a panel.
 * It cannot be the card's colour or the heading row disappears into the rows, and it
 * cannot be the ground or the card looks like it has a hole in it.
 */
pub fn raised() -> u32 {
    let t = snapshot();
    opaque(mix(t.surface_argb, t.text_argb, 12))
}

/// Zebra banding for long tables: the faintest step off the surface that still
/// reads as a band, so a forty row table is followable without stripes shouting.
pub fn band() -> u32 {
    let t = snapshot();
    opaque(mix(t.surface_argb, t.text_argb, 6))
}
