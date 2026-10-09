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

//! How long an app keeps asking for its window before it gives up. Pure, so
//! a host proof holds the numbers.
//!
//! The three steps that make a window (open it with the window manager, put
//! its pixels in the compositor's scene, subscribe to input) were retried a
//! few times with a bare `mk_yield` between. A yield returns at once when no
//! other process wants the processor, so on a machine whose peers are merely
//! slow the attempts were spent in microseconds: the window never opened,
//! and an on-demand instance ended (runner/no_window.rs) instead of waiting
//! out a compositor that was one frame behind. That is the second click a
//! person needed on a slow laptop. The app now rests between attempts, so
//! the time before it gives up is real time, whatever the scheduler does.

/// Attempts at one step, and the rest between them.
pub struct Patience {
    pub attempts: u32,
    pub rest_ms: u64,
}

impl Patience {
    /// The least real time before this step gives up: every call answering
    /// at once with a refusal. The rest after the last attempt is not
    /// waited, since nothing follows it.
    pub const fn floor_ms(&self) -> u64 {
        self.rest_ms * (self.attempts as u64).saturating_sub(1)
    }

    /// The most: every call waiting out the kernel's reply budget.
    pub const fn ceiling_ms(&self) -> u64 {
        self.attempts as u64 * CALL_BUDGET_MS + self.floor_ms()
    }
}

/// What one call waits for its answer: `mk_ipc_call` with no budget of its
/// own, which the kernel bounds (`sys_ipc_call`).
pub const CALL_BUDGET_MS: u64 = 5_000;

/// Opening the window. Without it there is nothing to show, so this is the
/// most patient of the three.
pub const WINDOW_OPEN: Patience = Patience { attempts: 4, rest_ms: 125 };

/// Putting the window's pixels in the compositor's scene. The compositor
/// answers between frames, and a whole frame to a slow display can outlast
/// one call, so each refusal is rested out and tried again.
pub const SCENE_SUBMIT: Patience = Patience { attempts: 8, rest_ms: 60 };

/// Subscribing to input. A window without it is drawn and takes no keys,
/// which the frame loop's heartbeat retries every couple of seconds, so a
/// shorter patience here costs only that wait.
pub const INPUT_SUBSCRIBE: Patience = Patience { attempts: 4, rest_ms: 50 };
