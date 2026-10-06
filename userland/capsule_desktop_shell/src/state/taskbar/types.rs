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

pub const TASKBAR_APP_MAX: usize = crate::state::apps::LAUNCHER_APPS.len();
pub const TASKBAR_NO_ACTIVE: u8 = 0xFF;

/// Milliseconds on the uptime clock, the only time the dock's timers take.
/// The wall clock reads an error until the RTC is read (and the shell's
/// first seconds are before that) and NTP steps it back by hours on a
/// laptop whose RTC keeps local time. On it a launch pulse, a brand reveal
/// or a window wait set before the step ran for as long as the step, and
/// one set while it read the error never ended at all.
#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Debug)]
pub struct Uptime(pub i64);

/// One open app window, as the window manager announced it.
#[derive(Clone, Copy)]
pub struct TrackedWindow {
    pub window_id: u32,
    pub pid: u32,
    pub app: u8,
}

pub struct TaskbarState {
    pub open: [bool; TASKBAR_APP_MAX],
    pub windows: alloc::vec::Vec<TrackedWindow>,
    pub pulse_until_ms: [i64; TASKBAR_APP_MAX],
    pub reveal_until_ms: i64,
    pub active: u8,
    /// The dock is shown, as the rule has it (dock_rule.rs).
    pub visible: bool,
    /// The windows the window manager says cover the dock's band, by owner
    /// pid and window id: full screen and not minimised.
    pub full_screen: alloc::vec::Vec<(u32, u32)>,
    /// The pointer touched the bottom edge (or the brand was pressed) while
    /// a full-screen window was up, so the dock is over it.
    pub revealed: bool,
    /// The pointer is in the dock's area, its panel and shadow down to the
    /// bottom edge.
    pub pointer_in_dock: bool,
    /// The dock as last painted and committed: shown, or its area clear.
    pub drawn: bool,
    /// The dock's popup window is open with the window manager, which ranks
    /// it over every window, so presses there reach the dock.
    pub window_open: bool,
    /// The apps a click asked a new window of, and when (expect.rs).
    pub expecting: alloc::vec::Vec<(u8, i64)>,
}
