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

//! A store run whose program is for a terminal, not a window, asks for it to
//! run in the Terminal: `linux <package>`, as if typed there. Without it the
//! store's Open started such a program with nothing to show it on.
//!
//! Only the run the kernel started for the store may ask: it is known by the
//! pid the kernel stamps on the call, against the pid registered as
//! app.linux.run, a name no other capsule may take. The body is the package
//! name alone, and the shell writes the line (open_arg::linux).

use crate::protocol::{Request, E_INVAL, E_PERM};
use crate::server::handlers::launcher_request::lookup_pid;
use crate::server::handlers::launcher_request::LaunchOutcome;
use crate::server::respond;
use crate::state::open_arg::linux_command;
use crate::state::Context;

const RUN: &[u8] = b"app.linux.run";
/// The Terminal could not be brought up.
const E_IO: i32 = -5;

pub fn handle(ctx: &mut Context, sender_pid: u32, req: &Request, body: &[u8], tx: &mut [u8]) {
    if sender_pid == 0 || lookup_pid(RUN) != Some(sender_pid) {
        let _ = respond::status(sender_pid, req, E_PERM, tx);
        return;
    }
    let Some(line) = linux_command(body) else {
        let _ = respond::status(sender_pid, req, E_INVAL, tx);
        return;
    };
    let status = match super::hand_over::hand_command(ctx, line) {
        LaunchOutcome::Failed => E_IO,
        _ => 0,
    };
    let _ = respond::status(sender_pid, req, status, tx);
}
