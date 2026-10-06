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

//! The check, run once on the way to userspace, and its line on the log.

use nonos_boot_measure::gate::{measured, self_reported};

use super::evidence::{gather, Evidence};
use super::key::signed;
use super::log::say;
use super::verdict::{keep, Verdict};
use crate::security::tpm::boot_reads::{pcr4, rollback_floor};

/// Decide, keep and log the verdict. Later calls return the first one.
pub fn check_bootloader() -> Verdict {
    let v = keep(decide(gather()));
    say(v);
    v
}

fn decide(e: Evidence) -> Verdict {
    let (Some(record), Some(trailer)) = (e.record, e.trailer) else {
        return Verdict::NoEvidence;
    };
    /*
     * A TPM that answers both reads and a log the loader carried: the measured
     * path, whatever it finds. Anything less is the loader's word only.
     */
    if let (Some(log), Ok(pcr), Ok(floor)) = (e.log, pcr4(), rollback_floor()) {
        return match measured(log, &pcr, floor, record, trailer, signed) {
            Ok(a) => Verdict::Measured(a),
            Err(err) => Verdict::Refused(err.code()),
        };
    }
    let Some(loader) = e.loader else {
        return Verdict::NoEvidence;
    };
    match self_reported(loader, record, trailer, signed) {
        Ok(a) => Verdict::SelfReported(a),
        Err(err) => Verdict::Refused(err.code()),
    }
}
