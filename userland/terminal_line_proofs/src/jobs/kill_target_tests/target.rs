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

//! What `kill` was given: a pid, a name, or text no service could carry.

use crate::kill_target::{target, Target, NAME_MAX};

#[test]
fn a_number_is_a_pid() {
    assert_eq!(target(b"61"), Target::Pid(61));
    assert_eq!(target(b"0"), Target::Pid(0));
}

#[test]
fn a_word_is_a_name() {
    assert_eq!(target(b"browser"), Target::Name);
    assert_eq!(target(b"app.browser"), Target::Name);
    assert_eq!(target(b"dotenv-linter"), Target::Name);
}

#[test]
fn text_no_service_could_carry_is_refused() {
    assert_eq!(target(b""), Target::Unreadable);
    assert_eq!(target(b"a b"), Target::Unreadable);
    assert_eq!(target(b"../wm"), Target::Unreadable);
    assert_eq!(target(&[b'a'; NAME_MAX + 1]), Target::Unreadable);
    assert_eq!(target(b"99999999999999999999999"), Target::Unreadable);
}

#[test]
fn a_refusal_says_why_in_words() {
    use crate::kill_target::refused;
    let perm = String::from_utf8_lossy(refused(-1));
    assert!(perm.contains("Process Manager"), "{perm}");
    assert!(perm.contains("only what it started"), "{perm}");
    assert!(String::from_utf8_lossy(refused(-22)).contains("2, 9 or 15"));
}
