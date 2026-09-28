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

use super::say::{note, say};
use super::serve::serve;
use super::source::source;
use super::start_guest::start;
use super::{file::family::choose, guest::Guest};

pub fn run() -> ! {
    super::heap::init();
    note(b"[LINUX] personality up\n");
    if let Some((name, pin)) = super::request::install_request() {
        say(b"[LINUX] installing\n");
        let pkg = choose(&name);
        super::file::allow_shared_writes();
        // The exit code names the reason, which the store shows.
        let done = super::install::install(pkg, &pin);
        say(if done.is_ok() { b"[LINUX] installed\n" } else { b"[LINUX] install failed\n" });
        mk_exit(done.map_or_else(|why| why.code(), |()| 0))
    }
    let Some(launch) = source() else {
        // The terminal's request has already said what it looked for.
        if !super::terminal::started() {
            say(b"[LINUX] nothing installed under that name\n");
        }
        mk_exit(1)
    };
    let pid = mk_foreign_spawn(b"linux");
    if pid < 0 {
        say(b"[LINUX] no guest, errno ");
        let e = (-pid) as u32;
        let digits = [b'0' + (e / 10 % 10) as u8, b'0' + (e % 10) as u8, b'\n'];
        say(&digits);
        mk_exit(1)
    }
    super::call::mark_start();
    if !super::file::prepare_private() {
        say(b"[LINUX] no private directories, not starting\n");
        mk_exit(1)
    }
    let mut guest = Guest::new(pid as u32);
    guest.links = alloc::rc::Rc::new(super::guest::Links::load());
    let code = match start(&mut guest, &launch) {
        Ok(()) => {
            note(b"[LINUX] guest running\n");
            serve(guest)
        }
        Err(step) => {
            super::file::clear_private();
            say(step);
            mk_exit(2)
        }
    };
    super::file::clear_private();
    note(b"[LINUX] guest exited\n");
    mk_exit(code)
}
