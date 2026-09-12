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

//! The type ramp and the spacing unit.
//!
//! The screens grew twenty-five distinct text sizes, chosen one at a time, which
//! is why nothing lined up. This ramp was written to end that, and then did not,
//! because three of its six steps were below the toolkit's readable floor of
//! seventeen pixels: `LABEL` at 12.1, `SMALL` at 13.8 and `BODY` at 14.9 all
//! rendered at exactly seventeen. A hundred and twenty-five pieces of text asked
//! for twenty-five sizes and got thirteen, and the screens laid out around them
//! were spaced for small text that was never going to appear.
//!
//! So no step is below the floor now. The bottom three still coincide there,
//! because a caption genuinely cannot be smaller than body text on this display,
//! and pretending otherwise is what produced the flat screens in the first
//! place. They are kept as separate names because they mean different things and
//! are told apart by colour: `DIM` and `MUTED` for what labels, `FG` for what
//! reads. Two things of one size in two tones is a hierarchy; two things
//! claiming sizes the font will not grant is not.
//!
//! The upper steps are where size can carry the difference, so they are spaced
//! far enough apart to be read as different at a glance rather than as a
//! mistake.

/// The size below which the toolkit stops listening.
///
/// `nonos_toolkit::font::ttf::MIN_UI_PX` clamps anything under this up to it, so
/// a smaller number is not a smaller size, it is the same size written
/// misleadingly. Mirrored here rather than imported so the ramp can be proved on
/// the host without the whole toolkit; `scale_tests` asserts every step against
/// it, which is the check that was missing when three steps of this very file
/// sat underneath it.
pub const FLOOR: f32 = 17.0;

/// Micro labels, set in capitals: "TOTAL BALANCE", "LOCK TERM". At the floor,
/// so what separates it from body text is capitals and a dimmer tone.
pub const LABEL: f32 = FLOOR;
/// Secondary text: hints, availability, the quiet half of a row. The floor
/// again, told apart by `MUTED`.
pub const SMALL: f32 = FLOOR;
/// Default body text and control labels.
pub const BODY: f32 = FLOOR;
/// Figures inside a card, and anything the reader is meant to read first.
pub const VALUE: f32 = 21.0;
/// Section headings.
pub const TITLE: f32 = 26.0;
/// The one number a screen exists to show.
pub const HERO: f32 = 34.0;
/// The wordmark on the opening screen, which is the only thing on it.
pub const SPLASH: f32 = 46.0;

/// The spacing unit. Every gap, inset and margin is a multiple of this, so
/// vertical rhythm holds without each screen inventing its own.
pub const UNIT: u32 = 4;

/// `n` spacing units.
pub const fn space(n: u32) -> u32 {
    UNIT * n
}
