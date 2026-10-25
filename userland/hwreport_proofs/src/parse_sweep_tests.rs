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

//! The parser over random bytes, mutated reports and every buffer size.

use nonos_hwreport::parse::check;

use crate::rng::Rng;
use crate::sample::sample;

#[test]
fn random_bytes_and_mutated_reports_never_panic() {
    let mut buf = [0u8; 2048];
    let (len, _) = sample(&mut buf);
    let mut rng = Rng::new(0x5eed);
    for _ in 0..20_000 {
        let mut r = buf[..len].to_vec();
        for _ in 0..1 + rng.below(4) {
            let at = rng.below(r.len());
            r[at] = rng.next() as u8;
        }
        let _ = check(&r);
        let noise: Vec<u8> = (0..rng.below(300)).map(|_| rng.next() as u8).collect();
        let _ = check(&noise);
    }
}

#[test]
fn every_buffer_size_gives_a_report_that_parses_or_is_too_small_to_begin() {
    for size in 0..400 {
        let mut buf = vec![0u8; size];
        let (len, cut) = sample(&mut buf);
        assert!(len <= size);
        if size >= 120 {
            let s = check(&buf[..len]).unwrap_or_else(|e| panic!("size {size}: {e:?}"));
            assert_eq!(s.cut, cut, "size {size}");
        }
    }
}
