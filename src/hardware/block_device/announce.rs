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
use super::gpt::Places;

pub(super) fn announce(b: Backend, p: &Places) -> Backend {
    let (store, plan) = (p.store.first, p.plan.first);
    let disk = (b.disk.announced)(b.key);
    let line = format!("[BLOCK] NONOS disk on {disk}: store at LBA {store}, plan at LBA {plan}");
    crate::sys::serial::println(line.as_bytes());
    crate::log::info!("{}", line);
    b
}
