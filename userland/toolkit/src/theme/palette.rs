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

//! The one palette, in ARGB.
//!
//! Before this existed every application carried its own copy and they had
//! drifted: four different window backgrounds across five applications, and a
//! file manager whose accent was green while everything beside it was cyan. No
//! single screen looked wrong, and the desktop as a whole looked like a
//! collection of programs that had never met.
//!
//! These are the values the majority already agreed on, promoted to the one
//! place that defines them. An application that needs a colour takes it from
//! here. An application that needs a colour this does not have should add it
//! here rather than keep a private one, because a private colour is how the
//! drift started.
//!
//! The live theme store in `super::store` holds the same values and is what a
//! running system can change at runtime. This module is what everything
//! compiles against, so the two agree at rest.

/// The window ground. Near black with a blue cast rather than pure black, so
/// that raised surfaces have somewhere to be raised from.
pub const BACKGROUND: u32 = 0xFF0B_1319;

/// Panels, sidebars, and anything sitting on top of the ground.
pub const SURFACE: u32 = 0xFF13_1C24;

/// Headers and title bars: one step down from the ground rather than up, so
/// the chrome recedes and the content is what the eye lands on.
pub const HEADER: u32 = 0xFF08_111D;

/// The single accent. One colour, used for what the system says about itself:
/// selection, focus, the active tab, a link. Deliberately not green or red,
/// which carry meaning of their own.
pub const ACCENT: u32 = 0xFF35_C4E2;

/// Body text.
pub const TEXT: u32 = 0xFFE4_ECF5;

/// Secondary text: labels, captions, anything the reader scans past.
pub const MUTED: u32 = 0xFF9B_B0C7;

/// Text that is present but not available.
pub const DISABLED: u32 = 0xFF3C_4C60;

/// Hairlines and separators.
pub const BORDER: u32 = 0xFF23_3243;

/// Something completed. Kept apart from the accent so a reader who has learned
/// that green passed does not have to relearn it because green is also brand.
pub const OK: u32 = 0xFF56_D68B;

/// Something needs attention but nothing is lost.
pub const WARN: u32 = 0xFFE2_B341;

/// Something failed.
pub const ERROR: u32 = 0xFFE5_5C5C;
