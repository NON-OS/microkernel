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

use super::apply::apply;
use super::rela::{Placement, Rela};

/// The entry code's call: relocate the image linked at `link_base` and loaded
/// at `load_base`, `len` bytes long, from the `.rela.dyn` between `start` and
/// `end`. Returns 0, or the `Refused` code the entry code stops on.
///
/// # Safety
///
/// Boot CPU, MMU off, before any other Rust; the arguments are the linker's
/// symbols as the entry code found them.
#[no_mangle]
unsafe extern "C" fn aarch64_relocate(
    start: *const Rela,
    end: *const Rela,
    link_base: u64,
    load_base: u64,
    len: u64,
) -> u64 {
    let bytes = (end as u64).wrapping_sub(start as u64);
    let count = (bytes / core::mem::size_of::<Rela>() as u64) as usize;
    // SAFETY: the linker put `count` entries between the two symbols, in the
    // image's read-only data, which nothing writes.
    let relas = unsafe { core::slice::from_raw_parts(start, count) };
    let at = Placement { link_base, load_base, len };
    // SAFETY: the caller is the boot CPU before anything reads the image.
    match unsafe { apply(relas, at) } {
        Ok(_) => 0,
        Err(refused) => refused as u64,
    }
}
