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

//! The trust an input event carries to the router (G-76). Two bits of
//! `InputEvent.flags`, which the modifiers (bits 0..6) leave free:
//! SOURCE_BT marks every event this Bluetooth capsule posts, and
//! BOND_AUTHENTICATED marks the ones from a bond Passkey Entry or Numeric
//! Comparison protected. The router treats SOURCE_BT without
//! BOND_AUTHENTICATED as unauthenticated, and strips both before delivery,
//! so a device cannot set them itself. The two values are G-76's; under the
//! no-new-libc-constants rule they are copied here, to be held equal at
//! integration against `capsule_input_router/src/route/bond.rs` (not yet in
//! this tree).

/// Posted by the Bluetooth capsule (bit 15).
pub const SOURCE_BT: u16 = 1 << 15;
/// The posting bond was authenticated (bit 14).
pub const BOND_AUTHENTICATED: u16 = 1 << 14;

/// The trust bits for an event from a link at `authenticated` level.
pub fn source_flags(authenticated: bool) -> u16 {
    SOURCE_BT | if authenticated { BOND_AUTHENTICATED } else { 0 }
}
