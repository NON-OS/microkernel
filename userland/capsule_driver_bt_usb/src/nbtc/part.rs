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

//! The pieces of the board that are not the link's state itself.

/// A bond's identity: its address type (0 public, 1 static random) and
/// address, as `BondRecord` keeps them.
pub type Identity = (u8, [u8; 6]);

/// More levels than this are not kept: four Battery Services for each of
/// eight devices (`nonos_bt_hogp` reads at most four on a link).
pub use nonos_bt_proto::LEVELS_MAX;

/// What pairing asks of the user (Vol 3, Part H, 2.3.5.1), as PASSKEY
/// carries it to Settings.
pub use nonos_bt_proto::Asked as Part;

/// A level: whose (`None` for a link that kept no bond), which Battery
/// Service, and the percent.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct Level {
    pub who: Option<Identity>,
    pub instance: u8,
    pub percent: u8,
}
