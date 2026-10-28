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

//! Where the line goes: the run's own Terminal window, through the private
//! write that reaches no log and no serial console, and from where only the
//! network and the reason are at hand.

const SAID: &str = include_str!("../../../../../../capsule_linux/src/linux/net/routed/said.rs");
const APPLY: &str = include_str!("../../../../../../capsule_linux/src/linux/net/routed/apply.rs");

#[test]
fn it_is_said_in_the_run_s_window_with_no_destination_at_hand() {
    assert!(SAID.contains("if crate::linux::console::attached() {"));
    assert!(SAID.contains("crate::linux::console::say(line.as_bytes())"));
    /* Only the network and the reason reach the line: the entry's `why`. */
    assert!(APPLY.contains("super::said::say(net, why);"));
    assert!(!APPLY.contains("dest"), "the destination is not at hand where the line is said");
}
