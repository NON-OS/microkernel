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

use core::sync::atomic::Ordering;

use super::authority::{publish, withdraw, Cells};
use super::epoch::advance;
use super::mint::{mint, Binding};

/// One PCB of a thread group: its authority cells and what its token binds.
pub(crate) struct Member<'a> {
    pub cells: Cells<'a>,
    pub binding: Binding,
}

#[derive(Clone, Copy)]
pub(crate) enum Change {
    /// Add bits to the group's set.
    Grant(u64),
    /// Remove bits from the group's set and advance every epoch.
    Revoke(u64),
    /// Replace the set.
    Set(u64),
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(crate) enum Refusal {
    /// No signing key or boot session: no token could be minted.
    Unsigned,
    /// A member's epoch is at its top; that member now holds nothing.
    EpochExhausted,
}

/*
 * Every member gets the same new set, derived from `from` (the bits of the
 * PCB the change was asked for): a thread is its process and carries the
 * process's capabilities. A revoke advances each member's epoch before it
 * publishes that member's token, so the member's old token is stale from
 * the advance on, even if minting then fails. A member whose epoch cannot
 * advance is given an empty token: a refused revoke never leaves more
 * authority than the revoke asked for. A revoke that cannot mint at all
 * clears the member's list, so nothing is left decided on the old token.
 */
pub(crate) fn reissue(members: &[Member], from: u64, change: Change) -> Result<(), Refusal> {
    let bits = match change {
        Change::Grant(mask) => from | mask,
        Change::Revoke(mask) => from & !mask,
        Change::Set(bits) => bits,
    };
    let mut outcome = Ok(());
    for m in members {
        let (epoch, held) = match change {
            Change::Revoke(_) => match advance(m.cells.epoch) {
                Some(e) => (e, bits),
                None => {
                    outcome = Err(Refusal::EpochExhausted);
                    (u64::MAX, 0)
                }
            },
            _ => (m.cells.epoch.load(Ordering::Acquire), bits),
        };
        match mint(held, Binding { epoch, ..m.binding }) {
            Some(token) => publish(&m.cells, token),
            None => {
                if let Change::Revoke(_) = change {
                    withdraw(&m.cells);
                }
                if outcome.is_ok() {
                    outcome = Err(Refusal::Unsigned);
                }
            }
        }
    }
    outcome
}
