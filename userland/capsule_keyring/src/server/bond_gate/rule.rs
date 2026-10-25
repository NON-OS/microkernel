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

//! Who may keep records by name (L33): the services whose keys must outlive
//! their own restart. A name is held by one process at a time, and a capsule
//! registers only the names its manifest gives it, so whoever owns the name
//! now is the driver that stored the records, restarted or not.
//!
//! Pure: the caller supplies who owns each name. Held in `wallet_proofs`.

/// A record keeps its holder's place in this list. Append only: a place once
/// given names that service for good.
pub const BOND_HOLDERS: [&[u8]; 1] = [b"driver.bt_usb0"];

/// The place of the holder name `sender_pid` owns, given `owner_of`, the
/// pid that owns a name.
pub fn holder_of(sender_pid: u32, owner_of: impl Fn(&[u8]) -> Option<u32>) -> Option<u8> {
    if sender_pid == 0 {
        return None;
    }
    let at = BOND_HOLDERS.iter().position(|name| owner_of(name) == Some(sender_pid))?;
    u8::try_from(at).ok()
}
