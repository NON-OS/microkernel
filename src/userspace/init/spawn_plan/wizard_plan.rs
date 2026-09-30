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
 * The boot plan of an image with first-boot setup: the compositor and input
 * router come up, then one thing runs on them before any desktop app. That is
 * setup, or, when the boot menu asked for "Install NONOS", the installer.
 * The rest follows once it is done (see supervisor::after_setup).
 */

use crate::userspace::capsule_setup_wizard as wiz;
use crate::userspace::init::{request_instance, PendingApp};

#[cfg(not(feature = "microkernel-input-probe"))]
pub(in crate::userspace::init) fn spawn_desktop() {
    super::desktop_fleet::spawn_gui_core();
    if crate::boot::handoff::install_requested() {
        spawn_installer_first();
        return;
    }
    super::boot::capsule(
        "SETUP-WIZARD",
        "setup_wizard",
        wiz::spawn_setup_wizard_capsule,
        wiz::shared_state,
    );
}

/*
 * The installer is a window, so it needs the window manager and the shell
 * that hands it its first frame; those start here and nothing else does.
 * Setup is skipped: the disk the installer writes runs setup on its own
 * first boot. Init performs the queued spawn on its first loop pass.
 */
#[cfg(not(feature = "microkernel-input-probe"))]
fn spawn_installer_first() {
    super::desktop_fleet::spawn_rest();
    let line: &[u8] = if request_instance(PendingApp::Install) {
        b"[INIT] boot menu asked to install NONOS; installer queued before the apps\n"
    } else {
        b"[INIT] boot menu asked to install NONOS; the spawn queue is full\n"
    };
    crate::sys::serial::print(line);
}

pub(in crate::userspace::init) fn spawn_post_wizard() {
    super::desktop_fleet::spawn_rest();
    spawn_after_first();
}

/* The market and the apps, once setup or the installer has ended. */
pub(in crate::userspace::init) fn spawn_after_first() {
    super::core::spawn_market();
    super::apps::spawn();
}

pub(in crate::userspace::init) fn spawn_market() {}
