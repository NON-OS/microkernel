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

//! A Linux run holds every pack of its program open, one a package, for as
//! long as it runs. KeePassXC's closure is about 150 packages, and with 128
//! handles an owner its last packs failed to open and Qt found no Wayland
//! platform. An owner's share must hold the largest app's closure.

use crate::vfs_store::{Store, PER_OWNER_FDS};

/// The largest closure an app listed today needs, with room to grow.
const LARGEST_CLOSURE: usize = 160;

#[test]
fn one_run_holds_a_large_apps_packs_and_another_still_opens() {
    assert!(PER_OWNER_FDS >= 2 * LARGEST_CLOSURE, "two large apps run from one family");
    let mut s = Store::new();
    for _ in 0..2 * LARGEST_CLOSURE {
        s.open("/data/f", 7, true, false, false, true).expect("within one run's share");
    }
    assert!(s.open("/data/f", 8, true, false, false, true).is_ok(), "another client still opens");
}
