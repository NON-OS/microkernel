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

//! The deny verdict over the kernel's own capability tables, for every
//! registered syscall: a native capsule is ended exactly when the call is
//! outside what its manifest declares; MkDebug, the Linux personality and
//! a process with no manifest are never ended.

use super::cap_table::is_allowed;
use super::deny::{kill_line, verdict, Verdict};
use crate::capabilities::{bits_to_caps, caps_to_bits, Capability, CapabilityToken};
use crate::syscall::abi::REGISTRY;
use crate::syscall::numbers::SyscallNumber;

fn every() -> impl Iterator<Item = SyscallNumber> {
    REGISTRY.iter().flat_map(|s| s.iter()).map(|e| e.variant)
}

fn expect(envelope: u64, n: SyscallNumber) -> Verdict {
    let declared = CapabilityToken::with_caps(&bits_to_caps(envelope));
    if n == SyscallNumber::MkDebug || is_allowed(&declared, n, true) {
        Verdict::Refuse
    } else {
        Verdict::Kill
    }
}

#[test]
fn a_process_without_a_manifest_is_never_ended() {
    for n in every() {
        assert_eq!(verdict(0, n), Verdict::Refuse, "{n:?}");
    }
}

#[test]
fn the_linux_personality_is_never_ended() {
    let foreign = caps_to_bits(&[Capability::ForeignExec]);
    for envelope in [foreign, foreign | 0x1819, u64::MAX] {
        for n in every() {
            assert_eq!(verdict(envelope, n), Verdict::Refuse, "{n:?} with {envelope:#x}");
        }
    }
}

#[test]
fn mk_debug_is_never_a_kill() {
    for c in Capability::all() {
        if *c == Capability::ForeignExec {
            continue;
        }
        assert_eq!(verdict(caps_to_bits(&[*c]), SyscallNumber::MkDebug), Verdict::Refuse);
    }
}

#[test]
fn every_syscall_under_every_single_capability_manifest() {
    for c in Capability::all() {
        if *c == Capability::ForeignExec {
            continue;
        }
        let envelope = caps_to_bits(&[*c]);
        for n in every() {
            assert_eq!(verdict(envelope, n), expect(envelope, n), "{n:?} with {c:?}");
        }
    }
}

#[test]
fn real_manifests_kill_only_outside_what_they_declare() {
    // calculator 0x1819|0x100, e1000 0xB8038|0x100, a crates.io tool 0x59.
    for envelope in [0x1919u64, 0xB8138, 0x59] {
        let declared = CapabilityToken::with_caps(&bits_to_caps(envelope));
        let mut kills = 0;
        for n in every() {
            let v = verdict(envelope, n);
            assert_eq!(v, expect(envelope, n), "{n:?} with {envelope:#x}");
            if v == Verdict::Kill {
                kills += 1;
                assert!(!is_allowed(&declared, n, true), "{n:?} is declared");
            }
        }
        assert!(kills > 0, "{envelope:#x} declares every syscall");
    }
    // Random bytes are every process's: a tool without Crypto may ask.
    assert_eq!(verdict(0x59, SyscallNumber::CryptoRandom), Verdict::Refuse);
    assert_eq!(verdict(0x59, SyscallNumber::CryptoHash), Verdict::Kill);
}

#[test]
fn the_kill_line_names_capsule_syscall_and_allowlist() {
    let line = kill_line(42, "app.calculator", SyscallNumber::CryptoHash, 0x1919);
    assert_eq!(
        line,
        format!(
            "[CAP-KILL] pid=42 capsule=app.calculator syscall=CryptoHash({:#x}) allowlist=manifest:0x1919",
            SyscallNumber::CryptoHash as u64
        )
    );
}

/// Random bytes need no capability, the empty word included; every other
/// crypto call still needs Crypto (a project decision, 2026-11-02).
#[test]
fn random_bytes_are_everyones_and_nothing_else_of_crypto_is() {
    use super::allowlist::{allowed, Allowlist};
    let crypto: Vec<SyscallNumber> = REGISTRY
        .iter()
        .flat_map(|s| s.iter())
        .filter(|e| e.name.starts_with("Crypto"))
        .map(|e| e.variant)
        .collect();
    assert_eq!(crypto.len(), 12);
    let all_but: Vec<Capability> =
        Capability::all().iter().copied().filter(|c| *c != Capability::Crypto).collect();
    for word in [vec![], vec![Capability::CoreExec], all_but] {
        let t = CapabilityToken::with_caps(&word);
        crate::capabilities::token::test_key();
        let mut signed = t.clone();
        crate::capabilities::token::sign_token(&mut signed).expect("signing key set");
        let list: Allowlist = super::allowlist::for_token(&signed);
        for n in &crypto {
            let want = *n == SyscallNumber::CryptoRandom;
            assert_eq!(is_allowed(&t, *n, true), want, "{n:?} with {word:?}");
            assert_eq!(allowed(&list, *n as u64, false), want, "{n:?} listed with {word:?}");
        }
    }
    let with = CapabilityToken::with_caps(&[Capability::Crypto]);
    for n in &crypto {
        assert!(is_allowed(&with, *n, true), "{n:?} with Crypto");
    }
}
