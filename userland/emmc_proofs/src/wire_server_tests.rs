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

//! The capsule's own checks on a request, before the card is asked.

use crate::medium_rule::allows;
use crate::protocol::{
    serves_unit, E_INVAL, E_MSGSIZE, E_NXIO, OP_GEOMETRY, OP_HEALTHCHECK, OP_READ, OP_WRITE,
};
use crate::rw_parse::parse;

fn head(lba: u64, n: u32) -> [u8; 12] {
    let mut b = [0u8; 12];
    b[..8].copy_from_slice(&lba.to_le_bytes());
    b[8..].copy_from_slice(&n.to_le_bytes());
    b
}

#[test]
fn a_run_inside_the_disk_and_one_request_is_taken() {
    assert_eq!(parse(&head(0, 1), 100, 64), Ok((0, 1)));
    assert_eq!(parse(&head(36, 64), 100, 64), Ok((36, 64)));
    // The head is read from the front; WRITE's data after it is the caller's.
    let mut write = head(99, 1).to_vec();
    write.extend_from_slice(&[0xa5; 512]);
    assert_eq!(parse(&write, 100, 64), Ok((99, 1)));
}

#[test]
fn a_bad_run_is_refused_with_the_status_that_names_it() {
    assert_eq!(parse(&head(0, 0), 100, 64), Err(E_INVAL));
    assert_eq!(parse(&head(0, 65), 100, 64), Err(E_MSGSIZE));
    assert_eq!(parse(&head(37, 64), 100, 64), Err(E_NXIO));
    assert_eq!(parse(&head(100, 1), 100, 64), Err(E_NXIO));
    assert_eq!(parse(&head(u64::MAX, 1), u64::MAX, 64), Err(E_NXIO));
    assert_eq!(parse(&head(0, 1)[..11], 100, 64), Err(E_MSGSIZE));
}

#[test]
fn only_the_kernel_or_a_store_writer_reaches_the_medium() {
    for op in [OP_GEOMETRY, OP_READ, OP_WRITE] {
        assert!(allows(op, 0, false), "kernel, op {op}");
        assert!(!allows(op, 41, false), "a capsule without StoreWrite, op {op}");
        assert!(allows(op, 41, true), "a StoreWrite holder, op {op}");
    }
    assert!(allows(OP_HEALTHCHECK, 41, false));
}

#[test]
fn only_unit_zero_is_served() {
    assert!(serves_unit(0));
    for unit in [1, 2, 3, u16::MAX] {
        assert!(!serves_unit(unit), "unit {unit}");
    }
}
