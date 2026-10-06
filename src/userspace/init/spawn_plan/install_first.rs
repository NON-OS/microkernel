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
 * An install boot whose setup chose Install, or ended without a choice:
 * the installer starts at once, on the compositor and input router setup
 * ran on, and owns the whole screen. No desktop, shell or app
 * starts beside it; supervisor::after_install starts the desktop only if
 * it ends without the machine restarting. It is spawned here rather than
 * queued, since the queue's boot frame comes from a shell that is not
 * there. Setup has kept its answers, which the installer carries.
 */

/* True when the installer is running. */
pub(in crate::userspace::init) fn spawn_installer_first() -> bool {
    let started = spawn();
    let line: &[u8] = if started {
        b"[INIT] install boot: the installer has the screen; no desktop until it ends\n"
    } else {
        b"[INIT] install boot: the installer did not start; starting the desktop\n"
    };
    crate::sys::serial::print(line);
    started
}

#[cfg(feature = "nonos-capsule-install")]
fn spawn() -> bool {
    crate::userspace::capsule_install::spawn_install_instance().is_ok()
}

#[cfg(not(feature = "nonos-capsule-install"))]
fn spawn() -> bool {
    false
}
