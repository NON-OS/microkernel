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
use nonos_toolkit::design::scale::scale;

use super::rail_nav::head_h;
use super::rail_text::{gap, lh, pad, BAR_H};

/// The CPU trend's height, in logical pixels.
pub const SPARK_H: u32 = 30;

/// The heights the telemetry painters advance by, kept apart from the painters
/// themselves so the scroll offset and the composer read one set of numbers and
/// can never disagree about where a section ends.
pub fn sys_h() -> u32 {
    head_h() + lh() * 5 + scale().px(BAR_H) + gap() * 2 + scale().px(SPARK_H)
}

pub fn net_h() -> u32 {
    head_h() + lh() * 4
}

pub fn disk_h() -> u32 {
    head_h() + lh() * 2
}

/// The three sections stacked with one gap between each, plus the pad that
/// keeps the last row off the bottom edge of the rail.
pub fn telemetry_h() -> u32 {
    sys_h() + net_h() + disk_h() + gap() * 2 + pad()
}
