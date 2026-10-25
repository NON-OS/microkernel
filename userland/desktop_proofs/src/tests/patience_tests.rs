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

//! How long an app keeps asking for its window (app_skeleton
//! setup/patience.rs), and that the shell waits longer for the window than
//! the app spends trying to open it.
//!
//! The steps were retried with a bare yield between, which returns at once
//! when nothing else wants the processor: on a slow laptop the attempts went
//! by in microseconds, the window never opened, and an on-demand instance
//! ended rather than waiting out a compositor one frame behind.

use nonos_app_skeleton::setup::patience::{
    CALL_BUDGET_MS, INPUT_SUBSCRIBE, SCENE_SUBMIT, WINDOW_OPEN,
};

use crate::taskbar::expect::EXPECT_MS;

/// Every refusal answered at once still leaves real time between the
/// attempts: a tenth of a second at the least, not microseconds.
#[test]
fn a_step_is_not_spent_in_microseconds() {
    for step in [&WINDOW_OPEN, &SCENE_SUBMIT, &INPUT_SUBSCRIBE] {
        assert!(step.attempts >= 4, "too few attempts");
        assert!(step.rest_ms >= 50, "the rest is too short to be a rest");
        assert!(step.floor_ms() >= 150, "spent in {} ms", step.floor_ms());
    }
}

/// A window whose peers are slow but answering comes inside the wait the
/// shell gives it (state/taskbar/expect.rs): the rests of both steps, and
/// one answer that takes the whole call budget in each, fit in it. Said
/// earlier, the toast would name a window that was still on its way.
#[test]
fn the_shell_waits_out_a_slow_window() {
    let slow = WINDOW_OPEN.floor_ms() + SCENE_SUBMIT.floor_ms() + 2 * CALL_BUDGET_MS;
    assert!(slow <= EXPECT_MS as u64, "the window takes {slow} ms, the shell waits {EXPECT_MS} ms");
}

/// A peer that never answers at all: both steps give up, the instance ends
/// (runner/no_window.rs, exit 3, said under `log app-fail`), and nothing is
/// left holding its slot. The shell has already said the app did not open,
/// so the wait is the shorter of the two and the person is told first.
#[test]
fn a_wedged_peer_is_said_before_the_app_gives_up() {
    let wedged = WINDOW_OPEN.ceiling_ms() + SCENE_SUBMIT.ceiling_ms();
    assert!(wedged >= EXPECT_MS as u64, "the app gives up before the shell speaks");
}
