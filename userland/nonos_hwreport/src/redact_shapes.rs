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

//! The values a kept log line may not carry, found by their shape wherever
//! they sit in the line, glued to a word or not, and marked for masking.

use crate::redact_runs::run_at;

/// Whether `w` is hex groups of `lens` joined by `sep`.
fn shaped(w: &[u8], sep: u8, lens: &[usize]) -> bool {
    let mut at = 0;
    for (n, len) in lens.iter().enumerate() {
        if n > 0 && w.get(at) != Some(&sep) {
            return false;
        }
        at += usize::from(n > 0);
        if w.len() < at + len || !w[at..at + len].iter().all(u8::is_ascii_hexdigit) {
            return false;
        }
        at += len;
    }
    at == w.len()
}

/// A MAC (six pairs by : or -, or three quads by a dot) or a UUID at `i`.
fn fixed_at(line: &[u8], i: usize) -> usize {
    let shapes: [(u8, &[usize]); 4] =
        [(b'-', &[8, 4, 4, 4, 12]), (b':', &[2; 6]), (b'-', &[2; 6]), (b'.', &[4; 3])];
    for (sep, lens) in shapes {
        let n = lens.iter().sum::<usize>() + lens.len() - 1;
        if line.len() >= i + n && shaped(&line[i..i + n], sep, lens) {
            return n;
        }
    }
    0
}

/// Four dot-joined groups of one to three digits at `i`. The last group
/// takes at most three digits, so an address glued to more digits (another
/// address, a port, a count) is masked rather than missed.
fn ipv4_at(line: &[u8], i: usize) -> usize {
    let mut at = i;
    for n in 0..4 {
        if n > 0 && line.get(at) != Some(&b'.') {
            return 0;
        }
        at += usize::from(n > 0);
        let run = line.get(at..).unwrap_or(&[]).iter().take_while(|b| b.is_ascii_digit()).count();
        let digits = if n == 3 { run.min(3) } else { run };
        if digits == 0 || digits > 3 {
            return 0;
        }
        at += digits;
    }
    at - i
}

/// How many bytes from `i` a value to mask covers; 0 if none starts there.
pub(crate) fn value_at(line: &[u8], i: usize) -> usize {
    fixed_at(line, i).max(ipv4_at(line, i)).max(run_at(line, i))
}
