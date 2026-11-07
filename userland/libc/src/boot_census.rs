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

use crate::syscall::{call_raw, N_MK_BOOT_CENSUS};

/// The boot census the kernel last said (`MkBootCensus`): its lines,
/// `[CENSUS] <class> <up|absent|missing|held> expected=<n> up=<n>[ why=..]`
/// then `[CENSUS] summary ...`, as many whole lines as fit in `out`. Read
/// them with `nonos_hwreport::census::parse`. Needs AttestRead. Returns the
/// bytes written, 0 before the census is first said, or a negative errno.
pub fn mk_boot_census(out: &mut [u8]) -> i64 {
    call_raw(N_MK_BOOT_CENSUS, [out.as_mut_ptr() as u64, out.len() as u64, 0, 0, 0, 0])
}
