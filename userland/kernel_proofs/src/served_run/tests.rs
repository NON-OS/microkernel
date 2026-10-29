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

use super::rule::{exited, restarting, started, Served};

const WINDOW: u32 = 40;
const RUN: u32 = 41;
const NET: u32 = 42;
const CORE: &str = "deb:nonos-simplex-core";

fn launched() -> Option<Served> {
    Some(Served { launcher: Some(WINDOW), package: CORE.into(), run: None })
}

#[test]
fn the_run_ends_with_its_launcher() {
    let mut slot = launched();
    assert!(!started(&mut slot, CORE, RUN));
    assert_eq!(exited(&mut slot, 7), None, "another process ends nothing");
    assert_eq!(exited(&mut slot, WINDOW), Some(RUN), "the window's exit ends its core");
    assert_eq!(exited(&mut slot, RUN), None, "the core's own teardown ends no more");
}

#[test]
fn it_ends_with_its_launcher_across_its_start_with_the_network() {
    let mut slot = launched();
    started(&mut slot, CORE, RUN);
    restarting(&mut slot, RUN);
    assert_eq!(exited(&mut slot, RUN), None);
    assert!(slot.is_some(), "the first start's exit kept the record");
    assert!(!started(&mut slot, CORE, NET));
    assert_eq!(exited(&mut slot, WINDOW), Some(NET));
    // init started the network role before the first start said so.
    let mut slot = launched();
    started(&mut slot, CORE, RUN);
    started(&mut slot, CORE, NET);
    restarting(&mut slot, RUN);
    assert_eq!(exited(&mut slot, RUN), None);
    assert_eq!(exited(&mut slot, WINDOW), Some(NET));
}

#[test]
fn a_start_still_queued_when_the_launcher_left_ends_as_it_starts() {
    let mut slot = launched();
    assert_eq!(exited(&mut slot, WINDOW), None, "nothing started yet");
    assert!(started(&mut slot, CORE, RUN), "it would outlive its window");
    assert!(started(&mut slot, CORE, NET), "and so would its start with the network");
    assert!(!started(&mut slot, "jq", 9), "another package is no one's");
}

#[test]
fn a_run_that_ends_by_itself_takes_its_record() {
    let mut slot = launched();
    started(&mut slot, CORE, RUN);
    assert_eq!(exited(&mut slot, RUN), None);
    assert_eq!(slot, None);
    assert_eq!(exited(&mut slot, WINDOW), None, "the window's exit then ends nothing");
    assert!(!started(&mut slot, CORE, 50), "a later run of it is no one's");
}
