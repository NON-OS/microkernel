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

//! A signal's disposition: what the guest asked to happen when it fires.
//! Process-wide, as on Linux.

/// The largest signal Linux defines.
pub const NSIG: usize = 64;

/// `struct sigaction` as the guest passes it: handler, flags, restorer, mask.
#[derive(Clone, Copy, Default)]
pub struct SigAction {
    pub handler: u64,
    pub flags: u64,
    pub restorer: u64,
    pub mask: u64,
}

impl SigAction {
    /// SIG_DFL is a null handler and SIG_IGN is 1; neither enters guest code.
    pub fn catches(&self) -> bool {
        self.handler > 1
    }
    pub fn ignores(&self) -> bool {
        self.handler == 1
    }
}
