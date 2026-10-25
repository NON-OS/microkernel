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

//! The server's state: the device, the platform, one slot per output
//! (its current primary and the one waiting for RETIRE after a MODE_SET)
//! and the owner, the last pid to ask SURFACE (only the compositor can
//! send, so a new owner is a restarted compositor). Serials come from one
//! counter, so a serial names one primary across every output.

use super::display::{Display, Shadow};
use super::platform::Platform;
use super::primary::Primary;
use super::slot::{Slot, MAX_OUTPUTS};
use super::watch_tell::Tell;

pub struct Server<D: Display, P: Platform> {
    pub display: D,
    pub platform: P,
    pub slots: [Slot; MAX_OUTPUTS],
    pub owner: Option<u32>,
    pub counter: u32,
    /// When the owner was last heard, and whether its silence has blanked
    /// the outputs since (`watch.rs`).
    pub heard_ms: u64,
    pub quiet: bool,
    /// Whether this silence's blank has been said (`watch_tell.rs`).
    pub tell: Tell,
}

impl<D: Display, P: Platform> Server<D, P> {
    pub fn new(display: D, platform: P) -> Self {
        let slots = [Slot::default(); MAX_OUTPUTS];
        Server {
            display,
            platform,
            slots,
            owner: None,
            counter: 0,
            heard_ms: 0,
            quiet: false,
            tell: Tell::Quiet,
        }
    }

    /// Run `f` on the shadow of `p`.
    pub fn with_shadow<T>(&mut self, p: Primary, f: impl FnOnce(&mut D, &Shadow) -> T) -> T {
        let shadow = Shadow {
            bytes: self.platform.bytes(p.region),
            width: p.width,
            height: p.height,
            stride: p.stride,
            cursor_offset: p.cursor_offset,
        };
        f(&mut self.display, &shadow)
    }
}
