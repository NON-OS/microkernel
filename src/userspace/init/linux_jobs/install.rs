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

//! An install once the market vouches for it, in the box its release is
//! sold with (I-C5, plan/30): the release's guest_policy and its listing
//! and release keys go to the install role as argv m, which checks them
//! and binds them to the program it records. A release the market says has
//! no policy (a vendor program) is sent "-"; one whose policy the market
//! does not vouch for, or a market that cannot say, is refused, never
//! installed without its box.

use crate::sys::serial::{print, println};
use crate::userspace::capsule_linux::{package_arg, spawn_install};

use super::status::Stage;

pub(super) fn install(listing: &str, release: &str) {
    let (release, direct) = super::direct::split(listing, release);
    if direct {
        println(b"[LINUX-INSTALL] the person chose a direct download of this tier's model");
    }
    if super::super::app_choice::linux_off() {
        super::status::set(listing, Stage::Refused);
        return println(b"[LINUX-INSTALL] Linux turned off at setup, refused");
    }
    let Some(name) = listing.strip_prefix("linux.").and_then(package_arg) else {
        return refuse(listing, b"[LINUX-INSTALL] no package name the personality takes, refused ");
    };
    let (hash, policy) = match super::vouched::vouched(listing, release) {
        Ok(v) => v,
        Err(said) => return refuse(listing, said),
    };
    match spawn_install(&name, &hash, policy.as_deref(), direct) {
        Ok(pid) => {
            super::status::set(listing, Stage::Running(pid));
            print(if policy.is_some() {
                b"[LINUX-INSTALL] started, in its release's box "
            } else {
                b"[LINUX-INSTALL] started "
            });
            println(listing.as_bytes());
        }
        Err(e) => {
            // Which preflight check refused the installer, not only that one did.
            println(alloc::format!("[LINUX-INSTALL] installer refused: {e:?}").as_bytes());
            refuse(listing, b"[LINUX-INSTALL] refused ");
        }
    }
}

fn refuse(listing: &str, said: &[u8]) {
    super::status::set(listing, Stage::Refused);
    print(said);
    println(listing.as_bytes());
}
