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
 * Starting the desktop once first-boot setup has ended. Setup's answer
 * decides: Install hands the whole screen to the installer and starts no
 * desktop, on any boot. On an install boot, where the boot menu asked for
 * the installer, no answer at all does the same; Amnesic is respected and
 * the desktop starts.
 */

use crate::userspace::capsule_setup_wizard::{ended, Ended};

/// True once setup has ended and the desktop has been started behind it.
pub(super) fn poll() -> bool {
    if super::after_install::handed_over() {
        return super::after_install::poll();
    }
    let Some((end, apps_off)) = ended() else {
        return false;
    };
    /* Before any app spawns, so none the person turned off ever does. */
    super::super::app_choice::choose(apps_off);
    let install = match end {
        Ended::Installer => true,
        Ended::Unfinished => crate::boot::handoff::install_requested(),
        Ended::Desktop => false,
    };
    if install {
        super::after_install::hand_over();
        return false;
    }
    super::super::spawn_plan::spawn_post_wizard();
    true
}
