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

//! The parts the enter model's world is made of (L78): a claim's entry, the
//! function's wipe record, the copy a stepper acts on with no lock held, and
//! the knob that undoes one design for an undo check.

use super::end_rule::End;
use super::enter_stage::Stage;
use super::method_rule::Method;
use super::phase_rule::Phase;

/// A path done; a claimant's call that ended admitted, or not.
pub(super) const END: u8 = 100;
pub(super) const IN: u8 = 101;
pub(super) const OUT: u8 = 102;
/// Calls libc makes again on Resetting (the model's bound).
pub(super) const TRIES: u8 = 2;

/// The kernel as built, or one design undone: admits asked apart from the
/// push (2), owe apart from the removal (3), the reset waited out inside
/// Making (7), Held written at enter (11), a claimant waiting in its entry (12).
#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub(super) enum Knob {
    Built,
    SplitAdmit,
    OweApart,
    ResetInMaking,
    HeldAtEnter,
    WaitInMaking,
}

/// A claim's entry: its epoch, stage and releases under way.
#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub(super) struct Entry {
    pub epoch: u8,
    pub stage: Stage,
    pub leavers: u8,
}

/// The wipe record: its epoch, its holder's service (0 is A's and A2's, 1
/// B's), phase, method written, marker set, and a stepper's busy mark.
#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub(super) struct Rec {
    pub epoch: u8,
    pub by: u8,
    pub phase: Phase,
    pub method: Option<Method>,
    pub marker: bool,
    pub busy: bool,
}

/// What a stepper took: the record's copy, the end it came to, its time.
#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub(super) struct Job {
    pub rec: Rec,
    pub end: Option<End>,
    pub now: u64,
}
