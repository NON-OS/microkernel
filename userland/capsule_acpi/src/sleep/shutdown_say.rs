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

//! What the capsule says of a shutdown (F-120): one line for each of
//! \_TTS(5) and \_PTS(5), absent, run or faulted where; then one for \_S5,
//! unusable, handed to the kernel's soft-off, or refused by the kernel, so
//! the log shows which \_S5 the kernel will write: the capsule's, or its
//! own scanned one if it has one. Pure: the proofs mount it.

/// How a method went.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Went {
    Absent,
    Ran,
    Faulted(u32),
}

/// A piece of a log line.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Part {
    Str(&'static [u8]),
    Hex(u64),
    Dec(u64),
}

const STANDS: &[u8] = b"; the kernel's scanned \\_S5, if any, stands";

/// `what` is `_TTS` or `_PTS`.
pub fn method_line(what: &'static [u8], r: Went, out: &mut dyn FnMut(Part)) {
    out(Part::Str(what));
    match r {
        Went::Absent => out(Part::Str(b" absent")),
        Went::Ran => out(Part::Str(b"(5) ran")),
        Went::Faulted(at) => {
            out(Part::Str(b"(5) faulted at "));
            out(Part::Hex(u64::from(at)));
        }
    }
}

/// `types`: \_S5's pair, if it was usable; `handed`: the kernel's answer
/// to it, an errno when refused.
pub fn s5_line(types: Option<(u8, u8)>, handed: Result<(), u64>, out: &mut dyn FnMut(Part)) {
    let Some((a, b)) = types else {
        out(Part::Str(b"\\_S5 unusable at shutdown"));
        return out(Part::Str(STANDS));
    };
    out(Part::Str(b"\\_S5 "));
    out(Part::Dec(u64::from(a)));
    out(Part::Str(b"/"));
    out(Part::Dec(u64::from(b)));
    match handed {
        Ok(()) => out(Part::Str(b" handed to the kernel's soft-off")),
        Err(e) => {
            out(Part::Str(b" refused by the kernel, errno "));
            out(Part::Dec(e));
            out(Part::Str(STANDS));
        }
    }
}
