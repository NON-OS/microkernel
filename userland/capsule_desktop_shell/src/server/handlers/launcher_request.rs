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

use core::sync::atomic::{AtomicI64, Ordering};

use nonos_libc::{mk_ipc_send_to_pid, mk_pid_alive, mk_service_lookup, mk_spawn_instance};

use crate::state::apps::LauncherApp;
use crate::state::says::{NOTHING_REFUSED, NOT_ASKED};
use crate::state::{raise_tracked, reach_by, Context, Reach};

/// What a dock-launch click actually did, so the caller can surface it on
/// screen. `Queued` means the kernel accepted a new-window spawn; `Focused`
/// means the slot table was full and the running window was raised instead.
#[derive(Clone, Copy, PartialEq, Eq)]
pub enum LaunchOutcome {
    Queued,
    Focused,
    Failed,
}

const CONTROL_LEN: usize = 8;
const CONTROL_MAGIC: u32 = u32::from_le_bytes(*b"NCTL");
const CONTROL_VERSION: u16 = 1;
const OP_FOCUS_SELF: u16 = 1;

// Services that run as one window: a second would fight the first over the
// same session rather than give the user anything new. The App Store holds
// the catalog and every install in flight, and its capsule declares no
// instance endpoints, so a spawn could only fail and say so.
const SINGLE_INSTANCE: [&[u8]; 1] = [b"app.store"];

// Clicking a dock app asks the kernel to spawn another attested instance.
// The kernel queues the request and init performs the spawn in its own
// context, because doing it inline in this shell's syscall corrupted the
// caller. A click with a free instance slot returns ok and a new window
// appears a tick later; when every declared slot is live the kernel returns
// an error and we focus the running instance instead, so a click is never a
// dead end. Single-instance services skip the spawn and always focus.
pub fn request(app: &LauncherApp) -> LaunchOutcome {
    request_service(app.service)
}

/// Launch, or focus if already running, whatever capsule owns `service`. Used
/// by both the dock (a desktop app) and the Launchpad (an installed tool).
pub fn request_service(service: &[u8]) -> LaunchOutcome {
    let rc = if is_single_instance(service) { NOT_ASKED } else { mk_spawn_instance(service) };
    if rc >= 0 {
        return LaunchOutcome::Queued;
    }
    let outcome = focus_service(service);
    /* Kept only for the failure it explains, which says it next. */
    REFUSED.store(
        if outcome == LaunchOutcome::Failed { rc } else { NOTHING_REFUSED },
        Ordering::Relaxed,
    );
    outcome
}

/// Why the last launch spawned nothing, taken once: the kernel's answer to
/// the spawn it refused, `NOT_ASKED` for an app that opens one window only,
/// or `NOTHING_REFUSED` when no spawn was asked since (a menu raising a
/// window that turned out gone). What a launch that opened nothing says
/// (`state::says::not_opened`), never an answer left from an earlier click.
pub fn take_refusal() -> i64 {
    REFUSED.swap(NOTHING_REFUSED, Ordering::Relaxed)
}

static REFUSED: AtomicI64 = AtomicI64::new(NOTHING_REFUSED);

/// Focus whatever already owns `service`, spawning nothing.
///
/// The dock uses this when it knows the app is running. Going through
/// `request_service` spawned a fresh window on every click and left a
/// minimized one hidden, which made it unreachable: this message is the only
/// thing that reaches `wm::window_restore`.
///
/// Any live instance will do: with the first window closed, the base name
/// is gone from the registry while "app.terminal.2" still holds a window.
pub fn focus_service(service: &[u8]) -> LaunchOutcome {
    if super::instances::each_live_pid(service, focus_pid) {
        LaunchOutcome::Focused
    } else {
        LaunchOutcome::Failed
    }
}

/// Raise app `index`'s newest window the window manager said is open,
/// restoring it if minimised. A window whose process is gone is forgotten
/// on the way (state/taskbar/route.rs). Failed when the app has no window
/// left: the caller opens one, rather than handing the click to an instance
/// with no window to show.
pub fn focus_app(ctx: &mut Context, index: usize) -> LaunchOutcome {
    match raise_tracked(&mut ctx.taskbar, index, reach_pid) {
        Some(_) => LaunchOutcome::Focused,
        None => LaunchOutcome::Failed,
    }
}

/// Send `pid` the focus frame, and say whether its process took it, is
/// busy, or is gone. A process that has ended is gone, though the kernel
/// would still take a frame into its inbox (state/taskbar/route.rs).
fn reach_pid(pid: u32) -> Reach {
    reach_by(pid, |p| mk_pid_alive(p), send_focus)
}

/// Send `pid` the focus frame; false when it is gone or busy.
pub(crate) fn focus_pid(pid: u32) -> bool {
    reach_pid(pid) == Reach::Taken
}

fn send_focus(pid: u32) -> i64 {
    let frame = focus_frame();
    mk_ipc_send_to_pid(pid, frame.as_ptr(), frame.len())
}

pub fn is_single_instance(service: &[u8]) -> bool {
    SINGLE_INSTANCE.iter().any(|s| *s == service)
}

pub(crate) fn lookup_pid(service: &[u8]) -> Option<u32> {
    let mut port = 0u32;
    let mut pid = 0u32;
    let rc = mk_service_lookup(service.as_ptr(), service.len(), &mut port, &mut pid);
    if rc < 0 || pid == 0 {
        return None;
    }
    Some(pid)
}

pub(crate) fn focus_frame() -> [u8; CONTROL_LEN] {
    let mut frame = [0u8; CONTROL_LEN];
    frame[0..4].copy_from_slice(&CONTROL_MAGIC.to_le_bytes());
    frame[4..6].copy_from_slice(&CONTROL_VERSION.to_le_bytes());
    frame[6..8].copy_from_slice(&OP_FOCUS_SELF.to_le_bytes());
    frame
}
