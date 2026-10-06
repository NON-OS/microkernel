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

//! The reader under hostile bytes: a well-formed file with bytes changed at
//! random, and headers written from random fields. Deterministic, so a
//! failure is reproducible from its seed. Any panic fails the test; an
//! accepted file must still keep every promise the summary makes.

use super::build::Gguf;
use super::model::small_model;
use crate::{parse, Summary, DEFAULT};

fn next(s: &mut u64) -> u64 {
    *s ^= *s << 13;
    *s ^= *s >> 7;
    *s ^= *s << 17;
    *s
}

fn promises_kept(s: &Summary, len: u64) {
    assert!(s.data_start <= len || s.tensors == 0);
    assert!(s.data_bytes <= len);
    assert_eq!(s.data_start % u64::from(s.meta.alignment), 0);
    assert!(s.tensors <= DEFAULT.max_tensors && s.keys <= DEFAULT.max_keys);
}

#[test]
fn bytes_changed_at_random_never_panic_the_reader() {
    let good = small_model();
    let mut seed = 0x2545_f491_4f6c_dd1d_u64;
    for _ in 0..200_000 {
        let mut file = good.clone();
        for _ in 0..1 + next(&mut seed) % 4 {
            let at = (next(&mut seed) % file.len() as u64) as usize;
            file[at] = next(&mut seed) as u8;
        }
        if let Ok(s) = parse(&mut file.as_slice(), file.len() as u64, &DEFAULT) {
            promises_kept(&s, file.len() as u64);
        }
    }
}

#[test]
fn headers_of_random_fields_never_panic_the_reader() {
    let mut seed = 0x9e37_79b9_7f4a_7c15_u64;
    for _ in 0..50_000 {
        let mut g = Gguf::header(3, next(&mut seed) % 4, next(&mut seed) % 4);
        for _ in 0..next(&mut seed) % 24 {
            match next(&mut seed) % 3 {
                0 => g.u32((next(&mut seed) % 14) as u32),
                1 => g.u64(next(&mut seed) % 300),
                _ => g.u64(next(&mut seed)),
            };
        }
        g.data(32, (next(&mut seed) % 600) as usize);
        if let Ok(s) = parse(&mut g.out.as_slice(), g.out.len() as u64, &DEFAULT) {
            promises_kept(&s, g.out.len() as u64);
        }
    }
}
