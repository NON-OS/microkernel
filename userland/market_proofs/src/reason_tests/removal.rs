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

//! Where an install is held, how an uninstall reads, and the policy reason.

use nonos_market_proto::reason;
use nonos_market_proto::reason::*;

use crate::store::progress::Progress;

/* An install is never said to be kept where it is held in memory. */
#[test]
fn installed_says_where_it_is_held() {
    use nonos_market_proto::installed_line;
    let tier = installed_line(b"linux.qwen-small");
    assert!(tier.contains("in memory") && tier.contains("power off"), "{tier}");
    let package = installed_line(b"linux.jq");
    assert!(package.contains("for this session") && package.contains("restart"), "{package}");
    for line in [tier, package] {
        assert!(!line.to_lowercase().contains("kept"), "{line}");
    }
}

/* An uninstall is followed through: removing, then as good as never installed. */
#[test]
fn an_uninstall_reads_as_removing_then_not_installed() {
    use nonos_market_proto::{removal_only, Stage};
    assert_eq!(Stage::of(5), Stage::Removing);
    assert_eq!(Stage::of(6), Stage::Removed);
    assert!(Stage::Removing.pending() && !Stage::Removed.pending());
    assert_eq!(Progress::of(5), Progress::Removing);
    assert!(Progress::Removing.pending());
    assert_eq!(Progress::Removing.button(true), b"Removing");
    assert_eq!(Progress::of(6), Progress::Removed);
    assert_eq!(Progress::Removed.button(true), b"Install");
    assert!(!Progress::Removed.retryable());
    for code in [WHY_NOT_INSTALLED, WHY_MODEL_KEPT, WHY_REMOVE_PARTIAL, WHY_MODEL_DOWNLOADING] {
        assert!(removal_only(code), "{code}");
    }
    assert!(!removal_only(WHY_VOLUME_FAILED) && !removal_only(WHY_PACKAGE));
    let kept = reason(WHY_MODEL_KEPT);
    assert!(!kept.retry && kept.line.contains("without the model fetcher"), "{}", kept.line);
}

/* C5's install role fails a market program whose policy it cannot check or vouch (plan/30). */
#[test]
fn reason_36_is_the_policy_and_says_nothing_was_installed() {
    assert_eq!(WHY_POLICY, 36);
    let r = reason(WHY_POLICY);
    let line = "The market's policy for this program could not be checked or vouched for; nothing was installed.";
    assert_eq!((r.line, r.retry), (line, false));
}
