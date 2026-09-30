// NONOS Operating System
// Copyright (C) 2026 NONOS Contributors
// SPDX-License-Identifier: AGPL-3.0-or-later

//! Starting a tool by its service name, for boot and for `MkToolRun`.

use super::registry::embedded_tools;
use crate::sys::boot_log;

/// The name a terminal starts a Qwen tier by. It names no embedded tool: it
/// is the Linux personality, run as the caller's child on its terminal.
const QWEN_TOOL: &[u8] = b"tool.qwen";

/// Run the embedded tool whose service name matches `name`, parented to the
/// caller so it can drive the tool's stdin and stdout. `argv` is the NUL
/// separated argument blob. Returns the tool's pid, or `None`. Tools run on
/// demand, not at boot: a command-line tool has nothing to do until invoked.
/// `tool.qwen` is not an embedded tool: `argv` is then the tier word, or
/// `window` and the tier word. `tool.model-fetch` is the first-party model
/// fetcher. `run_for_caller` says why a refused run was refused.
pub fn run_named(name: &[u8], argv: &[u8]) -> Option<u32> {
    run_for_caller(name, argv).ok()
}

/// `run_named`, with the refusal as a negative errno: ENOENT for a name
/// nothing answers to, and for `tool.qwen` whatever
/// `capsule_linux::run_qwen_for_caller` refused with. A queued window run
/// is `Ok(0)`: init starts it later, so there is no pid to give.
pub fn run_for_caller(name: &[u8], argv: &[u8]) -> Result<u32, i64> {
    /* Qwen and its fetcher need the Linux personality, which setup may turn off. */
    if crate::userspace::init::app_tool_off(name) {
        return Err(crate::syscall::microkernel::errnos::ERRNO_ACCES);
    }
    if name == super::model_fetch::TOOL {
        return super::model_fetch::run_for_caller(argv);
    }
    if name == QWEN_TOOL {
        return crate::userspace::capsule_linux::run_qwen_for_caller(argv);
    }
    let Some(tool) = embedded_tools().into_iter().find(|t| t.name.as_bytes() == name) else {
        return Err(crate::syscall::microkernel::errnos::ERRNO_NOENT);
    };
    tool.spawn_with_args(argv).map_err(|_| {
        boot_log::error("tool capsule spawn failed");
        crate::syscall::microkernel::errnos::ERRNO_NOENT
    })
}
