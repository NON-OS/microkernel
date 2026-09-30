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

//! Starting the desktop once first-boot setup has ended.

use crate::userspace::capsule_setup_wizard::{ended, Ended};
use crate::userspace::init::{request_instance, PendingApp};

/// True once setup has ended and the desktop has been started behind it.
/// When setup asked for the installer, it is queued to open once the desktop
/// is up; init drains that queue on the same loop.
pub(super) fn poll() -> bool {
    let Some(end) = ended() else {
        return false;
    };
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
