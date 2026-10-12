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

//! QEMU's tablet report as the driver reads it through its descriptor.

use super::layout;
use super::vectors::QEMU_TABLET;
use crate::hid::report::pointer::{pointer_report, Motion};
use crate::hid::tablet::TabletReport;

pub fn tablet(raw: &[u8]) -> Option<TabletReport> {
    match pointer_report(&layout(QEMU_TABLET).pointer?, raw)? {
        Motion::Absolute(t) => Some(t),
        Motion::Relative(_) => None,
    }
}

pub fn at(x: u16, y: u16) -> [u8; 6] {
    let [xl, xh] = x.to_le_bytes();
    let [yl, yh] = y.to_le_bytes();
    [0, xl, xh, yl, yh, 0]
}
