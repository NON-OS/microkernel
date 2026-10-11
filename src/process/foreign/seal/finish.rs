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

//! Sealing a family: its live state becomes a record and a witness.

use super::hist::hist_commit;
use super::nonce::seal_no;
use super::record::{encode, Facts, FINAL, SATURATED};
use super::sealed::Sealed;
use super::table::{LIVE, MAX_SEALED, SEALED};

/// Seal `supervisor`'s live family, if it has one, and return its handle.
/// `whole` says no guest of it can still be running.
pub(crate) fn finish(supervisor: u32, whole: bool) -> Option<u64> {
    let mut fam = {
        let mut live = LIVE.lock();
        let at = live.iter().position(|f| f.sup == supervisor)?;
        live.swap_remove(at)
    };
    fam.leave_all();
    // The salt was drawn below p, so the commitment always exists.
    let hc = hist_commit(&fam.hist, &fam.salt)?;
    let mut sealed = SEALED.lock();
    let mut no = seal_no()?;
    while sealed.iter().any(|s| s.seal_no == no) {
        no = seal_no()?;
    }
    let flags = if whole { FINAL } else { 0 } | if fam.saturated { SATURATED } else { 0 };
    let record = encode(&Facts {
        flags,
        sup_meas: fam.sup_meas,
        sup_caps: fam.sup_caps,
        authority: fam.authority,
        seal_no: no,
        trace: fam.trace,
        rows: fam.rows,
        hc,
        meas: fam.meas,
        n_meas: fam.n_meas,
        app_id: fam.app_id.unwrap_or([0; 32]),
    });
    if sealed.len() >= MAX_SEALED {
        sealed.pop_front();
    }
    let (hist, salt, list, diag) = (fam.hist, fam.salt, fam.list, fam.diag);
    sealed.push_back(Sealed { seal_no: no, record, hist, salt, diag, list });
    Some(no)
}
