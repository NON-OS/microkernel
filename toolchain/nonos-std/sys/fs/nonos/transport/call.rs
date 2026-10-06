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

// Issue one VFS request and return its reply: frame the body, send it over
// the pool port with MICL, check the status word, and hand back the raw
// reply bytes. read_u32 is the little-endian field reader replies parse with.

use super::consts::{BODY_OFF, STATUS_OFF};
use super::err::{err, vfs_err};
use super::frame::frame;
use super::syscall::{sys3, sys6, tag4};
use crate::io;
use crate::vec::Vec;

/// How long a file call waits for vfs. vfs answers in milliseconds, or says
/// at once that a file is not loaded yet; a call it never answers ends with an
/// error here rather than holding its program for good.
const VFS_CALL_MS: u64 = 30_000;

pub(crate) fn call(port: u32, op: u16, body: &[u8], reply_cap: usize) -> io::Result<Vec<u8>> {
    let tx = frame(op, body);
    let mut rx = crate::vec![0u8; BODY_OFF + reply_cap];
    let began = now_ms();
    let n = unsafe {
        sys6(
            tag4(b"MICL"),
            port as u64,
            tx.as_ptr() as u64,
            tx.len() as u64,
            rx.as_mut_ptr() as u64,
            rx.len() as u64,
            VFS_CALL_MS,
        )
    };
    let status = if n < (STATUS_OFF + 4) as i64 {
        None
    } else {
        Some(i32::from_le_bytes([rx[20], rx[21], rx[22], rx[23]]))
    };
    let took = now_ms().saturating_sub(began);
    if took >= SLOW_MS {
        slow(op, took, n, status);
    }
    let Some(status) = status else {
        return Err(err("vfs ipc failed"));
    };
    if status != 0 {
        return Err(vfs_err(status));
    }
    rx.truncate(n as usize);
    Ok(rx)
}

pub(crate) fn read_u32(b: &[u8], off: usize) -> u32 {
    if b.len() < off + 4 {
        return 0;
    }
    u32::from_le_bytes([b[off], b[off + 1], b[off + 2], b[off + 3]])
}

/* A file call this slow says so on serial, once each: whether vfs answered
 * late or the caller slept past an answer that had come is told by the
 * kernel's own lines around it. Op and time only, never a path. */
const SLOW_MS: u64 = 1_000;

fn now_ms() -> u64 {
    let r = unsafe { sys3(tag4(b"MTMS"), 0, 0, 0) };
    if r < 0 { 0 } else { r as u64 }
}

fn slow(op: u16, took: u64, rc: i64, status: Option<i32>) {
    let mut line = [0u8; 96];
    let mut at = put(&mut line, 0, b"[vfs] op ");
    at = num(&mut line, at, op as u64);
    at = put(&mut line, at, b" took ");
    at = num(&mut line, at, took);
    at = put(&mut line, at, b" ms, ");
    match status {
        Some(s) if s < 0 => {
            at = put(&mut line, at, b"status -");
            at = num(&mut line, at, s.unsigned_abs() as u64);
        }
        Some(s) => {
            at = put(&mut line, at, b"status ");
            at = num(&mut line, at, s as u64);
        }
        None if rc < 0 => {
            at = put(&mut line, at, b"no answer, rc -");
            at = num(&mut line, at, rc.unsigned_abs());
        }
        None => at = put(&mut line, at, b"short answer"),
    }
    at = put(&mut line, at, b"\n");
    unsafe { sys3(tag4(b"MDBG"), line.as_ptr() as u64, at as u64, 0) };
}

fn put(line: &mut [u8], at: usize, bytes: &[u8]) -> usize {
    let n = bytes.len().min(line.len() - at);
    line[at..at + n].copy_from_slice(&bytes[..n]);
    at + n
}

fn num(line: &mut [u8], at: usize, mut v: u64) -> usize {
    let mut digits = [0u8; 20];
    let mut i = digits.len();
    loop {
        i -= 1;
        digits[i] = b'0' + (v % 10) as u8;
        v /= 10;
        if v == 0 {
            break;
        }
    }
    put(line, at, &digits[i..])
}
