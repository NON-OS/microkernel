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

//! Text between a guest and the desktop's clipboard, both ways, as
//! clip/desk.rs carries it: a selection the guest sets with focus read
//! through a pipe and copied there; the desktop's text offered while the
//! guest holds none, and written into the pipe a receive passes.

use nonos_capsule_wayland_proofs::client::*;
use nonos_app_skeleton::desk;
use nonos_capsule_wayland_proofs::input;

const MANAGER: u32 = 30;
const DEVICE: u32 = 31;
const SOURCE: u32 = 32;
const TEXT: &str = "text/plain;charset=utf-8";

/// A shown window with the keyboard, a data device, and the desktop's
/// clipboard service there.
fn ready() -> Client {
    desk::with(|d| d.clipboard_on = true);
    let mut c = toplevel(6);
    c.bind(40, &[("wl_data_device_manager", 3, MANAGER)]);
    c.req(MANAGER, 1, &[A::U(DEVICE), A::U(SEAT)]);
    c.req(SURFACE, 6, &[]);
    let serial = to(&c.events(), XDG_SURFACE)[0].word(0);
    c.req(XDG_SURFACE, 4, &[A::U(serial)]);
    c.buffer(SHM, POOL, BUFFER, (400, 300));
    c.req(SURFACE, 1, &[A::U(BUFFER), A::I(0), A::I(0)]);
    c.req(SURFACE, 6, &[]);
    c.events();
    // A key: the window was given input lately, which the desktop asks.
    input::tap(b'x' as u32, 0);
    c.events();
    c
}

fn pipe() -> (i32, i32) {
    let mut ends = [0i32; 2];
    // SAFETY: a new pipe for the test.
    assert_eq!(unsafe { libc::pipe(ends.as_mut_ptr()) }, 0);
    (ends[0], ends[1])
}

#[test]
fn a_selection_set_with_focus_is_copied_to_the_desktop() {
    let mut c = ready();
    c.req(MANAGER, 0, &[A::U(SOURCE)]);
    c.req(SOURCE, 0, &[A::S(TEXT)]);
    c.req(DEVICE, 1, &[A::U(SOURCE), A::U(1)]);
    // The source is asked to send its text, with a pipe's write end.
    let (bytes, fds) = c.s.recv(usize::MAX);
    assert_eq!(fds.len(), 1, "the write end");
    let mut at = 0;
    let mut sent = false;
    while at + 8 <= bytes.len() {
        let word = |i: usize| u32::from_le_bytes(bytes[i..i + 4].try_into().expect("a word"));
        let (object, head) = (word(at), word(at + 4));
        sent |= object == SOURCE && head & 0xFFFF == 1;
        at += (head >> 16) as usize;
    }
    assert!(sent, "wl_data_source.send");
    // SAFETY: the write end handed over: written and closed, as a source does.
    unsafe {
        libc::write(fds[0], b"from the guest".as_ptr().cast(), 14);
        libc::close(fds[0]);
    }
    c.events();
    assert_eq!(desk::with(|d| d.clipboard.clone()), b"from the guest");
}

#[test]
fn the_desktops_text_is_pasted_into_a_guest_that_holds_none() {
    let mut c = ready();
    desk::with(|d| d.clipboard = b"from the desktop".to_vec());
    // The keyboard entering again offers the desktop's selection.
    let window = c.s.guest.scene.win.window.expect("open");
    desk::keys_notice(window, false);
    desk::keys_notice(window, true);
    let evs = c.events();
    let offer = to(&evs, DEVICE).iter().find(|e| e.opcode == 5).expect("a selection").word(0);
    assert!(offer >= 0xff00_0000, "the desktop's offer");
    input::tap(b'v' as u32, 0);
    c.events();
    let (read, write) = pipe();
    c.req_fds(offer, 1, &[A::S(TEXT)], &[write]);
    c.events();
    let mut got = [0u8; 64];
    // SAFETY: the read end this test holds; the writer is closed by now.
    let n = unsafe { libc::read(read, got.as_mut_ptr().cast(), got.len()) };
    unsafe { libc::close(read) };
    assert_eq!(&got[..n.max(0) as usize], b"from the desktop");
}

#[test]
fn a_guest_that_lost_the_keyboard_reads_nothing_of_the_desktop() {
    let mut c = ready();
    let window = c.s.guest.scene.win.window.expect("open");
    desk::keys_notice(window, false);
    desk::keys_notice(window, true);
    let evs = c.events();
    let offer = to(&evs, DEVICE).iter().find(|e| e.opcode == 5).expect("a selection").word(0);
    input::tap(b'v' as u32, 0);
    c.events();
    // The user goes to another window and copies there, within FOCUS_MS.
    desk::keys_notice(window, false);
    c.events();
    desk::with(|d| d.clipboard = b"copied elsewhere".to_vec());
    let (read, write) = pipe();
    c.req_fds(offer, 1, &[A::S(TEXT)], &[write]);
    c.events();
    // SAFETY: the read end this test holds; non-blocking, so nothing is nothing.
    let n = unsafe {
        libc::fcntl(read, libc::F_SETFL, libc::O_NONBLOCK);
        let mut got = [0u8; 64];
        let n = libc::read(read, got.as_mut_ptr().cast(), got.len());
        libc::close(read);
        n
    };
    assert!(n <= 0, "nothing of the desktop's text, read {n}");
}
