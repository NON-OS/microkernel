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

//! NBTC v1, the wire format of driver.bt_usb0's control service, for both
//! ends of it. The capsule answers it; Settings and the shell ask it.

#![no_std]

pub mod body;
pub mod bonds;
pub mod header;
pub mod levels;
pub mod list;
pub mod look;
pub mod ops;
pub mod part;

pub use body::{answer_body, forget_body, scan_body, Reply};
pub use bonds::{Bond, Bonds};
pub use header::{read_reply, request, DATA_AT, HDR_LEN, MAGIC, STATUS_LEN, VERSION};
pub use levels::{Level, Levels};
pub use list::{BONDS_MAX, BOND_LEN, LEVELS_MAX, LEVEL_LEN, REPLY_MAX};
pub use look::{Health, Link, NO_PLACE};
pub use part::{Asked, PASSKEY_MAX};
