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

//! wait4's options: what programs pass is taken, what it does not know refused.

use crate::linux::abi::errno::EINVAL;
use crate::linux::call::wait_opts::{
    wait4_options, waitid_options, WALL, WCLONE, WCONTINUED, WEXITED, WNOHANG, WNOTHREAD, WNOWAIT,
    WUNTRACED,
};

/// An int option word as a C library widens it into a register.
pub(super) fn widened(options: u64) -> u64 {
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
