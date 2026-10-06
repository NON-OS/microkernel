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

use nonos_brand::palette as p;

/* The NØNOS palette (nonos_brand), the same as the installer and the loader. */
pub const BACKDROP: u32 = p::GROUND;
pub const ACCENT: u32 = p::CYAN;
pub const FG: u32 = p::TEXT;
pub const HINT: u32 = p::TEXT_3;
pub const ROW_BORDER: u32 = p::RULE;
pub const ROW_SEL_BG: u32 = p::CYAN_SOFT;
pub const SUB: u32 = p::TEXT_2;
pub const RULE: u32 = p::RULE;

pub const STEP_LABELS: &[&[u8]] = &[
    b"Keyboard",
    b"Your name",
    b"Time zone",
    b"Mode",
    b"Network",
    b"Network route",
    b"Privacy",
    b"Appearance",
    b"Qwen model",
    b"Apps",
    b"Installed software",
    b"Computer name",
    b"Review",
];
