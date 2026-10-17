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

//! Where each thread is in its path, and each process: its driver service,
//! the name the kernel knows it by, its claim attempts and its exit's hold.

use super::holder_rule::Name;
use super::model_stepper::Stepper;
use super::world_types::Wipe;

#[derive(Clone, Copy, PartialEq, Eq, Debug, Default)]
pub(super) enum Path {
    #[default]
    Off,
    /// In user code between claim attempts.
    Idle,
    /// claim.rs `claim`, and from step 9 its withdraw.
    Claim,
    /// A tenancy, in the driver's own code.
    Hold,
    /// give_back.rs `give_back`, to release.rs `release`.
    Give,
    /// exit.rs, `exit_begin` to `exit_end`.
    Exit,
    Done,
}

/// A thread or a CPU's tick: its step, when it may move, the claim it works
/// on, and the `step_all` call it is inside.
#[derive(Clone, Copy, Debug, Default)]
pub(super) struct Thread {
    pub path: Path,
    pub step: u8,
    pub at: u64,
    pub epoch: u64,
    pub stepper: Option<Stepper>,
}

/// A process. `svc` is its driver service as the invariants know it (A and
/// A2 share one); `name` is what `name_of` makes of what it serves.
#[derive(Clone, Copy, Debug, Default)]
pub(super) struct Proc {
    pub svc: u8,
    pub name: Option<Name>,
    pub dead: bool,
    pub attempts: u8,
    /// When the attempt began and how often it asked again: libc retries
    /// for a bounded time (L80), its count of tries a backstop.
    pub started: u64,
    pub tries: u32,
    /// The kept bound as the record's hold asked it (L80).
    pub over_cap: bool,
    /// The loss the attempt began on, past its hold and back by then (I8).
    pub owed_back: Option<u64>,
    /// The record as the attempt's entry was made (I7).
    pub before: Option<Wipe>,
    /// The claim its exit counted itself among the releases of.
    pub held: Option<u64>,
    pub maps: u8,
}
