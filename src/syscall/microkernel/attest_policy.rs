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

//! `MkAttestPolicy`: the roots, depths and epochs the gates check against.
//!
//! A new call rather than fields on `MkAttestStatus`, whose reply is a
//! `#[repr(C)]` struct written whole: a capsule built against its size would be
//! overrun by a longer one. This reply is a versioned byte record, refused into
//! a buffer shorter than the record.

use super::errnos::{ERRNO_FAULT, ERRNO_NOMEM};
use crate::security::attest_policy::{encode, Tree, RECORD_LEN};

pub fn sys_attest_policy(out_ptr: u64, out_len: u64) -> i64 {
    if out_len < RECORD_LEN as u64 {
        return ERRNO_NOMEM;
    }
    let Some(h) = crate::boot::handoff::get_handoff() else {
        return ERRNO_FAULT;
    };
    let p = h.policy;
    let kernel = (p.checked != 0).then_some(Tree {
        root: p.kernel_root,
        epoch: p.boot_epoch,
        depth: p.depth,
    });
    let capsule = crate::security::capsule_attest::published();
    let record = encode(kernel.as_ref(), capsule.as_ref());
    if crate::usercopy::write_user_bytes(out_ptr, &record).is_err() {
        return ERRNO_FAULT;
    }
    RECORD_LEN as i64
}
