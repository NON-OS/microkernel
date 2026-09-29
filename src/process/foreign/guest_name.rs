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

//! A guest's name in the process table follows the program it runs, as a
//! Linux task's comm does: the last part of argv[0], at most 15 bytes of
//! printable ASCII, behind a "foreign:" the guest cannot remove.

use alloc::string::String;
use alloc::vec::Vec;

use crate::memory::addr::{PhysAddr, VirtAddr};
use crate::memory::paging::manager::translate_in_asid;

use super::peer_guard::{in_user_half, PAGE};

/// Linux keeps this many bytes of a task's name.
const COMM: usize = 15;
/// How far into argv[0] its terminator is looked for.
const LOOK: u64 = 256;

/// The name argv[0] gives on a new stack at `rsp`, or None when the stack,
/// the pointer or the string cannot be read or names nothing printable.
/// The caller holds the peer lock, so no page read here can be unmapped.
pub(super) fn from_stack(asid: u32, rsp: u64) -> Option<String> {
    let at = rsp.checked_add(8)?;
    let ptr = u64::from_le_bytes(read(asid, at, 8, false)?.try_into().ok()?);
    let raw = read(asid, ptr, LOOK, true)?;
    let path = &raw[..raw.iter().position(|b| *b == 0)?];
    let base = path.rsplit(|b| *b == b'/').next()?;
    let comm = &base[..base.len().min(COMM)];
    if comm.is_empty() || !comm.iter().all(|b| (0x21..0x7f).contains(b)) {
        return None;
    }
    Some(alloc::format!("foreign:{}", core::str::from_utf8(comm).ok()?))
}

/// Up to `len` bytes of the guest at `va`, only from its own half, stopping
/// after the page that holds a terminator when `string` asks for one.
fn read(asid: u32, va: u64, len: u64, string: bool) -> Option<Vec<u8>> {
    let mut out = Vec::new();
    let mut at = va;
    while (out.len() as u64) < len {
        let page = at & !(PAGE - 1);
        let take = core::cmp::min(PAGE - (at - page), len - out.len() as u64);
        if !in_user_half(at, take) {
            return None;
        }
        let phys = translate_in_asid(asid, VirtAddr::new(page))?;
        let virt = crate::memory::unified::phys_to_virt(PhysAddr::new(phys.as_u64() + at - page))?;
        // SAFETY: eK@nonos.systems - `phys` came from the guest's own page
        // tables for an address in its own half, and `take` stays inside
        // that one frame, which the held peer lock keeps mapped.
        out.extend_from_slice(unsafe {
            core::slice::from_raw_parts(virt.as_u64() as *const u8, take as usize)
        });
        if string && out.contains(&0) {
            break;
        }
        at += take;
    }
    Some(out)
}
