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

//! A reveal lapses, whatever the clock reads.

use super::rig::WINDOW;
use crate::taskbar::{
    expire_taskbar_visibility, new_taskbar_state, reveal_taskbar, set_full_screen, Uptime,
};

/// The brand's reveal over a full-screen window lapses after its 1.8 s, and
/// also when the clock reads behind the time it was set.
#[test]
fn a_reveal_lapses_on_time_and_when_the_clock_reads_behind_it() {
    let mut t = new_taskbar_state();
    set_full_screen(&mut t, 300, WINDOW, true);
    let set = 5 * 3_600_000;
    assert!(reveal_taskbar(&mut t, Uptime(set)));
    assert!(t.revealed);
    assert!(!expire_taskbar_visibility(&mut t, Uptime(set + 1_799)));
    assert!(expire_taskbar_visibility(&mut t, Uptime(set + 1_800)));
    assert!(!t.revealed);

    assert!(reveal_taskbar(&mut t, Uptime(set)));
    assert!(expire_taskbar_visibility(&mut t, Uptime(set - 3_600_000)));
    assert!(!t.revealed, "a reveal set later than the clock reads lapses");
}
