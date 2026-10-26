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
//! The faces the desktop draws in: the brand's Geist for text, its Medium
//! for the bold weight, JetBrains Mono for data, and Noto Sans for the
//! wider Latin, Greek and the rest Geist has no glyph for. All SIL OFL 1.1;
//! the notices are beside them in assets/fonts.

use ab_glyph::FontRef;
use spin::Once;

const BODY: &[u8] = include_bytes!("../../../assets/fonts/Geist-Regular.ttf");
const BOLD: &[u8] = include_bytes!("../../../assets/fonts/Geist-Medium.ttf");
const MONO: &[u8] = include_bytes!("../../../assets/fonts/JetBrainsMono-Regular.ttf");
const WIDE: &[u8] = include_bytes!("../../../assets/fonts/NotoSans-Regular.ttf");

static BODY_FACE: Once<Option<FontRef<'static>>> = Once::new();
static BOLD_FACE: Once<Option<FontRef<'static>>> = Once::new();
static MONO_FACE: Once<Option<FontRef<'static>>> = Once::new();
static WIDE_FACE: Once<Option<FontRef<'static>>> = Once::new();

pub(super) fn face(mono: bool) -> Option<&'static FontRef<'static>> {
    if mono {
        MONO_FACE.call_once(|| FontRef::try_from_slice(MONO).ok()).as_ref()
    } else {
        BODY_FACE.call_once(|| FontRef::try_from_slice(BODY).ok()).as_ref()
    }
}

/// The face a character neither Geist nor JetBrains Mono has is drawn in.
pub(super) fn wide() -> Option<&'static FontRef<'static>> {
    WIDE_FACE.call_once(|| FontRef::try_from_slice(WIDE).ok()).as_ref()
}

pub fn builtin_face(mono: bool, bold: bool) -> Option<&'static FontRef<'static>> {
    if !mono && bold {
        return BOLD_FACE.call_once(|| FontRef::try_from_slice(BOLD).ok()).as_ref();
    }
    face(mono)
}
