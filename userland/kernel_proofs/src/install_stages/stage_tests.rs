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

use super::stage::Stage;

#[test]
fn an_installer_that_panicked_is_failed_with_its_code() {
    assert_eq!(Stage::Running(41).ended(134), Stage::Failed(134));
    assert!(Stage::Running(41).ended(134).finished());
}

#[test]
fn an_installer_that_was_killed_or_never_started_is_failed() {
    // A signal's end (128 + signo), and a status lost to the reap log.
    assert_eq!(Stage::Running(7).ended(137), Stage::Failed(137));
    assert_eq!(Stage::Running(7).ended(-1), Stage::Failed(-1));
}

#[test]
fn a_clean_exit_installs_or_removes() {
    assert_eq!(Stage::Running(3).ended(0), Stage::Installed);
    assert_eq!(Stage::Removing(3).ended(0), Stage::Removed);
    assert_eq!(Stage::Removing(3).ended(5), Stage::Failed(5));
}

#[test]
fn a_stage_with_no_process_is_left_as_it_is() {
    for s in [Stage::Queued, Stage::Installed, Stage::Refused, Stage::Failed(9), Stage::Removed] {
        assert_eq!(s.ended(134), s);
        assert_eq!(s.pid(), None);
    }
    assert_eq!(Stage::Running(12).pid(), Some(12));
    assert_eq!(Stage::Removing(13).pid(), Some(13));
}

const REAP_LOG: &str = include_str!("../../../../src/process/exit/reap_log.rs");
const STATUS: &str = include_str!("../../../../src/userspace/init/linux_jobs/status.rs");

#[test]
fn teardown_finishes_the_install_as_it_records_the_status() {
    let at = REAP_LOG.find("pub(super) fn record(").expect("record");
    let record = &REAP_LOG[at..at + REAP_LOG[at..].find("\n}\n").expect("end")];
    assert!(record.contains("crate::userspace::init::install_ended(pid, code);"));
    // Teardown cannot wait on init's table: it only tries the lock.
    let at = STATUS.find("pub(crate) fn ended(").expect("ended");
    assert!(STATUS[at..].contains("STAGES.try_lock()"));
}

#[test]
fn only_a_finished_stage_can_give_way() {
    assert!(!Stage::Queued.finished());
    assert!(!Stage::Running(1).finished());
    assert!(!Stage::Removing(1).finished());
}
