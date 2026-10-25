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

//! The fourth word of an install request, and the four checks a market
//! policy passes before the install role vouches for it (plan/30).

use crate::install::market::arg::{market_arg, NotMarket, MARKET_LEN};
use crate::install::market::check::{admit, names_program};
use nonos_linux_policy::{unlisted, LinuxAppPolicy, SyscallSet};

const PROGRAM: &[u8] = b"\x7fELF a market program";

pub(super) fn policy(listing: u8, release: u8, calls: SyscallSet) -> LinuxAppPolicy {
    let program = *blake3::hash(PROGRAM).as_bytes();
    let (listing_id, release_id) = ([listing; 32], [release; 32]);
    LinuxAppPolicy { listing_id, release_id, syscall_set: calls, ..unlisted(program) }
}

pub(super) fn ready() -> SyscallSet {
    SyscallSet::NONE.with(0).with(60).with(231)
}

pub(super) fn market(p: &LinuxAppPolicy, listing: u8, release: u8) -> [u8; MARKET_LEN] {
    let mut m = [0u8; MARKET_LEN];
    m[..256].copy_from_slice(&p.encode());
    m[256..288].fill(listing);
    m[288..320].fill(release);
    m
}

#[test]
fn the_word_is_a_dash_or_640_lowercase_hex() {
    assert_eq!(market_arg(b"-"), Ok(None));
    let m = market(&policy(1, 2, ready()), 1, 2);
    let hex: String = m.iter().map(|b| format!("{b:02x}")).collect();
    assert_eq!(market_arg(hex.as_bytes()), Ok(Some(m)));
    for bad in [hex.to_uppercase(), hex[1..].to_string(), format!("{hex}0"), String::new()] {
        assert_eq!(market_arg(bad.as_bytes()), Err(NotMarket), "{}", bad.len());
    }
    assert_eq!(market_arg(&[b'g'; 640]), Err(NotMarket));
}

#[test]
fn a_ready_policy_naming_its_listing_and_release_is_admitted() {
    let p = policy(1, 2, ready());
    assert_eq!(admit(&market(&p, 1, 2)), Ok(p));
    assert!(names_program(&p, PROGRAM) && !names_program(&p, b"\x7fELF another"));
}

#[test]
fn every_other_policy_fails_the_install() {
    let all_ones = policy(1, 2, SyscallSet::ALL);
    assert_eq!(admit(&market(&all_ones, 1, 2)), Err("its policy is not market ready"));
    let p = policy(1, 2, ready());
    assert!(admit(&market(&p, 3, 2)).is_err() && admit(&market(&p, 1, 3)).is_err());
    let zero = policy(0, 0, ready());
    assert_eq!(admit(&market(&zero, 0, 0)), Err("its policy names no listing or release"));
    let mut bad = market(&p, 1, 2);
    bad[0] = b'X';
    assert_eq!(admit(&bad), Err("its policy is malformed"));
}
