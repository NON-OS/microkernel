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

use super::Scheme;

/*
 * The NØNOS schemes, from nonos.software/brand: Ink #0A0B0D is the dark ground,
 * Paper #F2F3F7 the light one, NØNOS Cyan #66FFFF the accent on Ink, and Cyan for
 * type the accent on Paper, where the brand cyan is 1.1:1 and is never text.
 * The brand gives Cyan for type as #007C85 at "4.5:1"; it is 4.49:1 on Paper,
 * under the AA floor it is meant to meet, so this is #007C82, the nearest that
 * clears it (4.50:1 on Paper, 4.99:1 on white), three steps of blue apart. Surfaces step one way from the ground and the border is a rule, not an
 * edge: the site's thin rules, which divide and do not box.
 *
 * theme_proofs holds every scheme to the WCAG floors through the derived roles,
 * these two with the rest.
 */
pub const INK: Scheme = Scheme {
    bg: 0xFF0A_0B0D,
    surface: 0xFF10_1215,
    accent: 0xFF66_FFFF,
    text: 0xFFF2_F3F7,
    border: 0xFF26_282B,
};

pub const PAPER: Scheme = Scheme {
    bg: 0xFFF2_F3F7,
    surface: 0xFFFF_FFFF,
    accent: 0xFF00_7C82,
    text: 0xFF0A_0B0D,
    border: 0xFFD5_D7DD,
};
