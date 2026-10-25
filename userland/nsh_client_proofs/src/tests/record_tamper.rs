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

//! A record changed by someone without the host keys: an entry dropped or
//! reordered, with or without the sequence fixed and the chain recomputed;
//! and a record whose header, document or footer is not the host's.

use super::record_good::check;
use crate::kit::record::{entry, sample, Rec, START};
use crate::kit::record_forge::forge;
use nsh_client::{Half, RecordRefusal};
use nsh_wire::WireError;

fn renumbered(entries: &[Vec<u8>]) -> Vec<Vec<u8>> {
    let mut out = entries.to_vec();
    for (i, e) in out.iter_mut().enumerate() {
        e[..4].copy_from_slice(&(i as u32).to_le_bytes());
    }
    out
}

#[test]
fn a_dropped_entry_is_refused() {
    let rec = Rec::good(130);
    for k in [0usize, 64, 129, 130] {
        let mut entries = rec.entries.clone();
        entries.remove(k);
        let raw =
            [rec.header.clone(), entries.concat(), rec.bytes()[rec.no_footer().len()..].to_vec()]
                .concat();
        assert_eq!(check(&raw, &[]), Err(RecordRefusal::Parse(WireError::Value)), "entry {k}");
        let fixed = renumbered(&entries);
        assert_eq!(check(&forge(&rec, fixed.clone(), false), &[]), Err(RecordRefusal::Chain));
        let r = check(&forge(&rec, fixed, true), &[]);
        assert_eq!(r, Err(RecordRefusal::Sig(Half::Ed25519)), "entry {k}");
    }
}

#[test]
fn a_reordered_entry_is_refused() {
    let rec = Rec::good(130);
    let mut swapped = rec.entries.clone();
    swapped.swap(10, 11);
    assert_eq!(
        check(&forge(&rec, swapped.clone(), false), &[]),
        Err(RecordRefusal::Parse(WireError::Value))
    );
    let (a, b) = (sample(11), sample(10));
    swapped[10] = entry(10, START + 100, a.0, &a.1);
    swapped[11] = entry(11, START + 110, b.0, &b.1);
    assert_eq!(check(&forge(&rec, swapped.clone(), false), &[]), Err(RecordRefusal::Chain));
    assert_eq!(check(&forge(&rec, swapped, true), &[]), Err(RecordRefusal::Sig(Half::Ed25519)));
}
