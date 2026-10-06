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

use alloc::vec;

use nonos_libc::{heap_init, mk_exit, HeapError};

use crate::app::App;
use crate::discover::require_peers;
use crate::log_line::{say, Line};

use super::boot::boot;
use super::dispatch::DELIVERY_LEN;
use super::ephemeral::is_window_instance;
use super::fail::{fail, Who};
use super::frame_loop::frame_loop;
use super::idle;
use super::no_window::{no_window, NoWindow};
use super::open_peers::open_peers;

pub fn run<A: App, F: Fn() -> A>(build: F) -> ! {
    match heap_init() {
        Ok(()) | Err(HeapError::AlreadyInitialized) => {}
        Err(_) => fail(1, Who::Pid, b"no heap"),
    }
    /* An on-demand instance exits when closed; a base app returns to idle. */
    let ephemeral = is_window_instance();
    let mut request_id: u32 = 1;
    let mut rx = vec![0u8; DELIVERY_LEN.max(256)];
    let mut peers = None;
    loop {
        idle::wait(&mut rx);
        /* A base app stays for the next open; an instance nobody can see
         * would only hold its slot, so it exits. */
        let Some(peers) = open_peers(&mut peers, || require_peers().ok()) else {
            if no_window(ephemeral) == NoWindow::Exit {
                fail(2, Who::Pid, b"no window: the desktop's services did not answer");
            }
            continue;
        };
        let app = build();
        let title = app.manifest().title;
        let mut booted = match boot(app, peers, &mut request_id) {
            Ok(b) => b,
            /* An instance with no window would hold its slot for nothing
             * (no_window.rs): it ends, and the next click spawns afresh. */
            Err(why) if no_window(ephemeral) == NoWindow::Exit => {
                fail(3, Who::Titled(title), &no_window_why(why))
            }
            /* A base app waits for the next open, and says this one failed. */
            Err(why) => {
                let _ =
                    say(&Line::new(b"APP-FAIL").text(title).text(b": ").text(&no_window_why(why)));
                continue;
            }
        };
        frame_loop(&mut booted, &mut rx, peers, &mut request_id);
        /* Closed: the kernel zeroizes an exiting instance and frees its slot. */
        if ephemeral {
            mk_exit(0);
        }
    }
}

/* "no window: <the step that failed>", in the words boot gave. */
fn no_window_why(step: &str) -> alloc::vec::Vec<u8> {
    [&b"no window: "[..], step.as_bytes()].concat()
}
