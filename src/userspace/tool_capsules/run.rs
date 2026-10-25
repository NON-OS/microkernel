// NONOS Operating System
// Copyright (C) 2026 NONOS Contributors
// SPDX-License-Identifier: AGPL-3.0-or-later

//! Starting a tool by its service name, for boot and for `MkToolRun`.

use super::linux_terminal::spawn_errno;
use super::registry::{embedded_tools, LINUX_NET_TOOL, LINUX_TOOL};
use crate::sys::boot_log;
use crate::userspace::capsule_linux::{LINUX_CAPS, LINUX_MANIFEST_BYTES};
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
    if name == LINUX_TOOL || name == QWEN_TOOL {
        super::bound::check(LINUX_MANIFEST_BYTES, LINUX_CAPS, name)?;
    }
    if name == LINUX_TOOL {
        return super::linux_terminal::run(argv);
    }
    if name == LINUX_NET_TOOL {
        /* The role's own caps beyond LINUX_CAPS, which the caller must hold. */
        let net = crate::userspace::capsule_linux::TERMINAL_NET_EXTRA;
        super::bound::check(LINUX_MANIFEST_BYTES, LINUX_CAPS | net, name)?;
        return super::linux_terminal::run_net(argv);
    }
    if name == super::model_fetch::TOOL {
        super::bound::check(super::model_fetch::MANIFEST, super::model_fetch::CAPS, name)?;
        return super::model_fetch::run_for_caller(argv);
    }
    if name == QWEN_TOOL {
        return crate::userspace::capsule_linux::run_qwen_for_caller(argv);
    }
    let tool = embedded(name)?;
    super::bound::check(tool.manifest, tool.caps, name)?;
    spawn(&tool, argv)
}

/// The kernel's own run of an embedded tool, for init's boot self-test: the
/// kernel is the spawner, as for every boot spawn, so no caller's mask
/// bounds it. Never reached from `MkToolRun`, which takes `run_named`.
#[cfg(feature = "nonos-tool-selftest")]
pub fn run_embedded_for_kernel(name: &[u8], argv: &[u8]) -> Result<u32, i64> {
    spawn(&embedded(name)?, argv)
}

fn embedded(name: &[u8]) -> Result<super::spec::ToolCapsule, i64> {
    embedded_tools().into_iter().find(|t| t.name.as_bytes() == name).ok_or(ERRNO_NOENT)
}

fn spawn(tool: &super::spec::ToolCapsule, argv: &[u8]) -> Result<u32, i64> {
    tool.spawn_with_args(argv).map_err(|e| {
        boot_log::error("tool capsule spawn failed");
        spawn_errno(&e)
    })
}
