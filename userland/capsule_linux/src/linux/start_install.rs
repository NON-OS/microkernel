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

//! An install request, carried out; the exit code names the reason, which
//! the store shows.

use nonos_libc::mk_exit;

use super::file::family::choose;
use super::install::{install_from, Why};
use super::request_install::Install;
use super::say::say;

pub(super) fn install(asked: Install) -> ! {
    say(b"[LINUX] installing\n");
    // A USB boot loads the store from the stick a minute or more after the
    // desktop is up: an install asked for before then waits for it rather
    // than reading a store that is not there yet.
    let _ = super::settle::wait_settled();
    let pkg = choose(&asked.name);
    super::file::allow_shared_writes();
    // An m this capsule cannot read never installs the program in another box.
    let done = match asked.market {
        Ok(market) => install_from(pkg, &asked.pin, market.as_ref()),
        Err(_) => Err(Why::POLICY),
    };
    say(if done.is_ok() { b"[LINUX] installed\n" } else { b"[LINUX] install failed\n" });
    mk_exit(done.map_or_else(|why| why.code(), |()| 0))
}
