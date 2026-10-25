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

//! One byte changed anywhere in a good record: at sampled positions over
//! the whole record and at every field's edge in the header and footer.
//! Without checkpoints every one is refused. With them none is ever called
//! complete, and one inside what a checkpoint covers is refused.

use super::record_good::check;
use crate::kit::record::Rec;
use nsh_client::Verdict;

fn positions(rec: &Rec, len: usize) -> Vec<usize> {
    let h = rec.header.len();
    let edges = [0, 4, 6, 8, 24, 32, 96, 128, 2080, 2082, h - 1, h, h + 4, h + 12, h + 13];
    let tail = [len - 3450, len - 3446, len - 3438, len - 3437, len - 3373, len - 3309, len - 1];
    let mut at: Vec<usize> = (0..len).step_by(len / 240).collect();
    at.extend(edges.iter().chain(tail.iter()));
    at
}

#[test]
fn a_byte_changed_anywhere_is_refused() {
    let rec = Rec::good(130);
    let good = rec.bytes();
    for at in positions(&rec, good.len()) {
        let mut b = good.clone();
        b[at] ^= 0x10;
        assert!(check(&b, &[]).is_err(), "byte {at} of {}", good.len());
    }
}

#[test]
fn with_checkpoints_a_changed_record_is_never_complete() {
    let rec = Rec::good(130);
    let good = rec.bytes();
    let ckpts = vec![rec.ckpt(63), rec.ckpt(127)];
    let covered = rec.header.len() + rec.entries[..=127].iter().map(Vec::len).sum::<usize>();
    for at in positions(&rec, good.len()) {
        let mut b = good.clone();
        b[at] ^= 0x10;
        let v = check(&b, &ckpts);
        assert!(!matches!(v, Ok(Verdict::Complete { .. })), "byte {at}: {v:?}");
        if at < covered {
            assert!(v.is_err(), "byte {at} is covered by a checkpoint: {v:?}");
        } else if let Ok(v) = v {
            assert_eq!(v, Verdict::CutShort { seq: 127, boxed: false }, "byte {at}");
        }
    }
}
