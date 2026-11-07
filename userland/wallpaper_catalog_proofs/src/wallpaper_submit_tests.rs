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

//! The wallpaper's scene submit: a late answer is taken as done, since the
//! kernel delivered the request and the compositor serves it, so the
//! wallpaper does not submit again and have the whole screen composed again
//! (capsule_wallpaper state/submit_rule.rs, server/scene/register.rs).

use crate::wallpaper_submit_rule::taken;

const TIMED_OUT: &str = "compositor call timed out";
const REGISTER: &str = include_str!("../../capsule_wallpaper/src/server/scene/register.rs");
const WIRE: &str = include_str!("../../capsule_wallpaper/src/compositor_client/wire.rs");
const STATUS: &str = include_str!("../../capsule_wallpaper/src/compositor_client/wire/status.rs");

#[test]
fn an_answered_or_late_submit_is_taken() {
    assert!(taken(Ok(()), TIMED_OUT));
    assert!(taken(Err(TIMED_OUT), TIMED_OUT), "late: the compositor still has it");
}

#[test]
fn a_refused_or_undelivered_submit_is_asked_again() {
    for e in ["compositor rejected scene_submit", "compositor call failed", "short reply", ""] {
        assert!(!taken(Err(e), TIMED_OUT), "{e:?}");
    }
}

/// The rule is asked with the call's own timeout error, the one the wire
/// returns when the kernel answers ETIMEDOUT and for nothing else.
#[test]
fn the_submit_asks_the_rule_with_the_wire_timeout() {
    assert!(WIRE.contains(&format!("pub(crate) const TIMED_OUT: &str = \"{TIMED_OUT}\";")));
    assert!(STATUS.contains("if rc == ETIMEDOUT {\n        return Err(TIMED_OUT);\n    }"));
    assert_eq!(STATUS.matches("Err(TIMED_OUT)").count(), 1, "the timeout alone says it");
    assert!(REGISTER.contains("taken(answer, TIMED_OUT)"));
    assert!(!REGISTER.contains(".is_ok()\n}"), "a late answer no longer counts as a failure");
}
