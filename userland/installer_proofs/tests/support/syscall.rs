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

//! Host stand-in for libc's syscall bridge, under the real capsule_load.rs.
//
// Replaces libc's syscall/bridge/call_raw.rs (the SYSCALL trampoline), with
// N_MK_CAPSULE_LOAD copied from numbers/core.rs over the real tag4. call_raw
// records the request it is pointed at and a copy of each blob it names
// (load_by_name frees its blobs on return), and answers with a test's rc.

#[path = "../../../libc/src/syscall/numbers/tag.rs"]
pub mod tag;

use std::cell::{Cell, RefCell};

use crate::CapsuleLoadRequest;

pub const N_MK_CAPSULE_LOAD: i64 = tag::tag4(b"MCLD");

pub struct Load {
    pub num: i64,
    pub req: CapsuleLoadRequest,
    pub blobs: [Vec<u8>; 5], // elf, cert, manifest, trailer, args
}

thread_local! {
    static LOADS: RefCell<Vec<Load>> = const { RefCell::new(Vec::new()) };
    static RC: Cell<i64> = const { Cell::new(0) };
}

pub fn call_raw(num: i64, args: [u64; 6]) -> i64 {
    // SAFETY: mk_capsule_load passes the address of a live request, whose
    // pointer and length pairs name slices that live for this call.
    let req = unsafe { *(args[0] as *const CapsuleLoadRequest) };
    let blobs = [
        copy(req.elf_ptr, req.elf_len),
        copy(req.cert_ptr, req.cert_len),
        copy(req.manifest_ptr, req.manifest_len),
        copy(req.trailer_ptr, req.trailer_len),
        copy(req.args_ptr, req.args_len),
    ];
    LOADS.with(|l| l.borrow_mut().push(Load { num, req, blobs }));
    RC.with(Cell::get)
}

fn copy(ptr: u64, len: u32) -> Vec<u8> {
    if len == 0 {
        return Vec::new();
    }
    // SAFETY: see call_raw; a non-zero length comes with a live pointer.
    unsafe { core::slice::from_raw_parts(ptr as *const u8, len as usize) }.to_vec()
}

/// Clears what was recorded and sets the rc the next loads answer with.
pub fn reset(rc: i64) {
    LOADS.with(|l| l.borrow_mut().clear());
    RC.with(|r| r.set(rc));
}

pub fn take() -> Vec<Load> {
    LOADS.with(|l| core::mem::take(&mut *l.borrow_mut()))
}
