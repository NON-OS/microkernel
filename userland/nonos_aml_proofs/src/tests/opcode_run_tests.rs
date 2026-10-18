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

// NONOS Operating System (AGPL-3.0-or-later)
//! F-104: the rows that say more than "run", run. Noop and BreakPoint pass
//! over; Fatal is said and stops; Stall and Sleep past the time budget
//! fault; Unload of anything but a table handle faults; a Debug store is
//! said and a Debug read refused; and a byte in no row and no name is
//! refused when the method that holds it is called.

use crate::enc::*;
use crate::exec::{opcode_row, FaultKind};
use crate::rig::Rig;
use crate::run::{call_with, eval, num};

fn faults(body: &[u8]) -> FaultKind {
    eval(body, vec![]).expect_err("faults").kind
}

#[test]
fn the_special_rows_do_what_they_say() {
    assert_eq!(num(&eval(&cat(&[&[0xA3, 0xCC], &ret(&int(7))]), vec![]).unwrap()), 7);
    assert_eq!(faults(&[0x5B, 0x32, 1, 2, 0, 0, 0, 0x0A, 3]), FaultKind::Fatal);
    assert_eq!(faults(&cat(&[&[0x5B, 0x21], &int(0xFFFF_FFFF)])), FaultKind::Time);
    assert_eq!(faults(&cat(&[&[0x5B, 0x22], &int(60_000)])), FaultKind::Time);
    assert_eq!(faults(&cat(&[&[0x5B, 0x2A], &[0x60]])), FaultKind::Uninit);
    assert!(matches!(faults(&[0xA4, 0x5B, 0x31]), FaultKind::Parse(_)), "a Debug read");
    let mut r = Rig::new(&method("\\M", 0, &store(&int(5), &[0x5B, 0x31])));
    let limits = crate::exec::ExecLimits::DEFAULT;
    call_with(&mut r.aml, &mut r.probe, limits, "\\M", vec![]).unwrap();
    assert_eq!(r.probe.debug.len(), 1, "a Debug store is said");
}

#[test]
fn a_byte_in_no_row_and_no_name_is_refused() {
    let name = |b: u8| b.is_ascii_uppercase() || matches!(b, b'_' | b'\\' | b'^' | 0x2E | 0x2F);
    let mut refused = 0;
    for b in (0..=0xFFu8).filter(|b| opcode_row(u16::from(*b)).is_none() && !name(*b)) {
        if b == 0x5B {
            continue;
        }
        assert!(eval(&[b], vec![]).is_err(), "{b:#04x}");
        refused += 1;
    }
    for x in (0..=0xFFu8).filter(|x| opcode_row(0x5B00 | u16::from(*x)).is_none()) {
        assert!(eval(&[0x5B, x], vec![]).is_err(), "5B {x:#04x}");
        refused += 1;
    }
    assert!(refused > 300, "{refused}");
}
