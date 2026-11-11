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

use nonos_toolkit::design::size::PILL_H;
use nonos_toolkit::widgets::Pill;

use super::layout::px;

// Every chip is the shared Pill, sized here from its label, so a chip row and
// any per-row chip lay out to the widths the pills are drawn at.
const CHIP_GAP: u32 = 8;

/// A chip's height at the display scale in force, the pill's own.
pub fn chip_h() -> u32 {
    px(PILL_H)
}

/// The gap after a chip at the display scale in force.
pub fn chip_gap() -> u32 {
    px(CHIP_GAP)
}

/// The pill's own width, without the gap that follows it. Measured off the
/// pill rather than a surface, so the chip hit-test -- which has no
/// PaintBuffer to ask -- sizes every pill exactly as the painter did.
pub fn chip_w(label: &str) -> u32 {
    Pill::new(label).size().0
}
