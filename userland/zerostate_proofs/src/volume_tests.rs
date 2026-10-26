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

//! The data volume on a boot that keeps nothing: in RAM, never on the disk,
//! and dropped, not copied, when keeping turns on.

use crate::volume_rule::{drop_at_arm, may_write, place, Place};

#[test]
fn an_installed_disk_s_volume_opens_on_the_disk_only_once_the_boot_keeps() {
    assert_eq!(place(true, true), Place::Disk);
    assert_eq!(place(true, false), Place::Ram, "an installed disk keeping nothing yet");
    assert_eq!(place(false, false), Place::Ram, "a live stick");
    assert_eq!(place(false, true), Place::Ram, "a live stick that keeps has no volume on disk");
}

#[test]
fn a_disk_takes_a_sealed_sector_only_on_a_boot_that_keeps() {
    assert!(!may_write(false, false));
    assert!(may_write(false, true));
    assert!(may_write(true, false), "RAM is not kept");
    assert!(may_write(true, true));
}

#[test]
fn keeping_on_drops_only_a_volume_held_for_an_installed_disk() {
    assert!(drop_at_arm(true));
    assert!(!drop_at_arm(false), "a live stick's RAM volume stays for its boot");
}

/* Every boot, in every order of arm and first open: nothing reaches the
 * disk before keeping, and the volume the disk takes is the disk's own. */
#[test]
fn no_order_of_open_and_arm_writes_the_disk_before_keeping() {
    for installed in [false, true] {
        for arm_first in [false, true] {
            let mut keeping = arm_first;
            let mut held = installed && place(installed, keeping) == Place::Ram;
            let on_disk = place(installed, keeping) == Place::Disk;
            assert!(may_write(!on_disk, keeping));
            if !keeping {
                keeping = true;
                if drop_at_arm(held) {
                    held = false;
                }
            }
            assert!(!held, "no RAM volume stands for an installed disk once it keeps");
            assert_eq!(place(installed, keeping) == Place::Disk, installed);
        }
    }
}
