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

//! What the link tells the user and asks of the user, through whoever
//! serves the capsule's protocol (`crate::nbtc` in the capsule).

/// What pairing waits on the user for (Vol 3, Part H, 2.3.5.1).
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Asked {
    /// The six digits the device shows, typed here.
    Passkey,
    /// These six digits, shown on both sides, confirmed the same.
    Number(u32),
}

/// The user's answer.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Answer {
    Passkey(u32),
    Same(bool),
    Cancel,
}

pub trait Desk {
    /// HID is ready: what the device is, and whether its keys post.
    fn ready(&mut self, keyboard: bool, mouse: bool, keys_post: bool);
    /// The link runs on the bond with this identity; `fresh` when pairing
    /// just made it.
    fn linked(&mut self, identity_type: u8, identity: [u8; 6], fresh: bool);
    /// Pairing waits on the user; the answer comes through `answer`.
    fn asked(&mut self, asked: Asked);
    fn answer(&mut self) -> Option<Answer>;
    /// Whether a device that holds no bond here may pair now.
    fn may_pair(&mut self) -> bool;
    /// Whether the user asked for the link to end, its bond forgotten.
    fn hang_up(&mut self) -> bool;
    /// Whether the radio is turned off: a live link ends and nothing more
    /// is sent on it, the bond kept for when it comes back (E-72).
    fn radio_off(&mut self) -> bool;
}
