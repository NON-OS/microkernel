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

//! ioctl: every descriptor answers the file requests, only a console on a
//! terminal answers the terminal's, and every other request on every other
//! descriptor is ENOTTY; TCFLSH and TCXONC refuse a bad selector EINVAL;
//! TIOCSWINSZ holds until the terminal itself changes size; getrandom
//! refuses the flags Linux refuses.

use super::random::Regs;
use crate::console::winsize::shown;
use crate::linux::abi::errno::EINVAL;
use crate::linux::call::ioctl_req::{flow_ok, flush_input, request, winsize, Req};
use crate::linux::call::random_flags::random_flags;
use crate::linux::guest::Kind;

const KINDS: [Kind; 16] = [
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
const TTY: [u64; 14] = [
    0x5401, 0x5402, 0x5403, 0x5404, 0x5409, 0x540A, 0x540B, 0x540F, 0x5410, 0x5411, 0x5413, 0x5414,
    0x5425, 0x5429,
];
/// TIOCSCTTY, TIOCSTI, TIOCNOTTY, TIOCGPTN, FIOASYNC, and two that are no
/// request at all.
const NEVER: [u64; 7] = [0x540E, 0x5412, 0x5422, 0x8004_5430, 0x5452, 0x1234, 0xDEAD_BEEF];

fn console(kind: Kind) -> bool {
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

#[test]
fn only_a_console_on_a_terminal_answers_the_terminal_requests() {
    for kind in KINDS {
        for on in [false, true] {
            for tty in TTY {
                let got = request(kind, on, tty);
                assert_eq!(got == Req::NotTty, !(on && console(kind)), "{tty:#x} on {on}");
            }
            for never in NEVER {
                assert_eq!(request(kind, on, never), Req::NotTty, "{never:#x}");
            }
        }
    }
    assert_eq!(request(Kind::Stdout, true, 0x5404), Req::SetTermios { flush: true });
    assert_eq!(request(Kind::Stdin, true, 0x5414), Req::SetWinsize);
    // ash sets the terminal's foreground group to its own at startup.
    assert_eq!(request(Kind::Stdin, true, 0x5410), Req::SetPgrp);
    // The request is an unsigned int: the upper half is not part of it.
    assert_eq!(request(Kind::Stdin, true, (7 << 32) | 0x5401), Req::GetTermios);
}

#[test]
fn a_bad_flush_or_flow_selector_is_einval() {
    assert_eq!(flush_input(0), Ok(true), "TCIFLUSH");
    assert_eq!(flush_input(1), Ok(false), "TCOFLUSH");
    assert_eq!(flush_input(2), Ok(true), "TCIOFLUSH");
    assert_eq!(flush_input(3), Err(EINVAL));
    assert_eq!(flush_input(u64::MAX), Err(EINVAL));
    assert!((0..4).all(|a| flow_ok(a).is_ok()));
    assert_eq!(flow_ok(4), Err(EINVAL));
    assert_eq!(winsize(&[50, 0, 132, 0, 9, 9, 9, 9]), Some((50, 132)));
    assert_eq!(winsize(&[1, 2, 3]), None);
}

#[test]
fn a_size_a_guest_sets_holds_until_the_terminal_changes() {
    let term = (24, 80);
    assert_eq!(shown(None, term), term);
    assert_eq!(shown(Some(((50, 132), term)), term), (50, 132));
    assert_eq!(shown(Some(((50, 132), term)), (30, 100)), (30, 100), "a resize is newer");
}

#[test]
fn getrandom_refuses_the_flags_linux_refuses() {
    for ok in [0, 1, 2, 3, 4, 5] {
        assert_eq!(random_flags(ok), Ok(()), "{ok}");
    }
    for bad in [6, 7, 8, 0x80, u32::MAX as u64] {
        assert_eq!(random_flags(bad), Err(EINVAL), "{bad}");
    }
    assert_eq!(random_flags(1 << 32), Ok(()), "the flags are an unsigned int");
}

#[test]
fn random_requests_never_reach_a_terminal_request_off_a_console() {
    let mut r = Regs::new(0x696f_6374_6c5f_7474);
    for _ in 0..100_000 {
        let kind = KINDS[r.small(16) as usize];
        let (on, req) = (r.small(2) == 0, r.arg());
        let got = request(kind, on, req);
        let file = matches!(got, Req::CloseOnExec(_) | Req::NonBlock | Req::Queued);
        assert!(file || got == Req::NotTty || (on && console(kind)));
        let _ = (flush_input(r.arg()), flow_ok(r.arg()), random_flags(r.arg()));
    }
}
