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

//! Sealed frames against vectors/frames.txt, sealed outside the crate from
//! the spec's layout: the nonce is four zero bytes and the sequence number,
//! the associated data is the length field. A frame the crate seals must be
//! those bytes, and those bytes must open, at sequence 0, 1, 300 and the
//! last one a direction has.

use crate::{arr, de};
use nsh_crypto::{Opener, Sealer, Secret};

#[test]
fn every_frame_is_sealed_and_opened_byte_for_byte() {
    let text = include_str!("../../vectors/frames.txt");
    let mut n = 0;
    for f in text.lines().filter(|l| !l.starts_with('#')).map(|l| l.split(' ').collect::<Vec<_>>())
    {
        let (seq, inner, len, want) = (f[0].parse::<u64>().unwrap(), de(f[1]), arr(f[2]), de(f[3]));
        let key = || Secret::new(core::array::from_fn(|i| i as u8));
        let mut out = vec![0u8; want.len()];
        let mut s = Sealer::starting_at(key(), seq, 0);
        assert_eq!(s.seal(len, &inner, &mut out), Ok(want.len()));
        assert_eq!(out, want, "sealed at {seq}");
        let mut o = Opener::starting_at(key(), seq, 0);
        let mut back = vec![0u8; want.len()];
        assert_eq!(o.open(len, &want, &mut back), Ok(inner.len()));
        assert_eq!(&back[..inner.len()], &inner[..], "opened at {seq}");
        n += 1;
    }
    assert_eq!(n, 4);
}
