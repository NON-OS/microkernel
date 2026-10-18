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

//! The line for a link change, logged only when BCR4.LEDOUT differs from the
//! last state logged, so a stack polling the link does not flood the log.

use core::sync::atomic::{AtomicU8, Ordering};

use crate::constants::csr::BCR4_LEDOUT;

use super::line::Line;

/// Last link state logged: 0 down, 1 up, 2 not yet logged.
static LINK_SEEN: AtomicU8 = AtomicU8::new(2);

pub fn link(bcr4: u16) {
    let up = bcr4 & BCR4_LEDOUT != 0;
    if LINK_SEEN.swap(up as u8, Ordering::Relaxed) == up as u8 {
        return;
    }
    Line::new(if up { b"driver.pcnet: link up" } else { b"driver.pcnet: link down" }).say();
}
