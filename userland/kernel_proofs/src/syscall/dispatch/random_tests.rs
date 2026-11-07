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

//! a project decision 2026-11-02: random bytes need no capability, every other
//! Crypto call stays gated. A capsule whose capability word is empty is
//! given the allowlist install derives for it (the token signed, its MAC
//! checked, the kernel's tables asked), and the kernel's CryptoRandom
//! handler serves it real bytes.

use std::collections::BTreeSet;
use std::sync::atomic::Ordering;

use super::random::handle_crypto_random;
use crate::capabilities::CapabilityToken;
use crate::security::entropy_capsule::fast::SEEDED;
use crate::syscall::abi::REGISTRY;
use crate::syscall::contract::{allowlist, allowlist_for};
use crate::usercopy::user_written;

const BUF: u64 = 0x4000_0000;

fn crypto_calls() -> Vec<(&'static str, u64)> {
    REGISTRY
        .iter()
        .flat_map(|s| s.iter())
        .filter(|e| e.name.starts_with("Crypto"))
        .map(|e| (e.name, e.id))
        .collect()
}

#[test]
fn an_empty_word_may_draw_random_bytes_and_no_other_crypto_call() {
    let list = allowlist_for(&CapabilityToken::with_caps(&[]));
    let calls = crypto_calls();
    assert_eq!(calls.len(), 12, "{calls:?}");
    for (name, id) in calls {
        let want = name == "CryptoRandom";
        assert_eq!(allowlist::allowed(&list, id, false), want, "{name}");
        assert_eq!(allowlist::allowed(&list, id, true), want, "{name}, a launcher");
    }
}

#[test]
fn an_unsigned_empty_word_draws_nothing() {
    // A token whose MAC fails gets the empty list: random bytes included.
    let list = allowlist::Allowlist::EMPTY;
    for (name, id) in crypto_calls() {
        assert!(!allowlist::allowed(&list, id, false), "{name}");
    }
    let unsigned =
        crate::syscall::contract::allowlist_for_unsigned(&CapabilityToken::with_caps(&[]));
    for (name, id) in crypto_calls() {
        assert!(!allowlist::allowed(&unsigned, id, false), "{name}");
    }
}

fn draw(len: u64) -> Vec<u8> {
    let r = handle_crypto_random(BUF, len);
    assert_eq!(r.value, len as i64);
    assert!(!r.audit_required);
    let mut out = user_written();
    assert_eq!(out.len(), 1);
    let (at, bytes) = out.remove(0);
    assert_eq!(at, BUF);
    assert_eq!(bytes.len() as u64, len);
    bytes
}

#[test]
fn the_handler_serves_real_bytes_with_no_capability_check() {
    SEEDED.store(true, Ordering::Relaxed);
    let mut seen = BTreeSet::new();
    for _ in 0..64 {
        let b = draw(32);
        assert!(b.iter().any(|&x| x != 0));
        assert!(seen.insert(b), "two 32-byte draws were the same");
    }
    let big = draw(4096);
    let distinct: BTreeSet<u8> = big.iter().copied().collect();
    assert!(distinct.len() > 200, "{} distinct byte values in 4096", distinct.len());
}

#[test]
fn an_unseeded_generator_falls_back_to_the_hardware_source() {
    SEEDED.store(false, Ordering::Relaxed);
    let a = draw(32);
    let b = draw(32);
    SEEDED.store(true, Ordering::Relaxed);
    assert_ne!(a, b);
}

#[test]
fn the_handler_keeps_its_bounds() {
    for (buf, len) in [(0, 16), (BUF, 0), (BUF, 4097), (BUF, u64::MAX)] {
        assert_eq!(handle_crypto_random(buf, len).value, -22, "buf {buf:#x} len {len}");
    }
    assert_eq!(handle_crypto_random(0xffff_8000_0000_0000, 16).value, -14);
    assert!(user_written().is_empty());
}
