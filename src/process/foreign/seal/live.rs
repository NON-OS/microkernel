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

//! MkSealRead's live form: the family `pid` supervises, as it
//! stands under LIVE's lock, so record, witness and measurements agree.
//! FLAG_FINAL is clear and nothing is dropped. Each read draws its own
//! seal_no, so two snapshots of one family are two records.

use super::hist::hist_commit;
use super::nonce::seal_no;
use super::sealed::{len_for, Sealed};
use super::snapshot::record_of;
use super::table::LIVE;
use crate::syscall::microkernel::errnos::{ERRNO_IO, ERRNO_NOMEM, ERRNO_SRCH};

/// ESRCH when `pid` supervises no live family with an app noted: it exited,
/// its family has just sealed, or it never named an app.
pub(super) fn snapshot(pid: u64, room: u64) -> Result<Sealed, i64> {
    let pid = u32::try_from(pid).map_err(|_| ERRNO_SRCH)?;
    let live = LIVE.lock();
    let fam = live.iter().find(|f| f.sup == pid).ok_or(ERRNO_SRCH)?;
    let app_id = fam.app_id.ok_or(ERRNO_SRCH)?;
    // Checked before H is committed or copied, so a short buffer costs nothing.
    if len_for(fam.list.len()) as u64 > room {
        return Err(ERRNO_NOMEM);
    }
    let hc = hist_commit(&fam.hist, &fam.salt).ok_or(ERRNO_IO)?;
    let no = seal_no().ok_or(ERRNO_IO)?;
    let record = record_of(fam, 0, no, app_id, hc);
    let (hist, salt, diag, list) = (fam.hist.clone(), fam.salt, fam.diag, fam.list.clone());
    Ok(Sealed { seal_no: no, app_id, record, hist, salt, diag, list })
}
