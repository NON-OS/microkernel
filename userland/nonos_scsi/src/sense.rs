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

//! Sense data (SPC-5, 4.4) and what it means for the command that raised it.

use crate::errno::{E_INVAL, E_IO, E_NOMEDIUM, E_NXIO, E_ROFS};

pub const NO_SENSE: u8 = 0x0;
pub const RECOVERED_ERROR: u8 = 0x1;
pub const NOT_READY: u8 = 0x2;
pub const MEDIUM_ERROR: u8 = 0x3;
pub const HARDWARE_ERROR: u8 = 0x4;
pub const ILLEGAL_REQUEST: u8 = 0x5;
pub const UNIT_ATTENTION: u8 = 0x6;
pub const DATA_PROTECT: u8 = 0x7;
pub const ABORTED_COMMAND: u8 = 0xB;

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct Sense {
    pub key: u8,
    pub asc: u8,
    pub ascq: u8,
}

/// The key and the additional sense code of fixed (70h, 71h) or
/// descriptor (72h, 73h) format sense data; None for anything else.
pub fn parse(b: &[u8]) -> Option<Sense> {
    match b.first()? & 0x7F {
        0x70 | 0x71 => {
            let key = b.get(2)? & 0x0F;
            // The additional length decides whether the codes are there.
            let extra = *b.get(7).unwrap_or(&0) as usize;
            let has = |at: usize| at < 8 + extra && at < b.len();
            let asc = if has(12) { b[12] } else { 0 };
            let ascq = if has(13) { b[13] } else { 0 };
            Some(Sense { key, asc, ascq })
        }
        0x72 | 0x73 => Some(Sense { key: b.get(1)? & 0x0F, asc: *b.get(2)?, ascq: *b.get(3)? }),
        _ => None,
    }
}

/// What the layer does with a command that ended in this sense.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Verdict {
    /// Nothing went wrong after all (no sense, or an error the target
    /// recovered from itself).
    Done,
    /// Send it again: a unit attention (a reset, a medium or capacity
    /// change, reported once), an aborted command, a unit becoming ready.
    Retry,
    /// Give up with this wire status.
    Fail(i32),
}

pub fn verdict(s: Sense) -> Verdict {
    match (s.key, s.asc, s.ascq) {
        (NO_SENSE | RECOVERED_ERROR, _, _) => Verdict::Done,
        (UNIT_ATTENTION | ABORTED_COMMAND, _, _) => Verdict::Retry,
        // Logical unit is in the process of becoming ready.
        (NOT_READY, 0x04, 0x01) => Verdict::Retry,
        // Medium not present.
        (NOT_READY, 0x3A, _) => Verdict::Fail(E_NOMEDIUM),
        // Logical block address out of range (21h); no such logical unit (25h).
        (ILLEGAL_REQUEST, 0x21, _) | (ILLEGAL_REQUEST, 0x25, _) => Verdict::Fail(E_NXIO),
        (ILLEGAL_REQUEST, _, _) => Verdict::Fail(E_INVAL),
        (DATA_PROTECT, _, _) => Verdict::Fail(E_ROFS),
        _ => Verdict::Fail(E_IO),
    }
}

/// Invalid command operation code: the target does not have the command.
pub fn invalid_opcode(s: Sense) -> bool {
    s.key == ILLEGAL_REQUEST && s.asc == 0x20
}

#[cfg(test)]
mod tests {
    use super::*;

    fn fixed(key: u8, asc: u8, ascq: u8) -> [u8; 18] {
        let mut b = [0u8; 18];
        b[0] = 0x70;
        b[2] = key;
        b[7] = 10;
        b[12] = asc;
        b[13] = ascq;
        b
    }

    #[test]
    fn both_formats() {
        assert_eq!(parse(&fixed(6, 0x29, 0)), Some(Sense { key: 6, asc: 0x29, ascq: 0 }));
        let mut f = fixed(0x13, 0x11, 0);
        f[0] = 0xF1;
        assert_eq!(parse(&f), Some(Sense { key: 3, asc: 0x11, ascq: 0 }), "deferred, valid bit");
        assert_eq!(
            parse(&[0x72, 0x05, 0x24, 0x00, 0, 0, 0, 0]),
            Some(Sense { key: 5, asc: 0x24, ascq: 0 })
        );
        assert_eq!(parse(&[0x7F, 0, 0]), None);
        assert_eq!(parse(&[]), None);
        assert_eq!(parse(&[0x72, 0x05]), None);
    }

    #[test]
    fn short_fixed_sense_has_no_codes() {
        let mut f = fixed(4, 0x44, 0);
        f[7] = 4;
        assert_eq!(parse(&f), Some(Sense { key: 4, asc: 0, ascq: 0 }));
        assert_eq!(parse(&f[..3]), Some(Sense { key: 4, asc: 0, ascq: 0 }));
    }

    #[test]
    fn verdicts() {
        let v = |k, a, q| verdict(Sense { key: k, asc: a, ascq: q });
        assert_eq!(v(RECOVERED_ERROR, 0x17, 0), Verdict::Done);
        assert_eq!(v(UNIT_ATTENTION, 0x29, 0), Verdict::Retry);
        assert_eq!(v(UNIT_ATTENTION, 0x2A, 0x09), Verdict::Retry);
        assert_eq!(v(ABORTED_COMMAND, 0, 0), Verdict::Retry);
        assert_eq!(v(NOT_READY, 0x04, 0x01), Verdict::Retry);
        assert_eq!(
            v(NOT_READY, 0x04, 0x02),
            Verdict::Fail(-5),
            "needs a start the layer never sends"
        );
        assert_eq!(v(NOT_READY, 0x3A, 0x01), Verdict::Fail(-123));
        assert_eq!(v(MEDIUM_ERROR, 0x11, 0), Verdict::Fail(-5));
        assert_eq!(v(HARDWARE_ERROR, 0x44, 0), Verdict::Fail(-5));
        assert_eq!(v(ILLEGAL_REQUEST, 0x21, 0), Verdict::Fail(-6));
        assert_eq!(v(ILLEGAL_REQUEST, 0x25, 0), Verdict::Fail(-6));
        assert_eq!(v(ILLEGAL_REQUEST, 0x24, 0), Verdict::Fail(-22));
        assert_eq!(v(DATA_PROTECT, 0x27, 0), Verdict::Fail(-30));
        assert!(invalid_opcode(Sense { key: ILLEGAL_REQUEST, asc: 0x20, ascq: 0 }));
        assert!(!invalid_opcode(Sense { key: ILLEGAL_REQUEST, asc: 0x24, ascq: 0 }));
    }
}
