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

//! IRQ grants a depart took (`acpi::depart`): a driver that polls or waits
//! on one is told ENODEV, its device is gone, rather than the "no such
//! grant" a mistaken id earns. The most recent `KEPT` are remembered; a
//! grant id is never asked about long after its device went.

extern crate alloc;

use alloc::collections::VecDeque;
use spin::Mutex;

static GONE: Mutex<VecDeque<(u32, u64)>> = Mutex::new(VecDeque::new());

/// Whether `grant_id` was `pid`'s and a depart took it.
pub fn departed(pid: u32, grant_id: u64) -> bool {
    GONE.lock().iter().any(|&(p, id)| p == pid && id == grant_id)
}

crate::arch::acpi_only! {
    #[path = "gone/mark.rs"]
    mod mark;
    pub(super) use mark::mark;
}
