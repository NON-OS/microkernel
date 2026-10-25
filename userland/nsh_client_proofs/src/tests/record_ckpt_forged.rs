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

//! Forged checkpoints: by another key, with either half spoilt, for
//! another session, or malformed. Each refuses the record, complete or not.

use super::record_good::check;
use crate::kit::keys;
use crate::kit::record::{Rec, SESSION};
use nsh_client::{CkptRefusal, Half, RecordRefusal, Verdict};
use nsh_wire::WireError;

fn ck(r: CkptRefusal) -> Result<Verdict, RecordRefusal> {
    Err(RecordRefusal::Ckpt(r))
}

#[test]
fn a_forged_checkpoint_refuses_the_record() {
    let rec = Rec::good(130);
    let forged = rec.ckpt_with(SESSION, 63, rec.chain_after(63), keys::stranger());
    for record in [rec.bytes(), rec.no_footer()] {
        assert_eq!(
            check(&record, &[rec.ckpt(127), forged.clone()]),
            ck(CkptRefusal::Sig(Half::Ed25519))
        );
        let mut half = rec.ckpt(63);
        half[200] ^= 1;
        assert_eq!(check(&record, &[half.clone()]), ck(CkptRefusal::Sig(Half::MlDsa65)));
        half[200] ^= 1;
        half[100] ^= 1;
        assert_eq!(check(&record, &[half.clone()]), ck(CkptRefusal::Sig(Half::Ed25519)));
        half[100] ^= 1;
        half[40] ^= 1;
        assert_eq!(check(&record, &[half]), ck(CkptRefusal::Sig(Half::Ed25519)), "chain changed");
        let other = rec.ckpt_with([0x5F; 16], 63, rec.chain_after(63), keys::host());
        assert_eq!(check(&record, &[other]), ck(CkptRefusal::Session));
        let mut magic = rec.ckpt(63);
        magic[0] = b'X';
        assert_eq!(check(&record, &[magic]), ck(CkptRefusal::Parse(WireError::Magic)));
        let long = [rec.ckpt(63), vec![0]].concat();
        assert_eq!(check(&record, &[long]), ck(CkptRefusal::Parse(WireError::Trailing)));
    }
}
