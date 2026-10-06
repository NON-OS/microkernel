// NONOS Operating System
// Copyright (C) 2026 NONOS Contributors
// SPDX-License-Identifier: AGPL-3.0-or-later

//! Whether a standard stream reaches a terminal, and its size. A launcher
//! that renders a child's output says which of the child's streams reach
//! its screen; the child asks before choosing colour and columns.

use crate::syscall::{call_raw, N_MK_TTY_QUERY, N_MK_TTY_SET};

/// Standard streams, as bits of `mk_tty_set`'s `streams`.
pub const TTY_STDIN: u64 = 1 << 0;
pub const TTY_STDOUT: u64 = 1 << 1;
pub const TTY_STDERR: u64 = 1 << 2;

/// Say which of child `pid`'s streams reach the caller's terminal, and its
/// size in cells. Only the child's parent may; zero streams clears it.
pub fn mk_tty_set(pid: u32, streams: u64, cols: u16, rows: u16) -> i64 {
    call_raw(N_MK_TTY_SET, [pid as u64, streams, cols as u64, rows as u64, 0, 0])
}

/// The size, as `(cols, rows)`, of the terminal the caller's stream `fd`
/// (0, 1 or 2) reaches, or `None` when it reaches none.
pub fn mk_tty_query(fd: u32) -> Option<(u16, u16)> {
    let rc = call_raw(N_MK_TTY_QUERY, [fd as u64, 0, 0, 0, 0, 0]);
    (rc >= 0).then_some(((rc & 0xFFFF) as u16, ((rc >> 16) & 0xFFFF) as u16))
}

/*
 * A launcher whose screen takes a child's stderr and not its stdout (`> f`)
 * still reads both from the one output channel. A child that writes both
 * there then starts each message with the stream it came from, so the
 * launcher keeps stderr on its screen and stdout for the file.
 */
/// First byte of a message from stdout, on a split channel.
pub const TAG_STDOUT: u8 = 1;
/// First byte of a message from stderr, on a split channel.
pub const TAG_STDERR: u8 = 2;
