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

//! Selectors, sizes, getrandom's flags, and random requests off a console.

use super::super::super::random::Regs;
use super::requests::{console, KINDS};
use crate::console::winsize::shown;
use crate::linux::abi::errno::EINVAL;
use crate::linux::call::ioctl_req::{flow_ok, flush_input, request, winsize, Req};
use crate::linux::call::random_flags::random_flags;

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
