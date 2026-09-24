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

use nonos_libc::{heap_init, mk_debug, mk_exit, mk_foreign_spawn, mk_foreign_start};

use super::guest::Guest;
use super::image;
use super::serve::serve;

/// The stack top a guest wakes on, below the personality's own mappings
/// and above everything it maps for itself.
const STACK_TOP: u64 = 0x0000_7FFF_F000;

/// The proof image, until the file layer can fetch one: a static Linux
/// executable built for x86_64, embedded so the first runs need no disk.
static IMAGE: &[u8] = include_bytes!("../../guests/hello.elf");

pub fn run() -> ! {
    let _ = heap_init();
    say(b"[LINUX] personality up\n");
    let pid = mk_foreign_spawn(b"linux");
    if pid < 0 {
        say(b"[LINUX] no guest: refused\n");
        mk_exit(1)
    }
    let mut guest = Guest::new(pid as u32);
    let entry = match image::load(&guest, IMAGE) {
        Ok(entry) => entry,
        Err(_) => {
            say(b"[LINUX] image refused\n");
            mk_exit(2)
        }
    };
    let Some(rsp) = stack(&guest, entry) else {
        say(b"[LINUX] stack refused\n");
        mk_exit(3)
    };
    if mk_foreign_start(guest.pid, entry, rsp) < 0 {
        say(b"[LINUX] start refused\n");
        mk_exit(4)
    }
    say(b"[LINUX] guest running\n");
    let code = serve(&mut guest);
    say(b"[LINUX] guest exited\n");
    mk_exit(code)
}

fn stack(guest: &Guest, entry: u64) -> Option<u64> {
    guest.map(STACK_TOP - 0x10000, 0x10000, true, false);
    image::build(guest, STACK_TOP, entry)
}

fn say(line: &[u8]) {
    let _ = mk_debug(line.as_ptr(), line.len());
}
