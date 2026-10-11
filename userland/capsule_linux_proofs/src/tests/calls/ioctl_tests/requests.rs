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

//! The descriptor kinds, and the file requests every one answers.

use crate::linux::call::ioctl_req::{request, Req};
use crate::linux::guest::Kind;

pub(super) const KINDS: [Kind; 16] = [
    Kind::Free,
    Kind::Stdin,
    Kind::Stdout,
    Kind::Stderr,
    Kind::File,
    Kind::Dir,
    Kind::Socket,
    Kind::Unix,
    Kind::Memfd,
    Kind::Epoll,
    Kind::Timer,
    Kind::Pipe,
    Kind::Resolver,
    Kind::Event,
    Kind::Device,
    Kind::Signal,
];

/// TCGETS, TCSETS, TCSETSW, TCSETSF, TCSBRK, TCXONC, TCFLSH, TIOCGPGRP,
/// TIOCSPGRP, TIOCOUTQ, TIOCGWINSZ, TIOCSWINSZ, TCSBRKP, TIOCGSID.
pub(super) const TTY: [u64; 14] = [
    0x5401, 0x5402, 0x5403, 0x5404, 0x5409, 0x540A, 0x540B, 0x540F, 0x5410, 0x5411, 0x5413, 0x5414,
    0x5425, 0x5429,
];

/// TIOCSCTTY, TIOCSTI, TIOCNOTTY, TIOCGPTN, FIOASYNC, and two that are no
/// request at all.
pub(super) const NEVER: [u64; 7] =
    [0x540E, 0x5412, 0x5422, 0x8004_5430, 0x5452, 0x1234, 0xDEAD_BEEF];

pub(super) fn console(kind: Kind) -> bool {
    matches!(kind, Kind::Stdin | Kind::Stdout | Kind::Stderr)
}

#[test]
fn the_file_requests_are_every_descriptors() {
    for kind in KINDS {
        for on in [false, true] {
            assert_eq!(request(kind, on, 0x5451), Req::CloseOnExec(true));
            assert_eq!(request(kind, on, 0x5450), Req::CloseOnExec(false));
            assert_eq!(request(kind, on, 0x5421), Req::NonBlock);
            assert_eq!(request(kind, on, 0x541B), Req::Queued);
        }
    }
}
