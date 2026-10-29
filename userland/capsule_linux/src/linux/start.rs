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

//! Bring one Linux program up and stay with it until it ends.

use nonos_libc::{mk_exit, mk_foreign_spawn};

pub(super) use super::say::say;
use super::serve::serve;
use super::source::source;
use super::start_guest::start;
pub(super) use super::start_say::routine;
use super::guest::Guest;

pub fn run() -> ! {
    super::start_say::clock::begin();
    super::heap::init();
    routine(b"[LINUX] personality up\n");
    if let Some(asked) = super::request_install::install_request() {
        super::start_install::install(asked)
    }
    if let Some(name) = super::request::uninstall_request() {
        super::start_install::uninstall(&name)
    }
    /* source() has said why when it starts nothing. */
    let Some(launch) = source() else { super::start_stop::stop("no program to run", 1) };
    if !super::file::prepare_private() {
        say(b"[LINUX] no private directories, not starting\n");
        super::start_stop::stop("no private folders", 1)
    }
    // The role first: a program that belongs in the network role starts
    // again there before anything here runs for it (run_net::early).
    super::run_net::early(&launch);
    // GTK's tools run before the program's guest exists: one made beside it
    // joins its seal family and starts it, and the seal then refuses the
    // program's note (seal_note.rs), so the run would not be kept.
    super::start_say::clock::prep(|| super::gtk_prep::prepare(&launch.bytes));
    let pid = mk_foreign_spawn(b"linux");
    if pid < 0 {
        super::start_stop::no_guest(pid)
    }
    super::call::mark_start();
    let mut guest = Guest::new(pid as u32);
    guest.links = alloc::rc::Rc::new(super::guest::Links::load());
    let code = match start(&mut guest, launch) {
        Ok(()) => {
            routine(b"[LINUX] guest running\n");
            serve(guest)
        }
        Err(step) => {
            super::file::clear_private();
            say(step);
            super::start_stop::stop(super::start_stop::why(step), 2)
        }
    };
    super::file::clear_private();
    routine(b"[LINUX] guest exited\n");
    mk_exit(code)
}
