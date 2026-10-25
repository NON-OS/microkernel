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

//! The wallet's screens, the recovery words' standing, and the send form's
//! fields, by number.

pub const MAX_RAILS: usize = 8;
pub const VIEW_HOME: u8 = 0;
pub const VIEW_RECEIVE: u8 = 1;
pub const VIEW_SEND: u8 = 2;
// New hardened-wallet + private-swap + NOX screens.
pub const VIEW_SHIELD: u8 = 6;
pub const VIEW_NOX: u8 = 9;
pub const VIEW_SWAP: u8 = 10;
pub const VIEW_SETTINGS: u8 = 11;
pub const VIEW_IMPORT: u8 = 12;
pub const VIEW_RECOVER: u8 = 13;
pub const VIEW_EXPORT: u8 = 14;
pub const VIEW_ACCOUNTS: u8 = 15;
/* The meter's account and budget (B8). */
pub const VIEW_METER: u8 = 16;
/* Signing a listing as its publisher (B8, plan/31). */
pub const VIEW_PUBLISHER: u8 = 17;
/* A purchase the store named, to confirm (B8, plan/27). */
pub const VIEW_PURCHASE: u8 = 18;
/* Whether the keyring holds account 0's recovery words this session. */
pub const WORDS_UNREAD: u8 = 0;
pub const WORDS_HELD: u8 = 1;
/* The wallet came from a private key: there are no words. */
pub const WORDS_NONE: u8 = 2;
pub const SEND_FIELD_TO: u8 = 0;
pub const SEND_FIELD_AMOUNT: u8 = 1;
