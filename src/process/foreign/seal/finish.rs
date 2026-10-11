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

//! A family ends when its last guest leaves the registry, by exit or by
//! teardown. Its live state becomes a final record and a witness, or nothing:
//! a family no app was ever noted in is dropped, never kept.

use super::hist::hist_commit;
use super::nonce::seal_no;
use super::record::FINAL;
use super::sealed::Sealed;
use super::snapshot::record_of;
use super::table::{LIVE, MAX_SEALED, SEALED};

/// End `supervisor`'s live family, if it has one. The handle of the record
/// kept, or `None` when there was nothing to keep.
pub(crate) fn finish(supervisor: u32) -> Option<u64> {
    let mut fam = {
        let mut live = LIVE.lock();
        let at = live.iter().position(|f| f.sup == supervisor)?;
        live.swap_remove(at)
    };
    fam.leave_all();
    let app_id = fam.app_id?;
    // The salt was drawn below p, so the commitment always exists.
    let hc = hist_commit(&fam.hist, &fam.salt)?;
    let mut sealed = SEALED.lock();
    let mut no = seal_no()?;
    while sealed.iter().any(|s| s.seal_no == no) {
        no = seal_no()?;
    }
    let record = record_of(&fam, FINAL, no, app_id, hc);
    if sealed.len() >= MAX_SEALED {
        sealed.pop_front();
    }
    let (hist, salt, list, diag) = (fam.hist, fam.salt, fam.list, fam.diag);
    sealed.push_back(Sealed { seal_no: no, app_id, record, hist, salt, diag, list });
    Some(no)
}

/// End `supervisor`'s live family without keeping anything.
pub(crate) fn drop_family(supervisor: u32) {
    LIVE.lock().retain(|f| f.sup != supervisor);
}
