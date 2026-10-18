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

//! One answer to every transfer, so each bound shows on its own: a DEFER
//! tried eight times, a NACK or undefined code ending at once, an empty
//! ACK short; ever more swing given up at the top, not the pass limit.

use super::profile::{Profile, Status};
use super::train_rig::{run_with, Seen};
use crate::dp::aux::{Aux, AuxError};
use crate::dp::native::read;

struct Fixed {
    code: u8,
    len: usize,
    count: u32,
}

impl Aux for Fixed {
    fn transfer(&mut self, _: &[u8], reply: &mut [u8; 20]) -> Result<usize, AuxError> {
        (self.count, reply[0]) = (self.count + 1, self.code);
        assert!(self.count <= 64, "a read that does not end");
        Ok(self.len)
    }
}

#[test]
fn each_reply_kind_meets_its_own_bound() {
    let mut out = [0u8; 4];
    for (code, len, err, tries) in [
        // The standard asks at least seven tries; eight, and no more.
        (0x20, 1, AuxError::Refused, 8),
        (0x10, 1, AuxError::Refused, 1),
        (0x30, 1, AuxError::Refused, 1),
        (0x00, 1, AuxError::Short, 1),
        (0x00, 0, AuxError::Short, 8),
    ] {
        let mut f = Fixed { code, len, count: 0 };
        assert_eq!(read(&mut f, 0x200, &mut out), Err(err), "{code:#x} {len}");
        assert_eq!(f.count, tries, "{code:#x} {len}");
    }
}

#[test]
fn ever_more_swing_is_given_up_at_the_top() {
    let p = Profile {
        timeout: 0,
        defer: 0,
        nack: 0,
        undefined: 0,
        empty: 0,
        odd_len: 0,
        status: Status::Never,
        start_fails: false,
    };
    let b = [0x14, 0x0A, 0x01, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0];
    let Seen { outcome, status_reads } = run_with(1, p, b, (2700, 1));
    // Two rates on one lane; the sink's ask climbs with every read, so
    // each attempt reaches the top swing and stops there: six reads in
    // all, where the ten passes allowed would give twenty.
    assert_eq!((outcome, status_reads), (1, 2 * 3));
}
