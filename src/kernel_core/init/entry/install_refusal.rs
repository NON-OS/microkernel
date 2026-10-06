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

use crate::sys::boot_log;

/*
 * An image can install only with first-boot setup, whose profile holds the
 * desktop back until setup or the installer is done, and with the installer
 * capsule itself.
 */
const HAS_INSTALLER: bool =
    cfg!(all(feature = "microkernel-setup-wizard", feature = "nonos-capsule-install"));

/*
 * The boot menu's "Install NONOS" reached a kernel that cannot install.
 * Booting on into the desktop would look as if the choice had been ignored,
 * so the refusal goes on the panel and the boot stops there; the person
 * restarts and picks another entry. Runs before init, so nothing has been
 * started that the refusal would have to undo.
 */
pub(super) fn refuse_install_without_installer() {
    if HAS_INSTALLER || !crate::boot::handoff::install_requested() {
        return;
    }
    boot_log::show_notice(
        b"Install NONOS: this image has no installer",
        &[
            b"This kernel was built without the installer, so it cannot install NONOS.",
            b"Nothing was written to any disk. Restart and choose another boot entry.",
        ],
    );
    boot_log::error("install requested, but this image has no installer; stopping");
    crate::arch::halt_loop()
}
