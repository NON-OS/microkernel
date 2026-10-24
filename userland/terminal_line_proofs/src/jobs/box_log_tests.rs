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

//! The `[BOX]` line reaches the serial log whole, through a writer with the kernel's rule.

use super::box_log::{log, DEBUG_MAX, NO_LOG};

/// The kernel's mk_debug: a call longer than its bound, or empty, refused.
fn kernel(sent: &mut Vec<u8>, calls: &mut usize, piece: &[u8]) -> i64 {
    *calls += 1;
    if piece.is_empty() || piece.len() > 256 {
        return -22;
    }
    sent.extend_from_slice(piece);
    piece.len() as i64
}

#[test]
fn a_line_of_any_length_reaches_the_log_whole() {
    for len in [0usize, 1, 70, 255, 256, 257, 600, 2048] {
        let line: Vec<u8> = (0..len).map(|i| b'a' + (i % 26) as u8).collect();
        let (mut sent, mut calls) = (Vec::new(), 0);
        assert!(log(&line, |p| kernel(&mut sent, &mut calls, p)), "len {len}");
        assert_eq!(sent, [&line[..], b"\n"].concat(), "len {len}");
        assert_eq!(calls, (len + 1).div_ceil(256), "len {len}");
    }
}

#[test]
fn a_refused_piece_is_said_and_the_rest_not_sent() {
    let mut calls = 0;
    let refused = log(&[b'x'; 600], |_| {
        calls += 1;
        if calls == 2 {
            -5
        } else {
            256
        }
    });
    assert!(!refused);
    assert_eq!(calls, 2);
}

#[test]
fn a_build_without_the_serial_log_is_not_a_failure() {
    let mut calls = 0;
    assert!(log(&[b'x'; 600], |_| {
        calls += 1;
        NO_LOG
    }));
    assert_eq!(calls, 1);
}

#[test]
fn the_bound_and_the_missing_grant_are_the_kernels() {
    let kernel = include_str!("../../../../src/syscall/microkernel/debug.rs");
    assert!(kernel.contains(&format!("const MAX_LEN: usize = {DEBUG_MAX};")));
    let gate = include_str!("../../../../src/syscall/contract/dispatch.rs");
    assert!(gate.contains("return SyscallResult::error(errnos::EPERM);"));
    let posix = include_str!("../../../../src/syscall/types/errnos/posix.rs");
    assert!(posix.contains(&format!("pub const EPERM: i32 = {};", -NO_LOG)));
}
