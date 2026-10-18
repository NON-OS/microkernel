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

//! What the server keeps, and its two halves: a request served in a window
//! of its own, and one step of the search for the device, or of its check,
//! in another. Every call to driver.xhci0 in either waits at most what its
//! window has left, so a request is answered within REQUEST_MS, and a
//! request that comes during a step waits for that step alone: STEP_MS, and
//! the reset after a command that failed and the slot given back after a
//! probe that did (disk/recover.rs, xhci/slot.rs).

use super::dispatch::dispatch;
use super::respond;
use crate::protocol::{parse, refused, E_INVAL};
use crate::scan::Scanner;
use crate::state::State;
use crate::xhci::Window;

/// One block request's time. With the reset after a failed command on top,
/// and a check step before it that keeps the device (STEP_MS), a request
/// sent as the check starts is answered 3 s inside the kernel's 35 s wait
/// for a reply (services/lifecycle/reply_wait.rs); at 20 s it was 1 s.
const REQUEST_MS: u64 = 18_000;
/// One step's: longer than driver.xhci0's own 5 s wait for a transfer, so
/// the controller says why one failed.
const STEP_MS: u64 = 6_000;

pub struct Server {
    /// The bound device's transport: tags, counters, its residue quirk.
    pub state: State,
    /// The codec ops' own, so they never touch the bound device's.
    pub codec: State,
    pub scanner: Scanner,
}

impl Server {
    pub fn new() -> Self {
        Self { state: State::new(), codec: State::new(), scanner: Scanner::new() }
    }

    /// One frame from `sender`. A frame that does not parse is answered
    /// with a refusal, since its caller waits in its call for a reply.
    pub fn serve(&mut self, sender: u32, frame: &[u8], tx: &mut [u8]) {
        let Some((req, body)) = parse(frame) else {
            let _ = respond::status(sender, &refused(frame), E_INVAL, tx);
            return;
        };
        let _window = Window::open(REQUEST_MS);
        dispatch(self, sender, req, body, tx);
    }

    /// One step of the search for the device, or of its check.
    pub fn step(&mut self) {
        let _window = Window::open(STEP_MS);
        self.scanner.step(&mut self.state);
    }
}
