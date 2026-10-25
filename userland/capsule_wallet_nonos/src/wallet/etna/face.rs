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

//! The two faces the brand uses and no third: Geist for prose in two
//! weights, JetBrains Mono for every value, label and button. Both are OFL,
//! see assets/fonts/Geist-OFL.txt and assets/fonts/JetBrainsMono-OFL.txt.

use nonos_toolkit::ttf::FontRef;
use spin::Once;

const GEIST: &[u8] = include_bytes!("../../../../assets/fonts/Geist-Regular.ttf");
const GEIST_MEDIUM: &[u8] = include_bytes!("../../../../assets/fonts/Geist-Medium.ttf");
const MONO: &[u8] = include_bytes!("../../../../assets/fonts/JetBrainsMono-Regular.ttf");

#[derive(Clone, Copy, PartialEq, Eq)]
pub enum Face {
    Sans,
    SansMedium,
    Mono,
}

static FACES: [Once<Option<FontRef<'static>>>; 3] = [Once::new(), Once::new(), Once::new()];

/// The parsed face, once. A face that does not parse draws nothing rather
/// than falling back to a font the design does not use.
pub fn font(face: Face) -> Option<&'static FontRef<'static>> {
    let (slot, bytes) = match face {
        Face::Sans => (0, GEIST),
        Face::SansMedium => (1, GEIST_MEDIUM),
        Face::Mono => (2, MONO),
    };
    FACES[slot].call_once(|| FontRef::try_from_slice(bytes).ok()).as_ref()
}
