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

//! Why a proof did not come (I-B9a, section 4), and how the serial line
//! names it. The words are proof_said.rs's.
//!
//! Pure; market_proofs holds it.

use alloc::vec::Vec;

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub enum Fail {
    /// Op 8 answered none or refused, or did not answer.
    NoBox,
    /// The policy names another listing (policy[56..88]).
    NotNamed,
    /// MkProveJob's EPERM.
    MayNot,
    /// MkProveJob's EBUSY, or PROVE answered EBUSY.
    Busy,
    /// app.prove.job not up within its deadline.
    NotStarted,
    /// STATUS unanswered for the job's own quiet spell.
    Quiet,
    /// A GET that was empty, oversized, past the total, or over 1 MiB.
    Broken,
    /// The job's reply status: an errno or a wire code.
    Code(i32),
}

impl Fail {
    /// `<code>` of `failed: linux.<app>, <code>`: the number answered, or a word.
    pub fn code(self) -> Vec<u8> {
        let word: &[u8] = match self {
            Fail::NoBox => b"nobox",
            Fail::NotNamed => b"unnamed",
            Fail::MayNot => b"-1",
            Fail::Busy => b"-16",
            Fail::NotStarted => b"nostart",
            Fail::Quiet => b"quiet",
            Fail::Broken => b"broken",
            Fail::Code(c) => return decimal(c),
        };
        word.to_vec()
    }
}

/// `c` in decimal, as the serial lines print it.
pub fn decimal(c: i32) -> Vec<u8> {
    let mut digits = Vec::new();
    let mut n = c.unsigned_abs();
    loop {
        digits.push(b'0' + (n % 10) as u8);
        n /= 10;
        if n == 0 {
            break;
        }
    }
    digits.extend(if c < 0 { &b"-"[..] } else { &[] });
    digits.reverse();
    digits
}
