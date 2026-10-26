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

//! The kernel log's serial echo (src/log/backend/serial_budget.rs and its
//! lifecycle): PER_SECOND lines a second for each tag, the rest counted and
//! said by the tag's next echoed line; registered on exactly the images
//! that show capsule output on serial and on the attack suite's.

#[path = "../../../../src/log/backend/serial_budget.rs"]
mod serial_budget;

use serial_budget::{tag, Budget, Echo, PER_SECOND, SLOTS};

const LIFECYCLE: &str = include_str!("../../../../src/log/manager/lifecycle.rs");
const BACKENDS: &str = include_str!("../../../../src/log/backend/mod.rs");
const SERIAL: &str = include_str!("../../../../src/log/backend/serial.rs");
const BOTH: &str = r#"any(feature = "nonos-capsule-attack", feature = "capsule-serial-debug")"#;

#[test]
fn a_tag_is_echoed_per_second_up_to_its_budget_and_the_rest_is_counted() {
    let mut b = Budget::new();
    let line = b"[IPC-DENY] pid=9 ipc to power";
    for _ in 0..PER_SECOND {
        assert_eq!(b.admit(line, 5_000), Echo::Line);
    }
    for _ in 0..1000 {
        assert_eq!(b.admit(line, 5_999), Echo::Not, "the same second");
    }
    assert_eq!(b.admit(line, 6_000), Echo::After(1000), "the next second says the count");
    assert_eq!(b.admit(line, 6_001), Echo::Line, "said once");
    assert_eq!(b.admit(line, 9_000), Echo::Line, "nothing held, nothing said");
}

#[test]
fn tags_spend_their_own_budgets_and_past_the_slots_share_one() {
    let mut b = Budget::new();
    for _ in 0..PER_SECOND {
        assert_eq!(b.admit(b"[CAP-DENY] a", 0), Echo::Line);
    }
    assert_eq!(b.admit(b"[CAP-DENY] b", 0), Echo::Not);
    assert_eq!(b.admit(b"[IPC-DENY] c", 0), Echo::Line, "another tag's budget");
    assert_eq!(b.admit(b"no tag", 0), Echo::Line);
    let names: Vec<String> =
        (0..SLOTS + PER_SECOND as usize + 4).map(|i| format!("[T{i}] x")).collect();
    let echoed = names.iter().filter(|n| b.admit(n.as_bytes(), 0) == Echo::Line).count();
    assert!(echoed < names.len(), "tags past the slots share the last one");
    assert_eq!(
        (tag(b"[A] x"), tag(b"x [A]"), tag(b"[unclosed")),
        (&b"[A]"[..], &b""[..], &b""[..])
    );
}

#[test]
fn the_echo_is_registered_on_the_attack_suite_and_the_serial_debug_images_only() {
    assert_eq!(LIFECYCLE.matches("SerialBackend").count(), 1);
    let at = LIFECYCLE.find("SerialBackend").unwrap();
    assert!(LIFECYCLE[..at].rfind(&format!("#[cfg({BOTH})]")).is_some_and(|c| at - c < 200));
    assert_eq!(BACKENDS.matches(&format!("#[cfg({BOTH})]")).count(), 2, "mod and use");
    assert!(!LIFECYCLE.contains("cfg(feature = \"nonos-capsule-attack\")]"), "the old gate");
    let budget = SERIAL.find("self.budget.admit(").expect("the budget asked");
    assert!(budget < SERIAL.find("Line::new()").expect("the echo"));
}
