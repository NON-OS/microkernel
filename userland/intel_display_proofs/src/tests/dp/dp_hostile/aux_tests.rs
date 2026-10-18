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

//! Native and I2C reads against a sink whose replies DEFER, NACK, come
//! empty, short or past what was asked: each ends within its retries, and
//! nothing lands past the bytes asked for. A read or write that reaches
//! the multi-stream sideband is refused before the bus is touched.

use super::profile::profile;
use super::rng::Rng;
use super::sink::Hostile;
use crate::dp::native::{read, write, RETRIES};
use crate::tests::dp::edid_read::read_edid;

const GUARD: u8 = 0xEE;

#[test]
fn every_read_ends_within_its_retries_and_lands_only_where_asked() {
    let mut r = Rng::new(0xA0C5_EED5);
    let (mut whole, mut failed) = (0usize, 0usize);
    for i in 0..50_000u64 {
        let p = profile(&mut r);
        let len = 1 + r.below(40) as usize;
        let mut h = Hostile::new(i, p, [0x12; 16], len as u32 * RETRIES);
        let mut buf = vec![GUARD; len + 8];
        let edid = r.below(4) == 0;
        let got = if edid {
            h.bound = (len as u32 + 2) * RETRIES;
            read_edid(&mut h, 0, &mut buf[..len])
        } else {
            read(&mut h, r.below(0x300), &mut buf[..len])
        };
        assert!(buf[len..].iter().all(|&b| b == GUARD), "a byte landed past the read");
        (whole, failed) = (whole + got.is_ok() as usize, failed + got.is_err() as usize);
    }
    assert!(whole >= 5_000 && failed >= 5_000, "{whole} {failed}");
}

#[test]
fn the_sideband_is_refused_before_the_bus() {
    let p = profile(&mut Rng::new(3));
    for (addr, len) in
        [(0x111, 1), (0x110, 4), (0x1000, 16), (0x17FF, 1), (0x0FF8, 9), (0x1400, 48)]
    {
        let mut h = Hostile::new(1, p, [0x12; 16], 0);
        assert!(read(&mut h, addr, &mut vec![0; len]).is_err(), "{addr:#x}");
        assert!(write(&mut h, addr, &vec![0; len]).is_err(), "{addr:#x}");
        assert_eq!(h.transfers, 0, "nothing sent to {addr:#x}");
    }
    let mut h = Hostile::new(1, p, [0x12; 16], 64);
    let _ = (read(&mut h, 0x0FF0, &mut [0; 16]), read(&mut h, 0x1800, &mut [0; 1]));
    assert!(h.transfers > 0, "the edges either side are ordinary DPCD");
}
