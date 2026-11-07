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

//! `MkAppLaunch`: start the program a distribution package installed.

use crate::services::registry::{caller_group, caller_owns, serves::LISTINGS};
use crate::syscall::microkernel::errnos::{ERRNO_BUSY, ERRNO_INVAL, ERRNO_PERM};
use crate::userspace::capsule_linux::note_launch;

use super::app_install::id;

#[path = "app_launch_rule.rs"]
mod rule;
use rule::{admit_launch, Launch};
#[path = "app_launch_take.rs"]
mod take;
#[path = "app_launch_word.rs"]
mod word;

/// `MkAppLaunch(listing_ptr, listing_len, word_ptr, word_len)`. Queues the
/// run for init; whether the program may start is the exec gate's answer,
/// which checks the trailer the machine minted when it installed the
/// package. EPERM for a capsule SERVES lists asking for a listing not its
/// own (app_launch_rule.rs); EINVAL for a word it may not give (take.rs).
pub fn sys_app_launch(listing_ptr: u64, listing_len: u64, word_ptr: u64, word_len: u64) -> i64 {
    let listing = match id(listing_ptr, listing_len) {
        Ok(Some(s)) => s,
        Ok(None) => return ERRNO_INVAL,
        Err(e) => return e,
    };
    let Some(name) =
        listing.strip_prefix("linux.").and_then(crate::userspace::capsule_linux::package_arg)
    else {
        return ERRNO_INVAL;
    };
    let free = crate::syscall::caps::current_caps_or_default().can_app_install()
        || caller_owns("desktop_shell");
    let admitted = admit_launch(free, caller_owns, &listing, LISTINGS);
    if admitted == Launch::Refused {
        return ERRNO_PERM;
    }
    let word = match take::taken(free, &listing, word_ptr, word_len) {
        Ok(word) => word,
        Err(e) => return e,
    };
    if crate::userspace::capsule_linux::run_busy() || crate::userspace::init::run_waiting() {
        return ERRNO_BUSY;
    }
    // Noted before it is queued, so init cannot start it unnoted.
    note_launch(if free { None } else { caller_group() }, &name, word.as_deref());
    // A served launch starts its SERVES-paired core. When that core's image
    // is not placed in this session, init installs the paired listing first,
    // in its release's box, and the launch is answered busy until the core is
    // placed and its run queued; the window asks again on a busy answer. The
    // install is init's own (request_served_run), for this one listing
    // admit_launch just paired, so the window never holds AppInstall. A free
    // launch runs what is already placed, as before.
    let queued = match admitted {
        Launch::Served => crate::userspace::init::request_served_run(listing, name),
        _ => crate::userspace::init::request_run(name),
    };
    match queued {
        true => 0,
        false => {
            note_launch(None, "", None);
            ERRNO_BUSY
        }
    }
}
