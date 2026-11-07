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

//! Auto-lock: once the policy's auto-lock time has passed with no input,
//! the router tells the shell, once for each stretch without input; the
//! shell locks if a passphrase is set. An idle inhibit (a playing video, a
//! call) holds it back only when the person chose that in Settings
//! (LockInhibitHolds); otherwise it holds back the blank alone. Nothing is
//! sent while the session is locked already.

const MINUTE_MS: i64 = 60_000;

#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub struct AutoLock {
    /// The auto-lock time in ms; 0 never locks.
    pub lock_ms: i64,
    /// Whether an idle inhibit holds back auto-lock too.
    pub inhibit_holds: bool,
    /// The input time of the stretch the shell was told about.
    told_for: Option<i64>,
}

impl AutoLock {
    pub const fn new() -> AutoLock {
        AutoLock { lock_ms: 0, inhibit_holds: false, told_for: None }
    }

    /// The policy's auto-lock time in minutes (0 never), and its choice for
    /// the inhibit.
    pub fn set(&mut self, minutes: u8, inhibit_holds: bool) {
        self.lock_ms = i64::from(minutes) * MINUTE_MS;
        self.inhibit_holds = inhibit_holds;
    }

    /// Whether to tell the shell now.
    pub fn due(&self, last_input_ms: i64, now_ms: i64, inhibited: bool, locked: bool) -> bool {
        self.lock_ms > 0
            && !locked
            && !(inhibited && self.inhibit_holds)
            && self.told_for != Some(last_input_ms)
            && now_ms.saturating_sub(last_input_ms) >= self.lock_ms
    }

    /// The shell has it, for the stretch since `last_input_ms`.
    pub fn told(&mut self, last_input_ms: i64) {
        self.told_for = Some(last_input_ms);
    }
}
