// NONOS Operating System
// Copyright (C) 2026 NONOS Contributors
// SPDX-License-Identifier: AGPL-3.0-or-later

//! `tool.linux`: the terminal's `linux` command runs the Linux personality
//! rather than an embedded tool.

extern crate alloc;

/// The Linux personality, for the terminal's `linux` command. The program it
/// runs is read from the store and must carry its own proof, so naming one
/// grants nothing an unproven binary could use.
#[cfg(feature = "nonos-capsule-linux")]
pub(super) fn run(argv: &[u8]) -> Option<u32> {
    let argv = argv
        .split(|&b| b == 0)
        .filter(|s| !s.is_empty())
        .map(|s| alloc::string::String::from_utf8_lossy(s).into_owned())
        .collect();
    match crate::userspace::capsule_linux::spawn_terminal(argv) {
        Ok(pid) => Some(pid),
        Err(e) => {
            let line = alloc::format!("linux terminal spawn failed: {e:?}");
            crate::sys::boot_log::error(&line);
            None
        }
    }
}

/// A build without the personality has no `linux` to run.
#[cfg(not(feature = "nonos-capsule-linux"))]
pub(super) fn run(_argv: &[u8]) -> Option<u32> {
    None
}
