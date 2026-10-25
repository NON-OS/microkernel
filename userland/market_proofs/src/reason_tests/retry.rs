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

//! What the system lacks offers no Retry; a partial package, an unknown stop and a refusal do.

use nonos_market_proto::reason;
use nonos_market_proto::reason::*;

use crate::store::progress::Progress;

#[test]
fn what_the_system_lacks_offers_no_retry() {
    for code in [
        WHY_BOOT_OFFLINE,
        WHY_TOO_LARGE,
        WHY_NO_MIRROR,
        WHY_NO_KEYRING,
        WHY_MODEL_UNKNOWN,
        WHY_MODEL_UNKEPT,
        WHY_MODEL_TOO_LARGE,
    ] {
        assert!(!reason(code).retry, "code {code}");
        assert_eq!(Progress::Failed(code).button(true), b"Details", "code {code}");
    }
}

#[test]
fn a_package_that_did_not_land_whole_is_a_failure_with_a_retry() {
    let p = Progress::Failed(WHY_PARTIAL);
    assert!(p != Progress::Installed && p.retryable());
    assert!(reason(WHY_PARTIAL).line.contains("none of it was kept"));
}

#[test]
fn an_unknown_stop_and_a_refusal_may_be_retried() {
    assert!(reason(0).retry && reason(200).retry);
    assert!(Progress::Refused.retryable());
    assert_eq!(Progress::Refused.button(false), b"Retry");
}
