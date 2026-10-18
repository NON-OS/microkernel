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

//! The Shield screens, assets and fields, by number.

pub const SHIELD_HOME: u8 = 0;
pub const SHIELD_DEPOSIT: u8 = 1;
pub const SHIELD_SEND: u8 = 2;
pub const SHIELD_WITHDRAW: u8 = 3;
pub const SHIELD_REVIEW: u8 = 4;
pub const SHIELD_PROVING: u8 = 5;
pub const SHIELD_HISTORY: u8 = 6;
pub const SHIELD_NETWORK: u8 = 7;
/* Never shown on its own: the review's origin when the holder lands a
 * payment from their own account because no lander took it. */
pub const SHIELD_SETTLE: u8 = 8;

/* Assets by index: 0 is ETH. */
pub const ASSET_NOX: u8 = 1;

pub const FIELD_TO: u8 = 0;
pub const FIELD_AMOUNT: u8 = 1;
pub const FIELD_OVERRIDE: u8 = 2;
