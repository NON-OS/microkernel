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

//! The store's Open of a program made for a terminal. A run started by the
//! store has no terminal, so such a program used to start with nothing to
//! show it on and nothing to type into. Its box says what it is: TTY and no
//! WAYLAND. That run asks the desktop shell to run `linux <package>` in the
//! Terminal instead, and ends before its guest starts. The shell takes the
//! ask only from app.linux.run and writes the line itself
//! (capsule_desktop_shell server/handlers/run_linux.rs).

use nonos_app_skeleton::discover::lookup_service;
use nonos_app_skeleton::wire::{call_payload, HDR_LEN};
use nonos_linux_policy::mask::{TTY, WAYLAND};

use super::guest::Guest;
use super::run_mode::Mode;

// Hand-synced with capsule_desktop_shell protocol::{MAGIC, OP_RUN_LINUX}.
const NDSH: u32 = 0x4E44_5348;
const OP_RUN_LINUX: u16 = 0x0010;

/// Before run_net, since the shell knows this run as app.linux.run only. A
/// terminal program the Terminal took ends this run here, as a start that
/// did what was asked; one it did not take is a start refused, and said.
pub(super) fn to_terminal(guest: &Guest) -> Result<(), &'static [u8]> {
    if !wants_terminal(guest) {
        return Ok(());
    }
    if !hand_over() {
        return Err(&b"[LINUX] a terminal program, and no Terminal took it\n"[..]);
    }
    super::say::say(b"[LINUX] a terminal program: run in the Terminal\n");
    super::file::clear_private();
    super::start_stop::stop("run in the Terminal", 0)
}

/// Whether this run is the store's window run of a terminal program.
fn wants_terminal(guest: &Guest) -> bool {
    let window = matches!(super::request::run_request(), Some((_, Mode::Window)));
    let tty = super::app::granted(guest, TTY) && !super::app::granted(guest, WAYLAND);
    window && tty && !super::terminal::started()
}

/// Ask the shell to run the package in the Terminal; true when it took it.
fn hand_over() -> bool {
    let Some((name, _)) = super::request::run_request() else {
        return false;
    };
    let Some(shell) = lookup_service(b"desktop_shell") else {
        return false;
    };
    let mut rx = [0u8; HDR_LEN + 4];
    call_payload(shell.port, NDSH, OP_RUN_LINUX, 1, name.as_bytes(), &mut rx).is_ok()
}
