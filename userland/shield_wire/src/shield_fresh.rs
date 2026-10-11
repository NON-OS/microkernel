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
//! SHIELD_FROM_FRESH (op 18, v1.4.56): NOX found at a fresh address the
//! shield handed out, moved into the Shield from that address as standard
//! deposits, a job each step. Fields, one to a line:
//! "review", the address as 0x hex, the NOX there in wei; or "confirm" and
//! the review's id. A review answers as OP_REVIEW_SHIELD does, with
//! `remainder` (NOX wei left at the address) and `short_eth` (ETH wei the
//! address lacks for the gas); a confirm answers each `tx` sent, and
//! `stopped` if one failed.

pub const OP_SHIELD_FROM_FRESH: u16 = 18;
pub const SHIELD_FRESH_REVIEW: &str = "review";
pub const SHIELD_FRESH_CONFIRM: &str = "confirm";
