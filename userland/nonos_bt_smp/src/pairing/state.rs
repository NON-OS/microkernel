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

//! Where the initiator is in pairing (Vol 3, Part H, 2.3.5.6).

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum State {
    AwaitResponse,
    AwaitPublicKey,
    /// The user's passkey before round 0 can start.
    AwaitPasskey,
    /// The responder's confirm for this round (always 0 unless passkey).
    AwaitConfirm(u8),
    /// The responder's random for this round.
    AwaitRandom(u8),
    /// The user's yes or no on the numeric comparison.
    AwaitUserConfirm,
    AwaitDhKeyCheck,
    AwaitEncryption,
    /// The responder's keys still due, as key distribution bits.
    AwaitKeys(u8),
    Done,
    Failed,
}

impl State {
    pub fn finished(self) -> bool {
        matches!(self, State::Done | State::Failed)
    }
}
