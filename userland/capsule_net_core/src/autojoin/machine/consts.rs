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

//! How long autojoin waits for each thing it waits on.

/// The longest any call autojoin makes from the serve loop waits for its
/// answer.
pub const CALL_MS: u64 = 25;
/// Waiting for the driver, the store or the policy restore.
pub const NOT_YET_MS: i64 = 3_000;
/// The driver is ready but no saved network was heard: scan again this much
/// later.
pub const RESCAN_MS: i64 = 15_000;
/// Passes with the driver ready and no saved network in range before autojoin
/// gives up for this boot.
pub const EMPTY_PASSES_MAX: u8 = 8;
/// How long a join handed to the driver is watched before it counts as failed:
/// past the driver's own bound (the hunt and its joins, at most 16 s, on the RTL8821CE),
/// with room for the access point's slowest handshake.
pub const JOIN_WATCH_MS: i64 = 25_000;
/// How often a running join's link is looked at.
pub const WATCH_EVERY_MS: i64 = 1_000;
/// How often the link is looked at once autojoin is done.
pub const LINK_EVERY_MS: i64 = 2_000;
/// Passes a rejoin makes after one loss before it gives up.
pub const REJOIN_ATTEMPTS: u8 = 8;
/// From the loss to a rejoin's first pass; each later wait is twice the one
/// before, up to `REJOIN_MAX_MS`.
pub const REJOIN_FIRST_MS: i64 = 2_000;
pub const REJOIN_MAX_MS: i64 = 60_000;
