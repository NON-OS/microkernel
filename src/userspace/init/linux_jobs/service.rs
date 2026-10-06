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

use crate::security::market_capsule::client::{queued_get_release, queued_install_ready};
use crate::sys::serial::{print, println};
use crate::userspace::capsule_linux::{package_arg, spawn_install, spawn_uninstall};

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
            Job::Install(listing, release) => install(&listing, &release),
            Job::Uninstall(listing) => uninstall(&listing),
            Job::Run(package, quiet) => super::run::run(&package, quiet),
        }
    }
    hold(waiting);
}

fn install(listing: &str, release: &str) {
    /* A tier asked to download its model directly (`direct`). */
    let (release, direct) = super::direct::split(listing, release);
    if direct {
        println(b"[LINUX-INSTALL] the person chose a direct download of this tier's model");
    }
    if super::super::app_choice::linux_off() {
        super::status::set(listing, Stage::Refused);
        return println(b"[LINUX-INSTALL] Linux turned off at setup, refused");
    }
    /*
     * A listing whose package name the personality cannot take. Said as a
     * refusal: returning without a stage left the store showing Queued for
     * the rest of the boot.
     */
    let Some(name) = listing.strip_prefix("linux.").and_then(package_arg) else {
        super::status::set(listing, Stage::Refused);
        print(b"[LINUX-INSTALL] no package name the personality takes, refused ");
        return println(listing.as_bytes());
    };
    /*
     * The store showed the listing as ready, and that was its word. The
     * market's own verdict is asked for again here, and the release's
     * package hash goes to the installer, which refuses any other bytes.
     */
    let asked = queued_install_ready(listing, release);
    let ready = asked.as_ref().is_ok_and(|r| r.install_ready);
    let pinned = queued_get_release(listing, release).map(|r| r.package_hash);
    super::why::say(&asked, &pinned);
    let said: &[u8] = match (ready, pinned.ok()) {
        (true, Some(hash)) => match spawn_install(&name, &hash, direct) {
            Ok(pid) => {
                super::status::set(listing, Stage::Running(pid));
                b"[LINUX-INSTALL] started "
            }
            Err(e) => {
                super::status::set(listing, Stage::Refused);
                // Which preflight check refused the installer, not only that one did.
                println(alloc::format!("[LINUX-INSTALL] installer refused: {e:?}").as_bytes());
                b"[LINUX-INSTALL] refused "
            }
        },
        _ => {
            super::status::set(listing, Stage::Refused);
            b"[LINUX-INSTALL] not ready, refused "
        }
    };
    print(said);
    println(listing.as_bytes());
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
