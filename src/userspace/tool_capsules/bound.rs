// NONOS Operating System
// Copyright (C) 2026 NONOS Contributors
// SPDX-License-Identifier: AGPL-3.0-or-later

//! A tool never holds more than the process that runs it, unless that process
//! is a launcher: it holds SpawnWindow, or Admin, which stands in for it. The
//! rule `MkCapsuleLoad` applies (`from_vfs/load/bound.rs`), over what the
//! tool's signed manifest would install for the caps its spawn asks. A run
//! outside the bound is refused with one serial line, never narrowed.

use crate::kernel_core::process_spawn::capsule_spawn::{beyond_parent, child_caps};
use crate::security::capsule_manifest::decode;
use crate::syscall::microkernel::errnos::ERRNO_PERM;

/// Refuses running `name` when what it would hold exceeds its caller. A tool
/// not built in (no manifest) is left to its spawn, which refuses it.
pub(super) fn check(manifest: &[u8], requested: u64, name: &[u8]) -> Result<(), i64> {
    if manifest.is_empty() {
        return Ok(());
    }
    let child = child_caps(&decode(manifest).map_err(|_| ERRNO_PERM)?, requested);
    let caller = crate::process::current_pid().ok_or(ERRNO_PERM)?;
    let held = crate::process::caps::bits(caller).ok_or(ERRNO_PERM)?;
    let gap = beyond_parent(child, held);
    if gap == 0 {
        return Ok(());
    }
    let name = core::str::from_utf8(name).unwrap_or("?");
    let line =
        alloc::format!("[CAP-DENY] tool: caller pid={caller} tool={name} missing caps {gap:#x}");
    crate::sys::serial::println(line.as_bytes());
    Err(ERRNO_PERM)
}
