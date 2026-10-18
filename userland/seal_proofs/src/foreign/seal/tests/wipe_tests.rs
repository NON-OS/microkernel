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

//! The witness a family and its sealed record hold, H, the salt and the
//! diag, is cleared in place; the public record and the list are not.

use super::fixture::family;
use crate::foreign::seal::live::snapshot;
use crate::foreign::seal::table::LIVE;
use crate::foreign::trap_table::Answer;

#[test]
fn a_family_clears_its_witness_and_keeps_the_rest() {
    let mut f = family(61);
    (f.app_id, f.diag) = (Some([0xA6; 32]), [0xD6; 32]);
    f.trapped(5, 0);
    f.answered(5, 0, Answer::Value(4));
    assert!(f.hist.iter().any(|&h| h != 0));
    f.clear_witness();
    assert!(f.hist.iter().all(|&h| h == 0));
    assert_eq!((f.salt, f.diag, f.app_id), ([0; 4], [0; 32], Some([0xA6; 32])));
}

#[test]
fn a_snapshot_clears_its_witness_and_keeps_its_record() {
    let mut f = family(62);
    (f.app_id, f.diag) = (Some([0xA7; 32]), [0xD7; 32]);
    f.measured(&[0xC7; 32]);
    LIVE.lock().push(f);
    let mut s = snapshot(62, 1 << 20).expect("a snapshot");
    let record = s.record;
    s.clear_witness();
    assert!(s.hist.iter().all(|&h| h == 0));
    assert_eq!((s.salt, s.diag), ([0; 4], [0; 32]));
    assert_eq!((s.record, s.list.clone()), (record, vec![[0xC7; 32]]));
    let live = LIVE.lock();
    let f = live.iter().find(|f| f.sup == 62).expect("still live");
    assert_eq!((f.salt, f.diag), (super::fixture::SALT, [0xD7; 32]), "the family's own untouched");
}
