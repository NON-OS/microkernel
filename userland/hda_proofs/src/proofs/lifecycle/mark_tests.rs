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

//! The console lines bring-up and teardown write, built on the stack: each
//! is cut to its buffer, whatever is put in it, and the numbered give-up
//! mark fits its own at the widest code and return value.

use nonos_libc::logged;

use crate::error::HdaError;
use crate::setup::{mark_setup_fail, Line};

/// The lines logged while `f` runs.
fn lines(f: impl FnOnce()) -> Vec<String> {
    let from = logged().len();
    f();
    logged()[from..].to_vec()
}

#[test]
fn a_line_is_written_as_built_in_hex_and_decimal() {
    let l =
        lines(|| Line::new("[HDA] controller ").hex(0x8086, 4).s(":").dec(4_294_967_295).emit());
    assert_eq!(l, ["[HDA] controller 8086:4294967295\n"]);
}

#[test]
fn a_line_longer_than_its_buffer_is_cut_not_overrun() {
    let long = "x".repeat(200);
    let l = lines(|| Line::new(&long).hex(u32::MAX, 8).dec(u32::MAX).emit());
    assert_eq!(l, [format!("{}\n", "x".repeat(119))], "the line was not cut at its buffer");
}

#[test]
fn the_give_up_mark_fits_its_buffer_at_the_widest_code_and_return_value() {
    let l = lines(|| mark_setup_fail(HdaError::BrokerCallFailed(i64::MIN)));
    assert_eq!(l, ["[HDA] setup-fail code=3 rc=-9223372036854775808\n"]);
    let l = lines(|| mark_setup_fail(HdaError::EngineStopTimeout));
    assert_eq!(l, ["[HDA] setup-fail code=13\n"]);
}
