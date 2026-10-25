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

//! The relay fetch carries containers to (the owner's O10): None until one
//! is pinned, and then nothing leaves the machine; every op is NO_RELAY.
//! Its budgets (plan/14): a stream opens within 45 s (nonos_route_link's
//! OPEN_MS); a read waits 60 s on Nym, 30 s on Anyone; one op takes at most
//! 180 s over at most 6 connections.

pub const RELAY: Option<(&str, u16)> = None;

pub const READ_NYM_MS: i64 = 60_000;
pub const READ_ANYONE_MS: i64 = 30_000;
pub const OP_MS: i64 = 180_000;
pub const ROTATIONS: usize = 6;
