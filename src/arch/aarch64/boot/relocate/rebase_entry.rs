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

use super::rebase::rebase;
use super::rela::{Placement, Rela};

/// The move to the high half's call: rewrite the pointers of the image linked
/// at `link_base` and loaded at `load_base`, `len` bytes, for `run_base`,
/// from the `.rela.dyn` between `start` and `end`; places before `writable`
/// bytes in are refused. Returns 0, or the `Refused` code, in which case
/// nothing was written and the kernel stays where it runs.
///
/// # Safety
///
/// Boot CPU, alone, interrupts masked, before the jump to `run_base`, which
/// must follow with no pointer read in between: the arguments are the
/// linker's symbols as the caller found them.
#[no_mangle]
unsafe extern "C" fn aarch64_rebase(
    start: *const Rela,
    end: *const Rela,
    link_base: u64,
    load_base: u64,
    len: u64,
    run_base: u64,
    writable: u64,
) -> u64 {
    let bytes = (end as u64).wrapping_sub(start as u64);
    let count = (bytes / core::mem::size_of::<Rela>() as u64) as usize;
    // SAFETY: the linker put `count` entries between the two symbols, in the
    // image's read-only data, which nothing writes.
    let relas = unsafe { core::slice::from_raw_parts(start, count) };
    let at = Placement { link_base, load_base, len };
    // SAFETY: the caller's contract above.
    match unsafe { rebase(relas, at, run_base, writable) } {
        Ok(_) => 0,
        Err(refused) => refused as u64,
    }
}
