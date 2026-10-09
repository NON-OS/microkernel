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

/*
 * An install boot whose setup chose Install: the installer is the whole
 * screen and nothing else starts. It ends in a restart from the disk it
 * wrote, or with the person leaving it; only then does the desktop start
 * here, all of it, so a person who leaves without installing still has a
 * working machine. An installer that did not start counts as ended, so the
 * desktop comes up and the refusal is on the serial log.
 */

use core::sync::atomic::{AtomicBool, Ordering};

use crate::userspace::capsule_install::install_running;

static HANDED_OVER: AtomicBool = AtomicBool::new(false);

pub(super) fn hand_over() {
    let _ = super::super::spawn_plan::spawn_installer_first();
    HANDED_OVER.store(true, Ordering::SeqCst);
}

pub(super) fn handed_over() -> bool {
    HANDED_OVER.load(Ordering::SeqCst)
}

pub(super) fn poll() -> bool {
    if install_running() {
        return false;
    }
    super::super::spawn_plan::spawn_post_wizard();
    crate::sys::serial::print(b"[INIT] installer ended without a restart; starting the desktop\n");
    true
}
