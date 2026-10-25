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

//! The real periodic cache, when the build names it by NONOS_PERIODIC_CACHE
//! (tools/nix/periodic.nix's periodic.top, embedded as cache::SHIPPED): its
//! sha256 pinned, and taken by the capsule's own check, which refuses it with
//! one bit of its root changed. A build naming none says so on stderr, naming the variable,
//! past the harness's capture; the lane fails on that line (README.md).

use std::io::Write;

use super::json::{failed, lines, passed};
use super::top::sha256;
use crate::cache::{self, SHIPPED};

const SHIPPED_SHA256: &str = "b2d95883f63dd439ac7bc5ea92b6c6e621aa37d2a67707b8342cce05b0bcd96f";

/// The shipped cache, or None once the build is seen to name none.
pub(super) fn shipped(test: &str) -> Option<&'static [u8]> {
    if option_env!("NONOS_PERIODIC_CACHE").is_some() {
        return Some(SHIPPED);
    }
    assert!(SHIPPED.is_empty(), "a cache was embedded that no build named");
    let skipped = format!("{test}: NONOS_PERIODIC_CACHE not set, real cache test skipped");
    let _ = writeln!(std::io::stderr(), "{skipped}");
    None
}

#[test]
fn the_shipped_cache_is_the_pinned_file() {
    let Some(top) = shipped("the_shipped_cache_is_the_pinned_file") else { return };
    assert_eq!(top.len(), 8_388_660);
    assert_eq!(sha256(top), SHIPPED_SHA256, "NONOS_PERIODIC_CACHE is not the pinned periodic.top");
}

#[test]
fn the_capsules_check_takes_it_and_refuses_a_touched_copy() {
    let Some(top) = shipped("the_capsules_check_takes_it_and_refuses_a_touched_copy") else {
        return;
    };
    let (ok, text) = lines(cache::shipped_cache);
    assert!(ok && failed(&text).is_empty(), "{text}");
    assert_eq!(passed(&text).len(), 2, "{text}");
    assert!(nox_prover::load_cache(top).is_ok());
    let mut touched = top.to_vec();
    *touched.last_mut().expect("a non-empty cache") ^= 0x80;
    assert!(nox_prover::load_cache(&touched).is_err());
}
