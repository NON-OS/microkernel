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

//! Reading a slice at a time takes exactly what the blocking reader takes.

use super::gather_rig::{arrivals, never, sliced, Arrivals, BOUNDS};
use crate::wallet::net::routed_read::{read_routed, Source};

impl Source for Arrivals {
    fn read_wait(&mut self, into: &mut [u8], wait_ms: u64) -> Result<usize, &'static str> {
        let until = self.now + wait_ms as i64;
        loop {
            let got = self.due();
            if !got.is_empty() {
                into[..got.len()].copy_from_slice(&got);
                return Ok(got.len());
            }
            if self.ended() || self.now >= until {
                return Ok(0);
            }
            self.now += 1;
        }
    }

    fn now_ms(&self) -> i64 {
        self.now
    }
}

/// Pieces arriving at their times, and when the far end finishes, if it does.
type Case<'a> = (&'a [(i64, &'a [u8])], Option<i64>);

#[test]
fn sliced_reading_takes_what_the_blocking_reader_takes() {
    let cases: [Case; 6] = [
        (&[(300, b"he"), (350, b"llo")], Some(360)),
        (&[(900, b"late")], None),
        (&[(1_200, b"too late")], None),
        (&[(100, b"a"), (250, b"b"), (800, b"c")], None),
        (&[], Some(50)),
        (&[(10, b"0123456789"), (20, b"0123456789")], None),
    ];
    for (pieces, end_at) in cases {
        let blocking = read_routed(&mut arrivals(pieces, end_at), BOUNDS, never);
        assert_eq!(sliced(arrivals(pieces, end_at), never), blocking, "{pieces:?}");
    }
}
