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

//! What the driver should have kept over AUX, read off the sink's log by
//! the test's own rule, as for GMBUS (`ddc_sink/judge.rs`): block by block,
//! as many as declared and at most BLOCKS, each tried at most TRIES times,
//! each try aimed where E-DDC puts the block, the first whole one kept. A
//! try is taken when all 128 bytes came in ACKed reads and its stop was
//! ACKed.

use super::sink::Try;
use crate::tests::ddc_sink::judge::{Verdict, BLOCKS, TRIES};
use crate::tests::ddc_sink::own::{declared, whole, BLOCK};

pub fn judge(log: &[Try]) -> Verdict {
    let (mut kept, mut tries, mut retried) = (Vec::new(), log.iter(), false);
    'block: while kept.len() / BLOCK < BLOCKS && kept.len() / BLOCK <= declared(&kept) {
        let n = kept.len() / BLOCK;
        for k in 0..TRIES {
            let Some(t) = tries.next() else { return Verdict { kept, cut: true, retried } };
            aimed(t, n);
            if taken(t, n) && whole(&t.got, &kept) {
                kept.extend_from_slice(&t.got);
                retried |= k > 0;
                continue 'block;
            }
        }
        break;
    }
    assert!(tries.next().is_none(), "a try the rule does not call for");
    Verdict { kept, cut: false, retried }
}

/// Aimed at block `n`: segment n / 2 from block 2 on and none before, the
/// offset only after the segment took, at n % 2 * 128; no read before the
/// offset, and no byte past the block.
fn aimed(t: &Try, n: usize) {
    match (n, t.segment) {
        (0 | 1, s) => assert_eq!(s, None, "block {n} wrote a segment"),
        (_, Some((s, _))) => assert_eq!(s as usize, n / 2, "block {n}'s segment"),
        _ => {}
    }
    let pointed = n < 2 || t.segment.is_some_and(|s| s.1);
    assert!(t.offset.is_none() || pointed, "block {n}'s offset before its segment took");
    assert!(t.offset.is_none_or(|o| o as usize == n % 2 * 128), "block {n}'s offset");
    assert!(!t.read_unaimed && t.got.len() <= BLOCK, "block {n}: {} bytes", t.got.len());
}

fn taken(t: &Try, n: usize) -> bool {
    let pointed = n < 2 || t.segment.is_some_and(|s| s.1);
    pointed && t.offset.is_some() && t.got.len() == BLOCK && t.stop_ok
}
