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

//! How a handler is entered: with the direction flag clear, as a function
//! is, and never with a frame written below the alternate stack it is on.

use super::sigframe_tests::{regs, RSP};
use crate::sigframe::{build, returned};

/// rflags in the register word order, and its direction flag.
const RFLAGS: usize = 17;
const DF: u64 = 0x400;

/// A 32 KiB alternate stack well away from the thread's own stack.
const ALT: (u64, u64) = (0x7000_0000_0000, 0x8000);

#[test]
fn the_handler_starts_with_the_direction_flag_clear() {
    let mut saved = regs();
    saved[RFLAGS] = 0x246 | DF;
    let (_, buf, enter) = build(&saved, 1, 2, 3, 0, None, false).expect("frame");
    assert_eq!(enter[RFLAGS] & DF, 0, "the handler runs with DF clear");
    assert_eq!(enter[RFLAGS] & !DF, saved[RFLAGS] & !DF, "and every other flag as it was");
    /* The interrupted code gets its own DF back through rt_sigreturn. */
    assert_eq!(returned(&buf[8..]).unwrap()[RFLAGS], saved[RFLAGS]);
}

#[test]
fn a_frame_nested_on_the_alternate_stack_without_sa_onstack_is_bounded_too() {
    let mut saved = regs();
    /* In a handler on the alternate stack, too near its base for another. */
    saved[RSP] = ALT.0 + 0x200;
    assert!(build(&saved, 1, 2, 3, 0, Some(ALT), false).is_none());
    /* With room left it nests there, below the red zone. */
    saved[RSP] = ALT.0 + 0x6000;
    let (frame, _, _) = build(&saved, 1, 2, 3, 0, Some(ALT), false).expect("frame");
    assert!(frame < saved[RSP] - 128 && frame > ALT.0);
}
