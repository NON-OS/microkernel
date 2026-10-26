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

//! The bring-up's one summary line, and the shootdown boot proof that runs
//! right after it when its dark feature is lit.

/// `started` APs answered inside their deadline; `online` CPUs counted
/// themselves in (ap_start.rs says why the two differ).
pub(super) fn answered(started: usize, online: usize) {
    let mut l = crate::sys::serial::Line::new();
    l.str(b"[SMP] answered=").dec(started as u64);
    l.str(b" online=").dec(online as u64);
    l.end();
    #[cfg(feature = "nonos-smp-shootdown-proof")]
    crate::memory::paging::manager::shootdown::run_proof();
}
