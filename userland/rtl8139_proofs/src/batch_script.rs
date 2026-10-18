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

//! A scripted `recv_one` for the batch proofs: each step is what the part
//! hands back on one call, and the batch body comes back as its frames.

use std::collections::VecDeque;

use crate::rx::batch::gather;

pub type Step = Result<Option<Vec<u8>>, &'static str>;

pub fn run(max: usize, steps: Vec<Step>) -> (Option<Vec<Vec<u8>>>, Option<&'static str>, usize) {
    let mut steps: VecDeque<Step> = steps.into();
    let mut calls = 0;
    let got = gather(max, 64 * 1024, |out: &mut [u8]| {
        calls += 1;
        match steps.pop_front().unwrap_or(Ok(None))? {
            Some(f) => {
                out[..f.len()].copy_from_slice(&f);
                Ok(Some(f.len()))
            }
            None => Ok(None),
        }
    });
    (got.body.map(|b| frames(&b)), got.error, calls)
}

fn frames(body: &[u8]) -> Vec<Vec<u8>> {
    let count = u32::from_le_bytes(body[..4].try_into().unwrap()) as usize;
    let (mut at, mut out) = (4, Vec::new());
    for _ in 0..count {
        let len = u32::from_le_bytes(body[at..at + 4].try_into().unwrap()) as usize;
        out.push(body[at + 4..at + 4 + len].to_vec());
        at += 4 + len;
    }
    assert_eq!(at, body.len(), "trailing bytes in the batch");
    out
}
