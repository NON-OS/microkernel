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

//! Host stand-in for the kernel's side of mk_capsule_verify: a set status, and a record of each call.
//
// Replaces the MCVF syscall behind libc's real capsule_verify.rs, where the
// kernel checks the signatures, certificate and manifest. Nothing is checked
// here: call() copies the four sections it is pointed at and answers a set
// status, writing summary() through out at 0, as the kernel does on success.

use std::cell::{Cell, RefCell};

use crate::{CapsuleVerifyRequest, CapsuleVerifySummary};

pub const NAME: &[u8] = b"Demo Package";
pub const NS: &[u8] = b"org.nonos.demo_pkg";
pub const INSTALL_NAME: &[u8] = b"demo_pkg"; // NS's last label
pub const CAPS: u64 = 0x0000_0042_0000_0101;
pub const TIER: u8 = 2;

pub type Sections = [Vec<u8>; 4]; // elf, id_cert, manifest, zk_trailer

thread_local! {
    static CALLS: RefCell<Vec<Sections>> = const { RefCell::new(Vec::new()) };
    static RC: Cell<i64> = const { Cell::new(0) };
}

pub fn call(req: u64, out: u64) -> i64 {
    // SAFETY: mk_capsule_verify passes a live request, whose pointer and
    // length pairs name slices of the package held for this call.
    let r = unsafe { *(req as *const CapsuleVerifyRequest) };
    // SAFETY: as above.
    let copy = |p: u64, n: u32| unsafe { std::slice::from_raw_parts(p as *const u8, n as usize) };
    let s = [(r.elf_ptr, r.elf_len), (r.cert_ptr, r.cert_len)];
    let t = [(r.manifest_ptr, r.manifest_len), (r.trailer_ptr, r.trailer_len)];
    let [a, b, c, d] = [s[0], s[1], t[0], t[1]].map(|(p, n)| copy(p, n).to_vec());
    CALLS.with(|calls| calls.borrow_mut().push([a, b, c, d]));
    let rc = RC.with(Cell::get);
    if rc == 0 {
        // SAFETY: out is the live summary mk_capsule_verify was handed.
        unsafe { *(out as *mut CapsuleVerifySummary) = summary() };
    }
    rc
}

pub fn summary() -> CapsuleVerifySummary {
    let mut s = CapsuleVerifySummary::zeroed();
    (s.caps, s.tier, s.name_len, s.ns_len) = (CAPS, TIER, NAME.len() as u8, NS.len() as u8);
    s.name[..NAME.len()].copy_from_slice(NAME);
    s.namespace[..NS.len()].copy_from_slice(NS);
    s
}

/// Clears what was recorded and sets the status the next calls answer with.
pub fn reset(rc: i64) {
    CALLS.with(|c| c.borrow_mut().clear());
    RC.with(|r| r.set(rc));
}

pub fn calls() -> Vec<Sections> {
    CALLS.with(|c| std::mem::take(&mut *c.borrow_mut()))
}
