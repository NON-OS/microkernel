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
 * An install boot whose setup chose Install: the installer is queued before
 * any app. The market and the apps start once it has come and gone, so a
 * person who closes it without installing is left with a working desktop.
 * Polled before init drains its spawn queue, so a queued installer is never
 * mistaken for one that ended; one whose spawn was refused counts as ended,
 * so the desktop still comes up and the refusal is on the serial log.
 */

use core::sync::atomic::{AtomicBool, Ordering};

use crate::userspace::capsule_install::install_running;
use crate::userspace::init::instance_spawns_pending;

static HANDED_OVER: AtomicBool = AtomicBool::new(false);

pub(super) fn hand_over() {
    super::super::spawn_plan::spawn_installer_first();
    HANDED_OVER.store(true, Ordering::SeqCst);
}

pub(super) fn handed_over() -> bool {
    HANDED_OVER.load(Ordering::SeqCst)
}

pub(super) fn poll() -> bool {
    if instance_spawns_pending() || install_running() {
        return false;
    }
    super::super::spawn_plan::spawn_after_first();
    crate::sys::serial::print(b"[INIT] installer closed; starting the desktop apps\n");
    true
}
