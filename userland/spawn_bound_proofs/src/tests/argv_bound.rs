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

//! Every argv the kernel writes fits ARGV_BYTES_MAX, the largest an install's,
//! and one past it is refused (argv_bound.rs; the setter is argv.rs).

use alloc::string::String;
use alloc::vec::Vec;

use crate::argv_bound::{fits, joined_len, ARGV_BYTES_MAX};

fn words(parts: &[&str]) -> Vec<String> {
    parts.iter().map(|p| String::from(*p)).collect()
}

#[test]
fn the_largest_install_argv_fits() {
    /* src/userspace/capsule_linux/install.rs: install <pkg> <pin> <m> [direct],
    m 640 hex since I-C5; a package name of 128 bytes is past any Alpine one. */
    let name = "p".repeat(128);
    let (pin, m) = ("0".repeat(64), "a".repeat(640));
    let argv = words(&["install", &name, &pin, &m, "direct"]);
    assert!(joined_len(&argv) > 256, "the length a 256-byte read lost");
    assert!(fits(&argv) && joined_len(&argv) * 4 <= ARGV_BYTES_MAX);
}

#[test]
fn the_joined_length_is_what_mk_args_returns() {
    assert_eq!(joined_len(&words(&["install", "vim"])), 11);
    assert_eq!(joined_len(&words(&["run"])), 3);
    assert_eq!(joined_len(&[]), 0);
}

#[test]
fn an_argv_one_byte_past_the_bound_is_refused() {
    let at = words(&[&"x".repeat(ARGV_BYTES_MAX - 2), "y"]);
    assert_eq!(joined_len(&at), ARGV_BYTES_MAX);
    assert!(fits(&at));
    let past = words(&[&"x".repeat(ARGV_BYTES_MAX - 1), "y"]);
    assert!(!fits(&past));
}
