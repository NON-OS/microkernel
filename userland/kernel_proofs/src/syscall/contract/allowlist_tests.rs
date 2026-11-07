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

//! The allowlist gives the capability tables' answer for every syscall in
//! the registry, under every capability class tried, for a caller that
//! launches and one that does not. The tables are the kernel's
//! (`cap_table`), and so are the list, its derivation and its decision.

use super::allowlist::{allowed, depends_on_launcher, Allowlist};
use super::cap_table::is_allowed;
use crate::capabilities::{Capability, CapabilityToken};
use crate::syscall::abi::index::slot_of;
use crate::syscall::abi::REGISTRY;

fn token(caps: &[Capability]) -> CapabilityToken {
    CapabilityToken::with_caps(caps)
}

/// The list as the kernel derives it at install: the token signed, its MAC
/// checked, then the tables asked.
fn derive(t: &CapabilityToken) -> Allowlist {
    crate::capabilities::token::test_key();
    let mut signed = t.clone();
    crate::capabilities::token::sign_token(&mut signed).expect("signing key set");
    super::allowlist::for_token(&signed)
}

/// Every syscall, both launcher answers: list and tables agree.
fn agree(t: &CapabilityToken) {
    let list = derive(t);
    for slice in REGISTRY {
        for e in *slice {
            let id = e.id;
            assert_eq!(
                allowed(&list, id, false),
                is_allowed(t, e.variant, false),
                "{} with {:?}, not a launcher",
                e.name,
                t.permissions
            );
            assert_eq!(
                allowed(&list, id, true),
                is_allowed(t, e.variant, true),
                "{} with {:?}, a launcher",
                e.name,
                t.permissions
            );
            assert_eq!(
                depends_on_launcher(&list, id),
                is_allowed(t, e.variant, false) != is_allowed(t, e.variant, true),
                "{}",
                e.name
            );
        }
    }
}

fn all() -> &'static [Capability] {
    Capability::all()
}

#[test]
fn no_capability_and_an_empty_token() {
    agree(&token(&[]));
}

#[test]
fn every_single_capability() {
    for c in all() {
        agree(&token(&[*c]));
    }
}

#[test]
fn every_pair_of_capabilities() {
    let caps = all();
    for i in 0..caps.len() {
        for j in i + 1..caps.len() {
            agree(&token(&[caps[i], caps[j]]));
        }
    }
}

#[test]
fn every_capability_and_all_but_one() {
    let caps = all();
    agree(&token(caps));
    for skip in 0..caps.len() {
        let rest: Vec<Capability> =
            caps.iter().enumerate().filter(|(i, _)| *i != skip).map(|(_, c)| *c).collect();
        agree(&token(&rest));
    }
}

#[test]
fn random_capability_sets() {
    let caps = all();
    let mut x = 0x2545_f491_4f6c_dd1du64;
    for _ in 0..3_000 {
        x ^= x << 13;
        x ^= x >> 7;
        x ^= x << 17;
        let set: Vec<Capability> =
            caps.iter().enumerate().filter(|(i, _)| x & (1 << (i % 64)) != 0).map(|(_, c)| *c).collect();
        agree(&token(&set));
    }
}

#[test]
fn duplicated_and_reordered_permissions_change_nothing() {
    let caps = all();
    let mut doubled: Vec<Capability> = caps.iter().rev().copied().collect();
    doubled.extend_from_slice(caps);
    agree(&token(&doubled));
    assert_eq!(derive(&token(&doubled)), derive(&token(caps)));
}

#[test]
fn an_unexpired_deadline_reads_as_no_deadline() {
    // The resolver refuses an expired token before it asks the list, so the
    // list is the tables' answer for a token that has not expired.
    let mut t = token(all());
    t.expires_at_ms = Some(u64::MAX);
    agree(&t);
    let mut none = t.clone();
    none.expires_at_ms = None;
    assert_eq!(derive(&t), derive(&none));
}

#[test]
fn numbers_outside_the_registry_are_refused() {
    let list = derive(&token(all()));
    let mut x = 0x1234_5678_9abc_def1u64;
    for _ in 0..100_000 {
        x ^= x << 13;
        x ^= x >> 7;
        x ^= x << 17;
        if slot_of(x).is_none() {
            assert!(!allowed(&list, x, false) && !allowed(&list, x, true));
            assert!(!depends_on_launcher(&list, x));
        }
    }
}

#[test]
fn the_empty_list_permits_nothing() {
    for slice in REGISTRY {
        for e in *slice {
            assert!(!allowed(&Allowlist::EMPTY, e.id, false));
            assert!(!allowed(&Allowlist::EMPTY, e.id, true));
        }
    }
}

#[test]
fn only_app_launch_turns_on_the_launcher() {
    let list = derive(&token(&[Capability::IPC]));
    let turning: Vec<&str> = REGISTRY
        .iter()
        .flat_map(|s| s.iter())
        .filter(|e| depends_on_launcher(&list, e.id))
        .map(|e| e.name)
        .collect();
    assert_eq!(turning, ["MkAppLaunch"]);
}
