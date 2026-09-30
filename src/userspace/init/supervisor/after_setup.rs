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
 * Starting the desktop once first-boot setup has ended. On an install boot
 * setup runs first and its answer decides: Install (or no answer at all,
 * since the boot menu asked for it) hands over to the installer ahead of
 * the apps; Amnesic is respected and the desktop starts without one.
 */

use crate::userspace::capsule_setup_wizard::{ended, Ended};
use crate::userspace::init::{request_instance, PendingApp};

/// True once setup has ended and the desktop has been started behind it.
/// When setup asked for the installer, it is queued to open once the desktop
/// is up; init drains that queue on the same loop.
pub(super) fn poll() -> bool {
    if super::after_install::handed_over() {
        return super::after_install::poll();
    }
    let Some((end, apps_off)) = ended() else {
        return false;
    };
    /* Before any app spawns, so none the person turned off ever does. */
    super::super::app_choice::choose(apps_off);
    if crate::boot::handoff::install_requested() {
        return after_install_boot(end);
    }
    super::super::spawn_plan::spawn_post_wizard();
    if end == Ended::Installer {
        let line: &[u8] = if request_instance(PendingApp::Install) {
            b"[INIT] setup asked for the installer; queued\n"
        } else {
            b"[INIT] setup asked for the installer; the queue is full\n"
        };
        crate::sys::serial::print(line);
    }
    true
}

fn after_install_boot(end: Ended) -> bool {
    if end == Ended::Desktop {
        super::super::spawn_plan::spawn_post_wizard();
        crate::sys::serial::print(b"[INIT] install boot, setup chose not to install; desktop\n");
        return true;
    }
    super::after_install::hand_over();
    false
}
