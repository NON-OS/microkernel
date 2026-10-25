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

use crate::sys::serial::{print, println};
use crate::userspace::capsule_linux::{package_arg, spawn_uninstall};

use super::queue::{hold, take, Job};
use super::status::Stage;

/// Perform every queued job that can run now.
///
/// The personality runs one installer at a time: its endpoint has one owner,
/// so a second started while the first downloads a model was refused with
/// EndpointCollision and the store said only that the system refused. An
/// install or uninstall asked for while another is moving now stays queued,
/// in order, and starts on the pass after the one before it ends; its stage
/// reads Queued meanwhile, which the store shows as waiting behind it.
pub(crate) fn service() {
    let mut waiting = alloc::vec::Vec::new();
    for job in take() {
        match job {
            Job::Install(..) | Job::Uninstall(_) if super::status::one_moving() => waiting.push(job),
            Job::Install(listing, release) => super::install::install(&listing, &release),
            Job::Uninstall(listing) => uninstall(&listing),
            Job::Run(package, how) => super::run::run(&package, how),
        }
    }
    hold(waiting);
}

/* Nothing is fetched or checked: the personality removes only the files its
 * own install recorded for the package. */
fn uninstall(listing: &str) {
    if super::super::app_choice::linux_off() {
        super::status::set(listing, Stage::Refused);
        return println(b"[LINUX-UNINSTALL] Linux turned off at setup, refused");
    }
    let Some(name) = listing.strip_prefix("linux.").and_then(package_arg) else {
        super::status::set(listing, Stage::Refused);
        print(b"[LINUX-UNINSTALL] no package name the personality takes, refused ");
        return println(listing.as_bytes());
    };
    let said: &[u8] = match spawn_uninstall(&name) {
        Ok(pid) => {
            super::status::set(listing, Stage::Removing(pid));
            b"[LINUX-UNINSTALL] started "
        }
        Err(e) => {
            super::status::set(listing, Stage::Refused);
            println(alloc::format!("[LINUX-UNINSTALL] refused: {e:?}").as_bytes());
            b"[LINUX-UNINSTALL] refused "
        }
    };
    print(said);
    println(listing.as_bytes());
}
