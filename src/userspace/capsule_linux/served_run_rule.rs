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

//! The one served run: a run a capsule SERVES lists launched, which ends with
//! that capsule's process as a terminal's runs end with the terminal. Pure,
//! so kernel_proofs holds it as it ships (served_run.rs keeps the one).

use alloc::string::String;

#[derive(Clone, Debug, PartialEq, Eq)]
pub(crate) struct Served {
    /// The launcher's process; None once it has exited.
    pub(crate) launcher: Option<u32>,
    /// The package init was asked to run.
    pub(crate) package: String,
    /// The run role process holding it: None while it is queued, or between
    /// a start and its start again with the network (run_net.rs).
    pub(crate) run: Option<u32>,
}

/// `pid` started as a run role for `package`. True when it must end at once:
/// it is the served run, and its launcher left before it started.
pub(crate) fn started(slot: &mut Option<Served>, package: &str, pid: u32) -> bool {
    let Some(s) = slot.as_mut().filter(|s| s.package == package) else { return false };
    if s.launcher.is_none() {
        return true;
    }
    s.run = Some(pid);
    false
}

/// The run `pid` asked to start again with the network: queued again.
pub(crate) fn restarting(slot: &mut Option<Served>, pid: u32) {
    if let Some(s) = slot.as_mut().filter(|s| s.run == Some(pid)) {
        s.run = None;
    }
}

/// `pid` is exiting; the run that ends with it. The launcher's exit ends
/// the run and keeps the record, so a start still queued ends as it starts;
/// the run's own exit takes the record.
pub(crate) fn exited(slot: &mut Option<Served>, pid: u32) -> Option<u32> {
    let s = slot.as_mut()?;
    if s.launcher == Some(pid) {
        s.launcher = None;
        return s.run.take();
    }
    if s.run == Some(pid) {
        *slot = None;
    }
    None
}
