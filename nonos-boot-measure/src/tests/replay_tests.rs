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

use super::boot_log::{boot, LOADER};
use super::log_build::{digest, ev, extend, standard, EV_APP, EV_SEPARATOR};
use crate::tcg::{replay, EV_NO_ACTION};

#[test]
fn a_crafted_log_replays_to_the_pcr_the_tpm_would_hold() {
    let (log, pcr) = boot(&[]);
    let r = replay(&log).expect("replays");
    assert_eq!(r.pcr4, pcr);
    assert_eq!(r.last_application, Some(digest(LOADER)));
    assert_eq!(r.events, 6);
}

#[test]
fn a_no_action_event_in_pcr_4_is_logged_and_never_extended() {
    let (log, pcr) = boot(&[ev(4, EV_NO_ACTION, digest(7))]);
    assert_eq!(replay(&log).expect("replays").pcr4, pcr);
}

#[test]
fn an_application_started_after_the_loader_is_the_last_application() {
    let (log, pcr) = boot(&[ev(4, EV_APP, digest(8))]);
    let r = replay(&log).expect("replays");
    assert_eq!(r.last_application, Some(digest(8)));
    assert_eq!(r.pcr4, extend(pcr, digest(8)));
}

#[test]
fn a_log_with_no_application_names_none() {
    let mut log = standard();
    log.extend(ev(4, EV_SEPARATOR, digest(3)));
    assert_eq!(replay(&log).expect("replays").last_application, None);
}

#[test]
fn the_header_alone_is_an_empty_pcr_4() {
    let r = replay(&standard()).expect("replays");
    assert_eq!((r.pcr4, r.last_application, r.events), ([0; 32], None, 0));
}
