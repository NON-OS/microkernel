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

//! Switches, sliders, and the states a control can be in.

use super::super::derive::{mix, opaque, with_alpha};
use super::super::store::snapshot;

/// A switch that is off. Raised off the surface it sits on, because a switch drawn
/// flat reads as a label.
pub fn switch_off_bg() -> u32 {
    let t = snapshot();
    opaque(mix(t.surface_argb, t.text_argb, 30))
}

pub fn switch_off_border() -> u32 {
    let t = snapshot();
    opaque(mix(t.surface_argb, t.text_argb, 46))
}

/*
 * The knob is the part that moves, so it has to be the lightest thing in the
 * control in both positions. Off it steps back towards the ground; on it is the
 * text colour, which in every scheme is the lightest value there is.
 */
pub fn knob_off() -> u32 {
    let t = snapshot();
    opaque(mix(t.text_argb, t.background_argb, 110))
}

pub fn knob_on() -> u32 {
    opaque(snapshot().text_argb)
}

/// A slider track: the part still to be filled.
pub fn track_bg() -> u32 {
    let t = snapshot();
    opaque(mix(t.surface_argb, t.text_argb, 38))
}

/// The wash under a hovered or selected row. Alpha rather than a colour, so it
/// works over a card and over the ground without two versions of it.
pub fn hover_wash() -> u32 {
    with_alpha(snapshot().text_argb, 0x14)
}

/// The wash behind something active, in the accent. Stronger than a hover,
/// because it marks where the user is rather than where the pointer is.
pub fn wash_strong() -> u32 {
    with_alpha(snapshot().accent_argb, 0x20)
}

/// The ring around whatever has the keyboard. Half opaque: it has to be visible
/// over a filled control without hiding what the control says.
pub fn focus_ring() -> u32 {
    with_alpha(snapshot().accent_argb, 0x80)
}
