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

//! The cases the floors count, and the windows the invariants judge by: the
//! plan's own numbers, kept apart from the rules they judge so a rule's
//! window undone is seen. No access for 100 ms after an FLR, nor 10 ms after
//! a move through D3hot, then up to a second of retry status (all recalled,
//! unverified); Transactions Pending waited 100 ms and a lost function
//! refused 2 s (our bounds).

use super::phase_rule::MS;

pub(super) const QUIET_FLR: u64 = 100 * MS;
pub(super) const QUIET_PM: u64 = 10 * MS;
pub(super) const RETRY: u64 = 1000 * MS;
pub(super) const TP_HOLD: u64 = 100 * MS;
pub(super) const HOLD_LOST: u64 = 2000 * MS;
/// The longest a step may be waited for: a tick's gap with a stall, after
/// that tick's own settle.
pub(super) const GAP: u64 = 500 * MS;
/// The longest a wipe stays pending: Transactions Pending's wait, an FLR's
/// whole window, then the power cycle's, and a gap at each of four waits.
pub(super) const BOUND: u64 = TP_HOLD + QUIET_FLR + RETRY + 2 * QUIET_PM + RETRY + 4 * GAP;

/// B let in after a clean reset; A2 let in over a kept function; a lost one
/// asked again and someone let in after; the power cycle after an FLR.
pub(super) const B_CLEAN: usize = 0;
pub(super) const A2_KEPT: usize = 1;
pub(super) const LOST_ADMIT: usize = 2;
pub(super) const FALLBACK: usize = 3;
/// A claim refused over the kept frames' bound; a claimant killed.
pub(super) const CAP: usize = 4;
pub(super) const KILLED: usize = 5;
/// A tick found the table held; a step found the record being stepped.
pub(super) const CONTEND: usize = 6;
pub(super) const BUSY: usize = 7;
/// A start held off by pending transactions; each end of a wipe.
pub(super) const TP_STAY: usize = 8;
pub(super) const CLEAN: usize = 9;
pub(super) const UNRESET: usize = 10;
pub(super) const LOST: usize = 11;
/// A claim withdrawn; libc asking again on Resetting.
pub(super) const WITHDRAWN: usize = 12;
pub(super) const RETRIED: usize = 13;
/// A kept run settled before the registry let go, and the other way about.
pub(super) const SETTLE_FIRST: usize = 14;
pub(super) const LET_GO_FIRST: usize = 15;
/// A lost function, back and past its hold, refused by the kept bound.
pub(super) const CAP_LOST: usize = 16;
pub(super) const SEEN: usize = 17;
