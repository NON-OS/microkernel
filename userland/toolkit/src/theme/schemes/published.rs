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

//! Three schemes that exist outside this project, at their published values.

use super::Scheme;

/*
 * A theme offered under a name people already know has to be that theme. Someone
 * who picks Nord knows what Nord looks like, and approximating it from memory is
 * how a desktop ends up with a palette that is recognisably nobody's.
 *
 * So these carry the values their authors published, role for role, and the role
 * each swatch was chosen for is named beside it.
 */

/// Nord, by Arctic Ice Studio (nordtheme.com): nord0 as the ground, nord1 for
/// raised surfaces, nord8 as the frost accent, nord4 as snow-storm text, nord3
pub const NORD: Scheme = Scheme {
    bg: 0xFF2E_3440,
    surface: 0xFF3B_4252,
    accent: 0xFF88_C0D0,
    text: 0xFFD8_DEE9,
    border: 0xFF4C_566A,
};

/// Dracula, by Zeno Rocha (draculatheme.com), at its specified values:
/// Background, Current Line, Purple, Foreground, and Comment for the hairlines.
pub const DRACULA: Scheme = Scheme {
    bg: 0xFF28_2A36,
    surface: 0xFF44_475A,
    accent: 0xFFBD_93F9,
    text: 0xFFF8_F8F2,
    border: 0xFF62_72A4,
};

/*
 * Solarized dark, by Ethan Schoonover. The published body text for the dark mode
 * is base0 (#839496), which sits at about 4.1:1 on base03 and so misses the 4.5:1
 * the proofs hold every scheme to. Schoonover specifies base1 (#93A1A1) as the
 * emphasised foreground in the same mode, so the emphasised value is what this
 * takes: still his palette, and legible to someone who needs it to be.
 */
pub const SOLARIZED_DARK: Scheme = Scheme {
    bg: 0xFF00_2B36,
    surface: 0xFF07_3642,
    accent: 0xFF26_8BD2,
    text: 0xFF93_A1A1,
    border: 0xFF58_6E75,
};
