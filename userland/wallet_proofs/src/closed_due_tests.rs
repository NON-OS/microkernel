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

//! A CLOSE the wallet sent puts its channel's reclaimTo on the due list
//! (v1.4.60), and nothing else does.

use crate::meter_job::{closed_due, JobKind, MeterJob, Outcome};

#[test]
fn only_a_sent_close_puts_its_reclaim_to_on_the_due_list() {
    let sent = Outcome::Sent(String::from("close sent"));
    let close = MeterJob { id: 1, kind: JobKind::Close, reclaim_to: Some([7; 20]) };
    assert_eq!(closed_due(&close, &sent), Some([7; 20]));
    assert_eq!(closed_due(&close, &Outcome::Running), None, "not yet sent");
    assert_eq!(closed_due(&close, &Outcome::Failed("no")), None, "refused");
    let topup = MeterJob { kind: JobKind::TopUp, ..close };
    assert_eq!(closed_due(&topup, &sent), None, "a top-up reclaims nothing");
    let zero = MeterJob { reclaim_to: Some([0; 20]), ..close };
    assert_eq!(closed_due(&zero, &sent), None, "never the zero address");
    assert_eq!(closed_due(&MeterJob { reclaim_to: None, ..close }, &sent), None);
}
