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

//! Each ED's, TD pair's and buffer's place in the layout.

use super::layout::*;
use crate::facts::ed::ED_LEN;
use crate::facts::td::TD_LEN;

pub const fn intr_ed(i: usize) -> usize {
    INTR_ED + ED_LEN * i
}
pub const fn intr_td(i: usize, half: u8) -> usize {
    INTR_TD + TD_LEN * (2 * i + half as usize)
}
pub const fn intr_buf(i: usize) -> usize {
    INTR_BUF + INTR_BUF_LEN * i
}
pub const fn ctrl_td(half: u8) -> usize {
    CTRL_TD + CTRL_HALF * half as usize
}
/// Bulk ED `i`: slot `i / 2 + 1`, IN when `i` is even.
pub const fn bulk_ed(i: usize) -> usize {
    BULK_ED + ED_LEN * i
}
pub const fn bulk_td(i: usize, half: u8) -> usize {
    BULK_TD + TD_LEN * (2 * i + half as usize)
}
/// The polled bulk IN buffer of the device on root port `lane + 1`.
pub const fn poll_buf(lane: usize) -> usize {
    POLL_BUF + BULK_MAX * lane
}
