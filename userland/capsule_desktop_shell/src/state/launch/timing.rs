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

//! How long a launch is followed, how often it is looked for and probed,
//! and what a turn does with it.

/// The most a launch is followed: the budget the blocking call had.
pub const DEADLINE_MS: i64 = 30_000;
/// The first probe, then twice the gap each time up to the cap. Every probe
/// the installer is too busy to answer keeps a place in its reply queue until
/// it does, and the kernel gives one caller eight such places; this schedule
/// holds six at most over the whole deadline, the load's own one beside them.
pub const FIRST_PROBE_MS: i64 = 500;
pub const PROBE_GAP_MAX_MS: i64 = 8_000;
/// How often the registry is asked for the app.
pub const LOOK_GAP_MS: i64 = 50;

/// What the shell should do on this turn.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Step {
    /// Nothing yet; look again on a later turn.
    Wait,
    /// Ask the installer whether it has finished the load.
    Probe,
    /// The app is up: hand it its focus frame.
    Started(u32),
    /// The installer finished without the app, or the deadline passed.
    Failed,
}
