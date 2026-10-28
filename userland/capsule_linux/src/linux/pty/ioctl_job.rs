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

//! The requests about a pair's sessions and groups, and the master's own:
//! its number (ptsname), its lock (unlockpt) and a slave opened from it.
//! A slave answers the group and session requests only for a caller whose
//! session it controls, as Linux's tiocgpgrp and tiocspgrp require.

use crate::linux::abi::errno;
use crate::linux::guest::Guest;
use crate::linux::serve::guest_pid;

use super::cell::with;
use super::ioctl::put;
use super::ioctl_ctty::{ctty, notty, on};

const TCSBRK: u64 = 0x5409;
const TCXONC: u64 = 0x540A;
const TIOCSCTTY: u64 = 0x540E;
const TIOCGPGRP: u64 = 0x540F;
const TIOCSPGRP: u64 = 0x5410;
const TIOCOUTQ: u64 = 0x5411;
const TIOCNOTTY: u64 = 0x5422;
const TCSBRKP: u64 = 0x5425;
const TIOCGSID: u64 = 0x5429;
const TIOCGPTPEER: u64 = 0x5441;
const TIOCGPTN: u64 = 0x8004_5430;
const TIOCSPTLCK: u64 = 0x4004_5431;
const TIOCGPTLCK: u64 = 0x8004_5439;

/// The answer to `req` on pair `n`; None for one no terminal answers.
pub(super) fn ioctl(guest: &mut Guest, n: u32, master: bool, req: u64, arg: u64) -> Option<u64> {
    let (session, fg, locked) = with(|t| t.get(n).map(|p| (p.session, p.fg, p.locked)))?;
    let ours = master || (session != 0 && session == guest.sid);
    let int =
        |guest: &Guest| guest.read(arg, 4).map(|r| i32::from_le_bytes([r[0], r[1], r[2], r[3]]));
    Some(match req {
        TIOCGPTN if master => put(guest, arg, &n.to_le_bytes()),
        TIOCGPTLCK if master => put(guest, arg, &u32::from(locked).to_le_bytes()),
        TIOCSPTLCK if master => {
            int(guest).map_or(errno::fail(errno::EFAULT), |v| on(n, |p| p.locked = v != 0))
        }
        TIOCGPTPEER if master => super::open::slave(guest, n, arg),
        TIOCSCTTY => ctty(guest, n, session),
        TIOCNOTTY if session == guest.sid && session != 0 => notty(guest, n),
        TIOCGPGRP if ours => put(guest, arg, &fg.to_le_bytes()),
        TIOCSPGRP if ours => match int(guest) {
            Some(g) if g < 0 => errno::fail(errno::EINVAL),
            Some(g) => on(n, |p| p.fg = g as u32),
            None => errno::fail(errno::EFAULT),
        },
        TIOCGSID if ours && session != 0 => put(guest, arg, &guest_pid(session).to_le_bytes()),
        TIOCOUTQ => put(guest, arg, &0i32.to_le_bytes()),
        TCSBRK | TCSBRKP | TCXONC => errno::ok(0),
        TIOCNOTTY | TIOCGPGRP | TIOCSPGRP | TIOCGSID => errno::fail(errno::ENOTTY),
        _ => return None,
    })
}
