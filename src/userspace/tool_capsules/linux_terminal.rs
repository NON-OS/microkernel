// NONOS Operating System
// Copyright (C) 2026 NONOS Contributors
// SPDX-License-Identifier: AGPL-3.0-or-later

//! `tool.linux`: the terminal's `linux` command runs the Linux personality
//! rather than an embedded tool.

extern crate alloc;

use crate::kernel_core::process_spawn::capsule_spawn::SpawnError;
use crate::syscall::microkernel::errnos::{ERRNO_ACCES, ERRNO_EXIST};

/// The Linux personality, for the terminal's `linux` command. The program it
/// runs is read from the store and must carry its own proof, so naming one
/// grants nothing an unproven binary could use.
#[cfg(feature = "nonos-capsule-linux")]
pub(super) fn run(argv: &[u8]) -> Result<u32, i64> {
    /* Every argument as typed, an empty one included. */
    let argv = argv
        .split(|&b| b == 0)
        .map(|s| alloc::string::String::from_utf8_lossy(s).into_owned())
        .collect();
    crate::userspace::capsule_linux::spawn_terminal(argv).map_err(|e| {
        let line = alloc::format!("linux terminal spawn failed: {e:?}");
        crate::sys::boot_log::error(&line);
        spawn_errno(&e)
    })
}

/// A build without the personality has no `linux` to run.
#[cfg(not(feature = "nonos-capsule-linux"))]
pub(super) fn run(_argv: &[u8]) -> Result<u32, i64> {
    Err(crate::syscall::microkernel::errnos::ERRNO_NOENT)
}

/// The personality in its terminal role with Network, for a `--box` that
/// names destinations, and only for one: any other argv is EINVAL before
/// anything is spawned. The personality holds the run to the box's list.
#[cfg(feature = "nonos-capsule-linux")]
pub(super) fn run_net(argv: &[u8]) -> Result<u32, i64> {
    if !super::net_box::names_destinations(argv) {
        crate::sys::serial::print(b"[LINUX-TERM-NET] refused: no box names a destination\n");
        return Err(crate::syscall::microkernel::errnos::ERRNO_INVAL);
    }
    let argv = argv
        .split(|&b| b == 0)
        .map(|s| alloc::string::String::from_utf8_lossy(s).into_owned())
        .collect();
    crate::userspace::capsule_linux::spawn_terminal_net(argv).map_err(|e| spawn_errno(&e))
}

#[cfg(not(feature = "nonos-capsule-linux"))]
pub(super) fn run_net(_argv: &[u8]) -> Result<u32, i64> {
    Err(crate::syscall::microkernel::errnos::ERRNO_NOENT)
}

/// The errno a tool's refused spawn is reported by, as a capsule load reports
/// it: a live instance holding the role's endpoints is EEXIST, which the
/// terminal says is one already running, and any other refusal is EACCES.
pub(super) fn spawn_errno(e: &SpawnError) -> i64 {
    match e {
        SpawnError::EndpointCollision => ERRNO_EXIST,
        _ => ERRNO_ACCES,
    }
}
