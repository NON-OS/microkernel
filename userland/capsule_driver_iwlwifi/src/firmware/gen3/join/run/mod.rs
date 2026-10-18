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

//! A whole join, from the network's beacon to an open port, and leaving it.
//!
//! The MLME reads the beacon first: a network it will not join (open, TKIP,
//! Enterprise, HT-only, saved as WPA3 and now offering only WPA2, a
//! passphrase that cannot be one) is refused there, before any firmware
//! context exists or any frame is sent. Then the contexts go up (`Bss`), the
//! frames are exchanged to a connected MLME (`exchange`), the pairwise and
//! group keys go into the firmware (`Installed`), the station entry is
//! authorized, and only then is the port opened (`Link`). Any step that ends
//! the join takes down what was put up, keys first.

mod ends;
mod join;
mod leave;
mod open;
mod rekey;

pub use ends::{JoinEnd, Joined, LEAVE_MS};
pub use join::join;
pub use leave::{forget, leave};
