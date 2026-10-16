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

//! A load the installer is still running when `install` stops waiting for
//! its answer. The installer reads the capsule out of the store and runs the
//! whole verify and attestation before it answers, seconds for a large one,
//! and the terminal used to wait for that on its window's thread, up to
//! 30 s. The request now gets a short wait, and a load still running is
//! followed from the job's ticks.
//!
//! The late answer cannot be read: it lands in the terminal's reply inbox,
//! where its next call drops it as not its own. The installer makes the
//! terminal the parent of what it loads, so the loaded program is a child
//! that was not there before and is no older than the load. And the
//! installer serves one request at a time, in order, so once a health probe
//! sent after the load is answered, the load is over: no such child by then
//! means it failed. Hand-synced with the desktop's
//! `capsule_desktop_shell/src/state/launch.rs`, which follows a Launchpad
//! launch the same way.

use alloc::vec::Vec;

#[path = "follow_pace.rs"]
mod follow_pace;
#[path = "follow_step.rs"]
mod follow_step;
/// The most a load is followed: the budget the blocking call had.
pub const DEADLINE_MS: i64 = 30_000;
/// The first probe, then twice the gap each time up to the cap. Every probe
/// the installer is too busy to answer keeps a place in its reply queue
/// until it does, and the kernel gives one caller eight such places; this
/// holds six at most over the whole deadline, the load's own one beside.
pub const FIRST_PROBE_MS: i64 = 500;
pub const PROBE_GAP_MAX_MS: i64 = 8_000;
/// How often the process table is read for the child.
pub const LOOK_GAP_MS: i64 = 100;

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Step {
    /// Nothing yet; look again on a later tick.
    Wait,
    /// Ask the installer whether it has finished the load.
    Probe,
    /// The program is running as this pid.
    Loaded(u32),
    /// The installer finished without it.
    Failed,
    /// The deadline passed with no answer either way.
    TimedOut,
}

pub struct Follow {
    /// The terminal's children before the load.
    pub before: Vec<u32>,
    started_ms: i64,
    next_probe_ms: i64,
    probe_gap_ms: i64,
    next_look_ms: i64,
    served: bool,
}
