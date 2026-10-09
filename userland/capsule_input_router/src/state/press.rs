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

// Implicit pointer grab armed by a button press inside a window, or on the
// desk for the shell. The origin is the window's screen position frozen at
// press time, so the grab holder receives motion in a coordinate frame where
// deltas equal screen deltas even while it moves itself. The grab lasts while
// any button pressed with it is down: a second button pressed meanwhile, and
// both releases, belong to the same receiver as the first press.
#[derive(Clone, Copy)]
pub struct Press {
    pub pid: u32,
    pub origin_x: i32,
    pub origin_y: i32,
    // One bit per button code held under this grab.
    pub held: u32,
}

impl Press {
    // Arm the grab for a press at screen point (x, y) that the window manager
    // found at (local_x, local_y) inside `pid`'s window: the window's origin is
    // the difference, and stays fixed until the release.
    pub fn arm(pid: u32, x: u32, y: u32, local_x: u32, local_y: u32) -> Press {
        Press {
            pid,
            origin_x: signed(x).saturating_sub(signed(local_x)),
            origin_y: signed(y).saturating_sub(signed(local_y)),
            held: 0,
        }
    }

    // Screen point (x, y) in the frame the press was armed in. A window that
    // drags itself sees the pointer move by exactly the screen delta, whatever
    // it has done with its own position since.
    pub fn local(&self, x: u32, y: u32) -> (i32, i32) {
        (signed(x).saturating_sub(self.origin_x), signed(y).saturating_sub(self.origin_y))
    }

    // Take `button` under the grab. False when it is already held: its release
    // was lost, and this press starts a gesture of its own.
    pub fn hold(&mut self, button: u32) -> bool {
        let bit = bit(button);
        let fresh = self.held & bit == 0;
        self.held |= bit;
        fresh
    }

    // Let `button` go. True when it was held under this grab.
    pub fn lift(&mut self, button: u32) -> bool {
        let bit = bit(button);
        let was = self.held & bit != 0;
        self.held &= !bit;
        was
    }

    // No button is down any more: the grab is over.
    pub fn idle(&self) -> bool {
        self.held == 0
    }
}

// Drivers number buttons from 1; a code past the mask shares its top bit.
fn bit(button: u32) -> u32 {
    1u32 << button.min(31)
}

// The local point comes from the window manager's reply, another process; a
// value past i32 would wrap negative in a cast, so it stops at the limit.
fn signed(v: u32) -> i32 {
    i32::try_from(v).unwrap_or(i32::MAX)
}
