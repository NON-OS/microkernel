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

//! Which tables a grant's holder runs in, pure (K10). Proven on the host
//! (kernel_proofs::holder_choice).

/// Where a holder's pages are.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub(crate) enum Space {
    /// An address space the paging manager knows, by asid.
    Asid(u32),
    /// Neither the pid nor its tables have an asid: the caller falls back
    /// to the tables it is running on, which are the holder's only when
    /// the holder itself is running.
    ActiveTable,
}

/// A process has its own asid. A thread, or an heir running on tables it
/// inherited, has none by pid; its pcb's cr3 names the tables, and they
/// have one. A zero cr3 names no tables. `cr3` is read only when the pid
/// has no asid.
pub(crate) fn choose(
    by_pid: Option<u32>,
    cr3: impl FnOnce() -> Option<u64>,
    by_cr3: impl FnOnce(u64) -> Option<u32>,
) -> Space {
    by_pid
        .or_else(|| cr3().filter(|&c| c != 0).and_then(by_cr3))
        .map_or(Space::ActiveTable, Space::Asid)
}
