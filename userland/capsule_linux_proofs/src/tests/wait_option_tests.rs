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

//! wait4 and waitid read their options as the int they are, refuse what
//! kernel_wait4 and kernel_waitid refuse with EINVAL, and take what musl
//! passes for __WCLONE, an int whose sign fills the register's upper half.

use super::random::Regs;
use crate::linux::abi::errno::EINVAL;
use crate::linux::call::wait_opts::{
    wait4_options, waitid_options, WALL, WCLONE, WCONTINUED, WEXITED, WNOHANG, WNOTHREAD, WNOWAIT,
    WSTOPPED, WUNTRACED,
};

/// An int option word as a C library widens it into a register.
fn widened(options: u64) -> u64 {
    options as u32 as i32 as i64 as u64
}

#[test]
fn wait4_takes_what_a_program_passes_and_always_waits_for_an_exit() {
    let table: [(u64, u64); 6] = [
        (0, WEXITED),
        (WNOHANG, WNOHANG | WEXITED),
        (WNOHANG | WUNTRACED | WCONTINUED, WNOHANG | WUNTRACED | WCONTINUED | WEXITED),
        (WALL, WALL | WEXITED),
        (WNOTHREAD | WNOHANG, WNOTHREAD | WNOHANG | WEXITED),
        (WCLONE, WCLONE | WEXITED),
    ];
    for (options, want) in table {
        assert_eq!(wait4_options(widened(options)), Ok(want), "{options:#x}");
    }
}

#[test]
fn wclone_sign_extended_by_the_c_library_is_still_wclone() {
    assert_eq!(widened(WCLONE), 0xffff_ffff_8000_0000);
    assert_eq!(wait4_options(widened(WCLONE)), Ok(WCLONE | WEXITED));
    assert_eq!(waitid_options(widened(WCLONE | WEXITED)), Ok(WCLONE | WEXITED));
}

#[test]
fn wait4_refuses_waitid_options_and_unknown_bits() {
    for options in [WEXITED, WNOWAIT, WNOHANG | WEXITED, 0x10, 0x80_0000, 0x1000_0000] {
        assert_eq!(wait4_options(widened(options)), Err(EINVAL), "{options:#x}");
    }
}

#[test]
fn waitid_needs_something_to_wait_for() {
    for options in [0, WNOHANG, WNOWAIT, WNOHANG | WALL | WCLONE | WNOTHREAD] {
        assert_eq!(waitid_options(widened(options)), Err(EINVAL), "{options:#x}");
    }
    for options in [WEXITED, WSTOPPED, WCONTINUED, WEXITED | WNOWAIT | WNOHANG] {
        assert_eq!(waitid_options(widened(options)), Ok(options), "{options:#x}");
    }
    assert_eq!(waitid_options(WEXITED | 0x10), Err(EINVAL));
}

#[test]
fn random_options_depend_only_on_the_int() {
    let mut r = Regs::new(0x7761_6974_5f6f_7074);
    let wait4_known = WNOHANG | WUNTRACED | WCONTINUED | WNOTHREAD | WALL | WCLONE;
    let waitid_known = wait4_known | WEXITED | WNOWAIT;
    for _ in 0..200_000 {
        let raw = match r.small(3) {
            0 => r.arg(),
            1 => r.any() & (waitid_known | 0xffff_ffff_0000_0000),
            _ => widened(r.any() & waitid_known),
        };
        let low = u64::from(raw as u32);
        assert_eq!(wait4_options(raw), wait4_options(low));
        assert_eq!(waitid_options(raw), waitid_options(low));
        if let Ok(options) = wait4_options(raw) {
            assert_eq!(options & !(wait4_known | WEXITED), 0);
            assert_ne!(options & WEXITED, 0);
        }
        if let Ok(options) = waitid_options(raw) {
            assert_eq!(options & !waitid_known, 0);
            assert_ne!(options & (WEXITED | WSTOPPED | WCONTINUED), 0);
        }
    }
}
