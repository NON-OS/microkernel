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

//! What `qwen get` says for each refusal: a live boot apart from a volume
//! that failed, so nobody retries what cannot work.

use crate::errno::said;

#[test]
fn a_live_boot_says_an_installed_nonos_is_needed() {
    let line = said(-19);
    assert!(line.ends_with("(ENODEV)"), "{line}");
    assert!(line.contains("no disk carries NONOS") && line.contains("install NONOS"), "{line}");
}

#[test]
fn a_failing_or_unready_volume_is_named_as_such() {
    assert!(said(-5).ends_with("(EIO)"));
    let again = said(-11);
    assert!(again.ends_with("(EAGAIN)") && again.contains("try again"), "{again}");
}

#[test]
fn an_unnamed_errno_is_given_by_number() {
    assert_eq!(said(-99), "the kernel refused (errno 99)");
}

#[test]
fn a_live_session_out_of_memory_says_what_to_do() {
    let room = said(-28);
    assert!(room.ends_with("(ENOSPC)") && room.contains("free memory"), "{room}");
    let memory = said(-12);
    assert!(memory.ends_with("(ENOMEM)") && memory.contains("install NONOS"), "{memory}");
}
