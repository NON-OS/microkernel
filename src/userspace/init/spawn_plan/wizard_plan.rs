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
 * router come up, then setup runs on them before any desktop app, on an
 * install boot too, where it opens with "Install to this computer" chosen.
 * The rest follows once it is done (see supervisor::after_setup).
 */

use crate::userspace::capsule_setup_wizard as wiz;

#[cfg(not(feature = "microkernel-input-probe"))]
pub(in crate::userspace::init) fn spawn_desktop() {
    super::desktop_fleet::spawn_gui_core();
    super::boot::capsule(
        "SETUP-WIZARD",
        "setup_wizard",
        wiz::spawn_setup_wizard_capsule,
        wiz::shared_state,
    );
}

pub(in crate::userspace::init) fn spawn_post_wizard() {
    super::desktop_fleet::spawn_rest();
    spawn_after_first();
}

/* The market and the apps, once setup or the installer has ended. */
fn spawn_after_first() {
    super::core::spawn_market();
    super::apps::spawn();
}

pub(in crate::userspace::init) fn spawn_market() {}
