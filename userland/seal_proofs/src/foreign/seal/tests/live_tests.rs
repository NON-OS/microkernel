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

//! MkSealRead's live form (plan/15): a snapshot of the family a supervisor
//! runs, FINAL clear, the family left live and nothing dropped; ESRCH for a
//! supervisor with no live family or no app named.

use nonos_proof_abi::{decode_record, hist_commit, meas_chain};

use super::fixture::{family, SALT};
use crate::errnos::{ERRNO_NOMEM, ERRNO_SRCH};
use crate::foreign::seal::finish::finish;
use crate::foreign::seal::live::snapshot;
use crate::foreign::seal::table::{LIVE, SEALED};
use crate::foreign::trap_table::Answer;

fn serving(sup: u32) {
    let mut f = family(sup);
    (f.app_id, f.diag) = (Some([0xA2; 32]), [0xD2; 32]);
    f.measured(&[0xC4; 32]);
    f.trapped(5, 0);
    f.answered(5, 0, Answer::Value(4));
    LIVE.lock().push(f);
}

#[test]
fn a_live_read_is_the_family_as_it_stands_and_leaves_it_live() {
    serving(91);
    let s = snapshot(91, 1 << 20).expect("a snapshot");
    let r = decode_record(&s.record).expect("an NFSR record");
    assert_eq!(r.flags, 0, "FINAL clear: the family still serves");
    assert_eq!(Some(r.hc), hist_commit(&s.hist, &SALT));
    assert_eq!((r.app_id, r.n_meas, r.meas), ([0xA2; 32], 1, meas_chain(&[[0xC4; 32]])));
    assert_eq!((s.diag, s.list.clone()), ([0xD2; 32], vec![[0xC4; 32]]));
    assert!(LIVE.lock().iter().any(|f| f.sup == 91), "still live");
    let again = snapshot(91, 1 << 20).expect("again");
    assert_ne!(again.seal_no, s.seal_no, "each read its own handle");
    assert_eq!(again.record[..56], s.record[..56]);
    assert_eq!(again.record[64..], s.record[64..]);
    let n = finish(91).expect("it seals as before");
    let sealed = SEALED.lock();
    let f = sealed.iter().find(|x| x.seal_no == n).expect("kept");
    assert_eq!(decode_record(&f.record).map(|r| r.flags), Some(1), "the final record is FINAL");
}

#[test]
fn no_live_family_or_no_app_is_esrch_and_a_short_room_is_enomem() {
    assert_eq!(snapshot(92, 1 << 20).err(), Some(ERRNO_SRCH), "no family");
    assert_eq!(snapshot(u64::MAX, 1 << 20).err(), Some(ERRNO_SRCH), "not a pid");
    LIVE.lock().push(family(93));
    assert_eq!(snapshot(93, 1 << 20).err(), Some(ERRNO_SRCH), "no app named yet");
    serving(94);
    assert_eq!(snapshot(94, 100).err(), Some(ERRNO_NOMEM), "the room is short");
    assert!(LIVE.lock().iter().any(|f| f.sup == 94), "nothing lost");
}
