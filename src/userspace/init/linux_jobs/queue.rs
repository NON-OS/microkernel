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

use alloc::string::String;
use alloc::vec::Vec;
use spin::Mutex;

/// Deeper than a person clicks, shallower than a caller in a loop can grow.
const DEPTH: usize = 8;

#[derive(PartialEq, Eq)]
pub(super) enum Job {
    /// A listing and the release asked for, which is empty for the default.
    Install(String, String),
    /// A listing whose installed package should be taken away.
    Uninstall(String),
    /// A package whose program should start, and how (run::How): whether
    /// its name is kept off the log, or it starts again with the network.
    Run(String, super::run::How),
}

pub(super) static PENDING: Mutex<Vec<Job>> = Mutex::new(Vec::new());

/// Queue an install. False when full or already queued, or when the stage
/// table is full of installs still moving, which the caller reports as busy:
/// an install whose stage could not be recorded would run unseen.
pub(crate) fn request_install(listing: String, release: String) -> bool {
    if !super::status::room_for(&listing) {
        return false;
    }
    let name = listing.clone();
    let queued = push(Job::Install(listing, release));
    if queued {
        super::status::set(&name, super::status::Stage::Queued);
    }
    queued
}

/// Queue an uninstall. False when full or already queued, or when the
/// stage table is full of jobs still moving, which the caller reports as
/// busy: a removal whose stage could not be recorded would run unseen.
pub(crate) fn request_uninstall(listing: String) -> bool {
    if !super::status::room_for(&listing) {
        return false;
    }
    let name = listing.clone();
    let queued = push(Job::Uninstall(listing));
    if queued {
        super::status::set(&name, super::status::Stage::Queued);
    }
    queued
}

/// Queue a run. False when full or already queued.
pub(crate) fn request_run(package: String) -> bool {
    push(Job::Run(package, super::run::How::Named))
}

/// A served launch (app_launch_rule.rs `Launch::Served`): the window asks to
/// start the core its SERVES row pairs it with. On the amnesic boot that
/// core's image is placed by nothing, and the hidden core listing cannot be
/// installed from the Marketplace, so a bare run would load an image that is
/// not there and the core would never start. So: when the paired `listing`
/// is not installed in this session, init installs it first, in its release's
/// box; when it is installed, the run is queued. The install is init's own,
/// for this one listing the kernel just admitted the pairing of, so the
/// window holds no AppInstall and no install of any other listing follows
/// from a launch. On demand at launch and session-scoped: nothing is
/// pre-placed and nothing outlives the boot. True only when the run was
/// queued. False while the install is queued, moving, or failed too recently
/// to be asked again (served_rule.rs), and when the queue or the stage table
/// is full: the caller answers busy and the window asks again, so an install
/// that takes minutes over the mixnet still ends in the core's run, and a
/// network outage does not start an install a tick.
pub(crate) fn request_served_run(listing: String, package: String) -> bool {
    super::served::request(listing, package)
}

/// Queue a run whose package name never reaches the log. False when full
/// or already queued.
pub(crate) fn request_quiet_run(package: String) -> bool {
    push(Job::Run(package, super::run::How::Quiet))
}

pub(super) fn push(job: Job) -> bool {
    let mut q = PENDING.lock();
    if q.len() >= DEPTH || q.contains(&job) {
        return false;
    }
    q.push(job);
    drop(q);
    super::super::instance_spawn::raise_drain();
    true
}

/// Whether a job could be done now; a contended lock is a push in flight.
/// An install or uninstall waiting for the one still moving is not one: it
/// can wait for an hour behind a model download, and init need not run at
/// a raised priority for that hour to find it still waiting.
pub(crate) fn has_pending() -> bool {
    let Some(q) = PENDING.try_lock() else { return true };
    if q.is_empty() {
        return false;
    }
    let runs = q.iter().any(|j| matches!(j, Job::Run(..)));
    drop(q);
    runs || !super::status::one_moving()
}

pub(super) fn take() -> Vec<Job> {
    core::mem::take(&mut *PENDING.lock())
}

/// Put back what had to wait, ahead of anything asked meanwhile, so installs
/// run in the order they were asked for.
pub(super) fn hold(waiting: Vec<Job>) {
    if waiting.is_empty() {
        return;
    }
    let mut q = PENDING.lock();
    let newer = core::mem::take(&mut *q);
    *q = super::order::requeue(waiting, newer);
}
