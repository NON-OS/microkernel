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

//! A footerless record is cut short at its last checkpoint, or refused
//! with none; a checkpoint beyond it or unmet by its chain refuses it.

use super::record_box::{box_body, with_box};
use super::record_good::check;
use crate::kit::keys;
use crate::kit::record::{Rec, SESSION};
use nsh_client::{CkptRefusal, RecordRefusal, Verdict};

fn cut(seq: u32) -> Result<Verdict, RecordRefusal> {
    Ok(Verdict::CutShort { seq, boxed: false })
}

fn ck(r: CkptRefusal) -> Result<Verdict, RecordRefusal> {
    Err(RecordRefusal::Ckpt(r))
}

#[test]
fn a_record_without_a_footer_is_cut_short_at_its_last_checkpoint() {
    let rec = Rec::good(130);
    let open = rec.no_footer();
    assert_eq!(check(&open, &[rec.ckpt(63)]), cut(63));
    assert_eq!(check(&open, &[rec.ckpt(127), rec.ckpt(63)]), cut(127));
    assert_eq!(check(&open[..open.len() - 3], &[rec.ckpt(127)]), cut(127), "a torn last entry");
    assert!(matches!(check(&open, &[]), Err(RecordRefusal::Parse(_))));
}

#[test]
fn a_checkpoint_beyond_the_record_or_unmet_by_its_chain_refuses_it() {
    let rec = Rec::good(130);
    let short = Rec { entries: rec.entries[..50].to_vec(), header: rec.header.clone(), ..rec };
    assert_eq!(check(&short.no_footer(), &[rec.ckpt(63)]), ck(CkptRefusal::Beyond));
    let past = rec.ckpt_with(SESSION, 131, [0; 64], keys::host());
    assert_eq!(check(&rec.bytes(), &[past]), ck(CkptRefusal::Beyond));
    let mut open = rec.no_footer();
    open[rec.header.len() + 20] ^= 1;
    assert_eq!(check(&open, &[rec.ckpt(63)]), ck(CkptRefusal::Chain));
    let wrong = rec.ckpt_with(SESSION, 63, rec.chain_after(62), keys::host());
    assert_eq!(check(&rec.bytes(), std::slice::from_ref(&wrong)), ck(CkptRefusal::Chain));
    assert_eq!(check(&rec.no_footer(), &[rec.ckpt(127), wrong]), ck(CkptRefusal::Chain));
}

#[test]
fn a_cut_short_record_is_flagged_only_for_a_box_it_covers() {
    let rec = with_box(&box_body(1, 1));
    let cut = |seq, boxed| Ok(nsh_client::Verdict::CutShort { seq, boxed });
    assert_eq!(check(&rec.no_footer(), &[rec.ckpt(2)]), cut(2, false));
    assert_eq!(check(&rec.no_footer(), &[rec.ckpt(3)]), cut(3, true));
}

#[test]
fn a_footerless_record_is_read_only_up_to_a_gap_in_its_sequence() {
    let rec = Rec::good(130);
    let mut entries = rec.entries.clone();
    entries.remove(70);
    let open = [rec.header.clone(), entries.concat()].concat();
    assert_eq!(check(&open, &[rec.ckpt(63)]), cut(63));
    assert_eq!(check(&open, &[rec.ckpt(63), rec.ckpt(127)]), ck(CkptRefusal::Beyond));
}
