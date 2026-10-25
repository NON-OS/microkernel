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

//! Power off from the desktop. The kernel's power off wipes memory and ends
//! the session, so it is never sent on one click or one key: the shell first
//! asks, in the trusted band where no window can draw or answer, and only a
//! "Power Off" from the input router sends it, once.

use super::input_source_rule::Routed;

/// Permission to send one power-off request. Made only by `PowerOff::answer`, for
/// a confirmation that came from the router; neither Clone nor Copy, so the
/// sender consumes it and one answer sends at most once.
#[derive(Debug, PartialEq, Eq)]
pub struct Confirmed(());

/// Whether the confirmation is up.
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub struct PowerOff {
    asking: bool,
}

impl PowerOff {
    /// Ask: from the menu or the power key. Asking again changes nothing.
    pub fn ask(&mut self) {
        self.asking = true;
    }

    pub fn showing(self) -> bool {
        self.asking
    }

    /// The router's answer closes the confirmation. Permission to send comes
    /// only when it was up and the answer was Power Off; anything else sends
    /// nothing.
    pub fn answer(&mut self, _by: Routed, confirm: bool) -> Option<Confirmed> {
        let was = core::mem::replace(&mut self.asking, false);
        (was && confirm).then_some(Confirmed(()))
    }
}

/// Said when the service refused, or never answered and the machine is
/// still here.
pub const POWER_OFF_FAILED: &[u8] = b"Power off failed; the system is still running";
