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
use super::{file::family::choose, guest::Guest};

pub fn run() -> ! {
    super::heap::init();
    routine(b"[LINUX] personality up\n");
    if let Some(asked) = super::request_install::install_request() {
        super::start_install::install(asked)
    }
    if let Some(name) = super::request::uninstall_request() {
        say(b"[LINUX] uninstalling\n");
        let pkg = choose(&name);
        super::file::allow_shared_writes();
        // The exit code names the reason, which the store shows.
        let done = super::install::uninstall(pkg);
        say(if done.is_ok() { b"[LINUX] uninstalled\n" } else { b"[LINUX] uninstall failed\n" });
        mk_exit(done.map_or_else(|why| why.code(), |()| 0))
    }
    /* source() has said why when it starts nothing. */
    let Some(launch) = source() else { super::start_stop::stop("no program to run", 1) };
    let pid = mk_foreign_spawn(b"linux");
    if pid < 0 {
        say(b"[LINUX] no guest, errno ");
        let e = (-pid) as u32;
        let digits = [b'0' + (e / 10 % 10) as u8, b'0' + (e % 10) as u8, b'\n'];
        say(&digits);
        super::start_stop::stop("the kernel gave no guest", 1)
    }
    super::call::mark_start();
    if !super::file::prepare_private() {
        say(b"[LINUX] no private directories, not starting\n");
        super::start_stop::stop("no private folders", 1)
    }
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
