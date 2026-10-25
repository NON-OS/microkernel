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

use super::rule::{admit, Facts, Owner, Refused};

const TID: u32 = 41;

/// A worker thread's call: `proc.<tid>`, its own inbox, owned by it.
fn thread() -> Facts<'static> {
    Facts {
        recorded: "proc.41",
        names_now: "proc.41",
        named: false,
        owner: Owner::Pid(TID),
        caller: TID,
        alive: true,
        reachable: true,
    }
}

/// A process's call: the reply inbox it was spawned with, held by the kernel.
fn process() -> Facts<'static> {
    Facts {
        recorded: "endpoint.app.terminal.1.reply",
        names_now: "endpoint.app.terminal.1.reply",
        named: true,
        owner: Owner::Kernel,
        ..thread()
    }
}

#[test]
fn a_thread_and_a_process_are_each_answered() {
    assert_eq!(admit(&thread()), Ok(()));
    assert_eq!(admit(&process()), Ok(()));
}

#[test]
fn an_inbox_the_caller_no_longer_names_is_refused() {
    let moved = Facts { names_now: "proc.42", ..thread() };
    assert_eq!(admit(&moved), Err(Refused::NotItsInbox));
    // A pid reused by a process spawned with its own reply inbox.
    let reused = Facts { names_now: "endpoint.app.files.1.reply", named: true, ..thread() };
    assert_eq!(admit(&reused), Err(Refused::NotItsInbox));
}

#[test]
fn an_inbox_registered_to_another_or_to_nobody_is_refused() {
    assert_eq!(admit(&Facts { owner: Owner::Pid(TID + 1), ..thread() }), Err(Refused::NotItsOwner));
    assert_eq!(admit(&Facts { owner: Owner::Missing, ..thread() }), Err(Refused::NotItsOwner));
    // The kernel holds an inbox only for the process that was spawned with it.
    assert_eq!(admit(&Facts { owner: Owner::Kernel, ..thread() }), Err(Refused::NotItsOwner));
    assert_eq!(admit(&Facts { named: false, ..process() }), Err(Refused::NotItsOwner));
}

#[test]
fn a_dead_caller_or_one_off_the_peer_list_is_refused() {
    assert_eq!(admit(&Facts { alive: false, ..thread() }), Err(Refused::CallerGone));
    assert_eq!(admit(&Facts { alive: false, ..process() }), Err(Refused::CallerGone));
    assert_eq!(admit(&Facts { reachable: false, ..thread() }), Err(Refused::Unreachable));
}
