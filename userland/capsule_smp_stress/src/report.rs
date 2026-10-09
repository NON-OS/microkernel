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

//! The lines the run prints: on the Terminal that started it, and on the
//! serial log when the build grants Debug. `log SMP-STRESS` finds them.

use crate::line::Line;
use crate::stall::Counts;

pub fn started(seconds: u64, threads: usize) {
    emit(
        Line::tagged().str(b" start").field(b"seconds", seconds).field(b"threads", threads as u64),
    );
}

/// A worker the kernel would not start; the run stops there.
pub fn refused(seat: usize, errno: i64) {
    let mut line = Line::tagged();
    line.str(b" FAIL worker not started").field(b"seat", seat as u64);
    emit(line.str(b" errno=-").dec(errno.unsigned_abs()));
}

pub fn progress(elapsed_ms: u64, counts: &Counts) {
    let mut line = Line::tagged();
    line.field(b"t_s", elapsed_ms / 1000);
    emit(fields(&mut line, counts));
}

pub fn finished(elapsed_ms: u64, counts: &Counts, pass: bool, still_running: usize) {
    let mut line = Line::tagged();
    line.str(if pass { b" PASS" } else { b" FAIL" }).field(b"ran_ms", elapsed_ms);
    line.field(b"left", still_running as u64);
    emit(fields(&mut line, counts));
}

fn fields<'a>(line: &'a mut Line, c: &Counts) -> &'a mut Line {
    line.field(b"futex", c.futex).field(b"ipc", c.ipc).field(b"sleeps", c.sleeps);
    line.field(b"stalls", c.stalls).field(b"late", c.late).field(b"errors", c.errors);
    line.field(b"max_futex_ms", c.max_futex_ms).field(b"max_ipc_ms", c.max_ipc_ms);
    line.field(b"max_sleep_over_ms", c.max_sleep_over_ms)
}

fn emit(line: &mut Line) {
    let bytes = line.bytes();
    let _ = nonos_libc::mk_debug(bytes.as_ptr(), bytes.len());
}
