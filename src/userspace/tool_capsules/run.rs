// NONOS Operating System
// Copyright (C) 2026 NONOS Contributors
// SPDX-License-Identifier: AGPL-3.0-or-later

//! Starting a tool by its service name, for boot and for `MkToolRun`.

use super::linux_terminal::spawn_errno;
use super::registry::{embedded_tools, LINUX_TOOL};
use crate::sys::boot_log;
use crate::syscall::microkernel::errnos::{ERRNO_ACCES, ERRNO_NOENT};

/// The name a terminal starts a Qwen tier by. It names no embedded tool: it
/// is the Linux personality, run as the caller's child on its terminal.
const QWEN_TOOL: &[u8] = b"tool.qwen";

/// Run the embedded tool whose service name matches `name`, parented to the
/// caller so it can drive the tool's stdin and stdout. `argv` is the NUL
/// separated argument blob. Returns the tool's pid, or why not as a negative
/// errno: ENOENT for a name nothing answers to, EEXIST for a tool already
/// running, EACCES for one setup turned off. Tools run on demand, not at
/// boot: a command-line tool has nothing to do until invoked.
///
/// Three names are not embedded tools. `tool.linux` is the personality the
/// terminal's `linux` command runs. `tool.qwen` takes the tier word, or
/// `window` and the tier word, and is refused with whatever
/// `capsule_linux::run_qwen_for_caller` refused with; a queued window run is
/// `Ok(0)`, since init starts it later and there is no pid to give.
/// `tool.model-fetch` is the first-party model fetcher.
pub fn run_named(name: &[u8], argv: &[u8]) -> Result<u32, i64> {
    /* Each of the three needs the Linux personality, which setup may turn off. */
    if crate::userspace::init::app_tool_off(name) {
        return Err(ERRNO_ACCES);
    }
    if name == LINUX_TOOL {
        return super::linux_terminal::run(argv);
    }
    if name == super::model_fetch::TOOL {
        return super::model_fetch::run_for_caller(argv);
    }
    if name == QWEN_TOOL {
        return crate::userspace::capsule_linux::run_qwen_for_caller(argv);
    }
    let Some(tool) = embedded_tools().into_iter().find(|t| t.name.as_bytes() == name) else {
        return Err(ERRNO_NOENT);
    };
    tool.spawn_with_args(argv).map_err(|e| {
        boot_log::error("tool capsule spawn failed");
        spawn_errno(&e)
    })
}
