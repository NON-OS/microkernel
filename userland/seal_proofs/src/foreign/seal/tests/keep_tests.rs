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
//! MkSealPeek and MkSealDrop, for a receipt: a peek copies the newest record
//! for an app and leaves it sealed, any number of times; a drop takes only
//! the record whose app and seal number both match, copying nothing; and
//! after it, nothing of that record can be read again. Each by the gate
//! MkSealRead has. Every test uses its own app, as SEALED is one table.

use crate::errnos::ERRNO_NOMEM;
use crate::foreign::seal::hist::HIST_WORDS;
use crate::foreign::seal::sealed::{len_for, Sealed};
use crate::foreign::seal::sys_keep::sys_seal_peek;
use crate::foreign::seal::table::SEALED;

pub fn seal(app: u8, seal_no: u64) -> [u8; 32] {
    let mut record = [0u8; 240];
    record[0] = app;
    let s = Sealed {
        seal_no,
        app_id: [app; 32],
        record,
        hist: Box::new([0; HIST_WORDS]),
        salt: [1; 4],
        diag: [0; 32],
        list: vec![[2; 32]],
    };
    SEALED.lock().push_back(s);
    [app; 32]
}

pub fn held(app: &[u8; 32]) -> usize {
    SEALED.lock().iter().filter(|s| s.app_id == *app).count()
}

pub fn peek(app: &[u8; 32]) -> (i64, Vec<u8>) {
    let mut out = vec![0u8; len_for(1)];
    let n = sys_seal_peek(app.as_ptr() as u64, out.as_mut_ptr() as u64, out.len() as u64);
    (n, out)
}

#[test]
fn a_peek_copies_the_record_and_leaves_it_sealed() {
    let app = seal(0x61, 7);
    for _ in 0..3 {
        let (n, out) = peek(&app);
        assert_eq!(n as usize, len_for(1));
        assert_eq!(out[0], 0x61, "the record, as MkSealRead hands it");
    }
    assert_eq!(held(&app), 1, "still sealed");
    let mut short = vec![0u8; len_for(1) - 1];
    let n = sys_seal_peek(app.as_ptr() as u64, short.as_mut_ptr() as u64, short.len() as u64);
    assert_eq!((n, held(&app)), (ERRNO_NOMEM, 1));
}
