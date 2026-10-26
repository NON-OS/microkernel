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
//! A process paints with the store's starting theme until it hears the
//! system's. That theme is the default, Ink, and its derived pair is what
//! `replace` would compute for it, so a first frame is the frame every
//! later one is.

use crate::theme::schemes::{scheme, SCHEMES};
use crate::theme::store::{Theme, START, START_MUTED, START_QUIET};

#[test]
fn the_store_starts_on_the_default_scheme_ink() {
    assert!(START == SCHEMES[0] && START == scheme(0), "stored index 0");
    assert_eq!((START.bg, START.accent, START.text), (0xFF0A_0B0D, 0xFF66_FFFF, 0xFFF2_F3F7));
}

#[test]
fn the_starting_secondary_text_is_what_replace_derives() {
    let t = Theme::from_roles(START.bg, START.surface, START.accent, START.text, START.border);
    assert_eq!((t.muted_argb, t.quiet_argb), (START_MUTED, START_QUIET));
}
