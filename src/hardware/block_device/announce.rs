// NØNOS Operating System
// Copyright (C) 2026 NØNOS Contributors
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

//! Saying, once, which disk the block layer settled on.

use alloc::format;

use super::backend::Backend;

pub(super) fn announce(b: Backend) -> Backend {
    let line = format!("[BLOCK] NONOS disk on {}", (b.disk.announced)(b.key));
    crate::sys::serial::println(line.as_bytes());
    crate::log::info!("{}", line);
    b
}
