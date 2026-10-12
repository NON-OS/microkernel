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

//! The reply's counts, and the code for a join that did not end associated.

use nonos_wifi_core::mlme::MlmeFailure;

use crate::assoc::{self, Outcome};
use crate::status;

use super::result::{failure_code, ConnectResult, CODE_REFUSED, CODE_TIMED_OUT};

/// The reply's progress counts and the AKM that ran.
pub(super) fn counts(report: &assoc::Report) -> ConnectResult {
    ConnectResult {
        sent: report.sent,
        recv: report.recv,
        data: report.data,
        eapol: report.eapol,
        probe: report.probe,
        deauth: report.deauth,
        to_us: report.to_us,
        state: report.state,
        akm: report.akm.map_or(0, |a| a.suite_type()),
        ..ConnectResult::default()
    }
}

/// The code for a join that did not end associated, with the AP's own.
pub(super) fn missed(outcome: Outcome, failure: Option<MlmeFailure>, r: &mut ConnectResult) -> i32 {
    match outcome {
        Outcome::Refused => {
            status::debug(b"[rtl8821ce] connect: refused\n");
            let (code, ap_code) = failure.map_or((CODE_REFUSED, 0), failure_code);
            r.ap_code = ap_code;
            code
        }
        Outcome::TimedOut => {
            status::debug(b"[rtl8821ce] connect: timed out\n");
            CODE_TIMED_OUT
        }
        Outcome::Joined { .. } => 0,
    }
}
