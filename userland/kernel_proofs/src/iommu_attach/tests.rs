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

use std::cell::RefCell;

use super::attach_steps::attach;

/// Runs the steps against a log, `fail` naming the calls that fail.
fn run(fail: &[&str]) -> (Result<(), &'static str>, Vec<&'static str>) {
    let log = RefCell::new(Vec::new());
    let step = |name: &'static str| {
        log.borrow_mut().push(name);
        if fail.contains(&name) {
            Err(name)
        } else {
            Ok(())
        }
    };
    let invalidations = RefCell::new(0);
    let invalidate = || {
        *invalidations.borrow_mut() += 1;
        step(if *invalidations.borrow() == 1 { "invalidate" } else { "invalidate again" })
    };
    let result = attach(|| step("set"), invalidate, || step("clear"));
    (result, log.into_inner())
}

#[test]
fn a_clean_attach_writes_then_drops_the_caches() {
    assert_eq!(run(&[]), (Ok(()), vec!["set", "invalidate"]));
}

#[test]
fn a_failed_invalidation_takes_the_entry_back_out() {
    let (result, log) = run(&["invalidate"]);
    assert_eq!(result, Err("invalidate"), "the first error is the one reported");
    assert_eq!(log, ["set", "invalidate", "clear", "invalidate again"]);
}

#[test]
fn a_failed_write_touches_nothing_more() {
    assert_eq!(run(&["set"]), (Err("set"), vec!["set"]));
}

#[test]
fn a_clear_that_fails_is_not_followed_by_a_drop() {
    let (result, log) = run(&["invalidate", "clear"]);
    assert_eq!((result, log), (Err("invalidate"), vec!["set", "invalidate", "clear"]));
}
