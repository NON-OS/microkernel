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

//! Whether a page can be mapped write-combining. x86_64 programs a PAT entry
//! for it on every CPU (arch/x86_64/pat); the other architectures have no such
//! entry here yet, so a write-combining request falls back to uncached.

#[inline]
pub fn write_combining_ready() -> bool {
    #[cfg(target_arch = "x86_64")]
    {
        crate::arch::x86_64::pat::wc_ready()
    }
    #[cfg(not(target_arch = "x86_64"))]
    {
        false
    }
}
