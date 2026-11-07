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

//! `printf FORMAT [ARG...]`: write the arguments through a format string.
//!
//! `echo` writes its words with a space between each and a newline after; it
//! cannot place a tab, omit the newline, or render a number in hex. `printf`
//! interprets backslash escapes (`\n \t \r \\ \0`) and the conversions
//! `%s %d %i %x %o %c %%`, and reuses the format while arguments remain, the
//! way the POSIX utility does. Output is line oriented here: the built string
//! is split on newlines and each piece becomes one scrollback line.

use alloc::vec::Vec;

use crate::out::ShellOut;
use crate::util::format_u64;

/// Render the format over the operands and write the result. Returns false
/// only when no format was given, so `printf && ...` gates on real use.
pub fn run(out: &mut dyn ShellOut, argv: &[&[u8]]) -> bool {
    let Some(fmt) = argv.get(1) else {
        out.writeln(b"usage: printf <format> [arg ...]");
        return false;
    };
    let args = argv.get(2..).unwrap_or(&[]);
    let mut buf = Vec::new();
    let mut idx = 0;
    // One pass per cycle; keep cycling only while a pass still consumes an
    // argument, so a format with no conversions runs exactly once.
    loop {
        let before = idx;
        render(fmt, args, &mut idx, &mut buf);
        if idx >= args.len() || idx == before {
            break;
        }
    }
    emit(out, &buf);
    true
}

/// Append one pass of `fmt` to `buf`, advancing `idx` over the args it uses.
fn render(fmt: &[u8], args: &[&[u8]], idx: &mut usize, buf: &mut Vec<u8>) {
    let mut i = 0;
    while i < fmt.len() {
        match fmt[i] {
            b'\\' if i + 1 < fmt.len() => {
                buf.push(escape(fmt[i + 1]));
                i += 2;
            }
            b'%' if i + 1 < fmt.len() => {
                i += 1;
                convert(fmt[i], args, idx, buf);
                i += 1;
            }
            c => {
                buf.push(c);
                i += 1;
            }
        }
    }
}

/// The byte a backslash escape stands for; an unknown escape keeps the letter.
fn escape(c: u8) -> u8 {
    match c {
        b'n' => b'\n',
        b't' => b'\t',
        b'r' => b'\r',
        b'0' => 0,
        b'\\' => b'\\',
        other => other,
    }
}

/// Write one `%` conversion, taking the next argument where the spec needs one.
fn convert(spec: u8, args: &[&[u8]], idx: &mut usize, buf: &mut Vec<u8>) {
    if spec == b'%' {
        buf.push(b'%');
        return;
    }
    let arg = args.get(*idx).copied().unwrap_or(b"");
    *idx += 1;
    match spec {
        b's' => buf.extend_from_slice(arg),
        b'c' => {
            if let Some(&b) = arg.first() {
                buf.push(b);
            }
        }
        b'd' | b'i' => push_signed(buf, to_i64(arg)),
        b'x' => push_radix(buf, to_i64(arg) as u64, 16),
        b'o' => push_radix(buf, to_i64(arg) as u64, 8),
        // An unknown conversion is written literally and no argument is taken.
        other => {
            *idx -= 1;
            buf.push(b'%');
            buf.push(other);
        }
    }
}

/// Parse a base-ten integer, allowing a leading sign; junk reads as zero, the
/// way the shell utility treats a non-numeric argument.
fn to_i64(arg: &[u8]) -> i64 {
    let (neg, digits) = match arg.first() {
        Some(b'-') => (true, &arg[1..]),
        Some(b'+') => (false, &arg[1..]),
        _ => (false, arg),
    };
    let mut v: i64 = 0;
    for &b in digits {
        if !b.is_ascii_digit() {
            break;
        }
        v = v.saturating_mul(10).saturating_add((b - b'0') as i64);
    }
    if neg {
        -v
    } else {
        v
    }
}

fn push_signed(buf: &mut Vec<u8>, v: i64) {
    if v < 0 {
        buf.push(b'-');
    }
    let mut tmp = [0u8; 20];
    let n = format_u64(v.unsigned_abs(), &mut tmp);
    buf.extend_from_slice(&tmp[..n]);
}

fn push_radix(buf: &mut Vec<u8>, mut v: u64, radix: u64) {
    if v == 0 {
        buf.push(b'0');
        return;
    }
    let mut tmp = [0u8; 22];
    let mut n = 0;
    while v > 0 {
        let d = (v % radix) as u8;
        tmp[n] = if d < 10 { b'0' + d } else { b'a' + d - 10 };
        n += 1;
        v /= radix;
    }
    for i in (0..n).rev() {
        buf.push(tmp[i]);
    }
}

/// Split the built bytes on newlines into scrollback lines. A trailing newline
/// does not add an empty row; its absence still leaves the last piece a row,
/// because the scrollback stores whole lines.
fn emit(out: &mut dyn ShellOut, buf: &[u8]) {
    let mut start = 0;
    let mut i = 0;
    while i < buf.len() {
        if buf[i] == b'\n' {
            out.writeln(&buf[start..i]);
            start = i + 1;
        }
        i += 1;
    }
    if start < buf.len() {
        out.writeln(&buf[start..]);
    }
}
