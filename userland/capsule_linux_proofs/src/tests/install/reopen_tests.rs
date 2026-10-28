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

//! An install's GET opens its connection once more past init's restart of a
//! service that ended, and no more (install/http/reopen.rs): net.core ended
//! once mid-install on KVM, and the next GET, within that second, failed.

const REOPEN: &str = include_str!("../../../../capsule_linux/src/linux/install/http/reopen.rs");
const WINDOW: &str = include_str!("../../../../capsule_linux/src/linux/install/get/window.rs");
const WATCH: &str = include_str!("../../../../../src/userspace/init/supervisor/watch_rule.rs");

/// The number a `const NAME: u64 = …;` line gives, underscores dropped.
fn ms(src: &str, name: &str) -> u64 {
    let at = src.find(&format!("const {name}: u64 = ")).expect(name) + name.len() + 14;
    src[at..at + src[at..].find(';').expect(name)].replace('_', "").parse().expect(name)
}

#[test]
fn a_refused_connect_is_tried_once_more_past_the_restart() {
    assert!(ms(REOPEN, "BACK_MS") >= 2 * ms(WATCH, "SETTLE_MS"), "past init's restart");
    assert_eq!(REOPEN.matches("Conn::open(at, port)").count(), 2, "once more, no more");
    assert!(WINDOW.contains("Conn::open_or_again(m.0, m.1)"), "every GET's connection");
    assert!(!WINDOW.contains("Conn::open(m.0, m.1)"));
}
