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

//! The kept verdict as the `MkBootAttest` record. A self-reported pass keeps
//! its own state, so a reader can never take it for a measured one.

use nonos_boot_measure::gate::Admitted;

use super::record::{encode, Answer, Enrolled, RECORD_LEN};
use super::verdict::{verdict, Verdict};

/// The record for the verdict as it stands; `NOT_YET` before the check has run.
pub fn boot_attest_record() -> [u8; RECORD_LEN] {
    let answer = match verdict() {
        None => Answer::NotYet,
        Some(Verdict::Measured(a)) => Answer::Measured(enrolled(a)),
        Some(Verdict::SelfReported(a)) => Answer::SelfReported(enrolled(a)),
        Some(Verdict::Refused(code)) => Answer::Refused(code),
        Some(Verdict::NoEvidence) => Answer::NoEvidence,
    };
    encode(&answer)
}

fn enrolled(a: Admitted) -> Enrolled {
    Enrolled { measurement: a.measurement, root: a.root, epoch: a.epoch }
}
