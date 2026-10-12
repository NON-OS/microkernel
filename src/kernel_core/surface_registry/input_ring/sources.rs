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

//! The kernel's table of input posters (source.rs says what a source is).

use spin::Mutex;

use super::source::{index_of, SLOTS};

static POSTERS: Mutex<[u32; SLOTS]> = Mutex::new([0; SLOTS]);

/// The source a post by `pid` carries: its process's index, by thread group.
pub(super) fn source_of(pid: u32) -> u8 {
    let group = crate::process::with_process(pid, |p| p.thread_group_id())
        .filter(|&g| g != 0)
        .unwrap_or(pid);
    let alive = |p: u32| crate::process::with_process(p, |_| ()).is_some();
    index_of(&mut POSTERS.lock(), group, alive)
}
