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

//! A capsule's call sequence through the syscall gate's pure stages: the
//! number decode, the capability table, and the user-copy bounds every
//! pointer argument passes. Each step is 32 bytes: number, capability bits,
//! address, length. The gate holds no state between calls, so a sequence
//! checks that a decision depends only on the token and the number, and that
//! no step of it reaches a state the capability model forbids.

#![no_main]

use kernel_proofs::capabilities::bits::{bits_to_caps, caps_to_bits};
use kernel_proofs::capabilities::{Capability, CapabilityToken};
use kernel_proofs::syscall::abi::REGISTRY;
use kernel_proofs::syscall::contract::{is_allowed, is_allowed_as_shell};
use kernel_proofs::syscall::numbers::SyscallNumber;
use kernel_proofs::usercopy::{check, MAX_COPY_SIZE, USER_SPACE_END};
use libfuzzer_sys::fuzz_target;

fn token(bits: u64) -> CapabilityToken {
    CapabilityToken {
        owner_module: 0,
        permissions: bits_to_caps(bits),
        expires_at_ms: None,
        nonce: 0,
        signature: [0u8; 64],
        token_id: 0,
        subject_capsule_id: 0,
        subject_asid: 0,
        subject_measurement: [0u8; 32],
        boot_session_nonce: [0u8; 16],
        revocation_epoch: 0,
        delegation_depth: 0,
    }
}

fn network() -> u64 {
    caps_to_bits(&[Capability::Network])
}

fn word(b: &[u8]) -> u64 {
    let mut w = [0u8; 8];
    w.copy_from_slice(&b[..8]);
    u64::from_le_bytes(w)
}

fn decode(id: u64) -> Option<SyscallNumber> {
    let found = SyscallNumber::from_u64(id);
    let mut named = REGISTRY.iter().flat_map(|d| d.iter()).filter(|e| e.id == id);
    match (found, named.next()) {
        (None, None) => None,
        (Some(v), Some(first)) => {
            assert!(v == first.variant, "decode of {id:#x} is not the registry's entry");
            assert!(named.all(|e| e.variant == v), "two registry entries share {id:#x}");
            Some(v)
        }
        _ => panic!("decode and the registry disagree on {id:#x}"),
    }
}

fn gate(number: SyscallNumber, bits: u64, extra: u64) {
    let caps = caps_to_bits(&bits_to_caps(bits));
    assert_eq!(caps_to_bits(&bits_to_caps(caps)), caps, "bit conversion is not stable");
    let t = token(caps);
    let allowed = is_allowed(&t, number);
    // The same token and number give the same answer, every time.
    assert_eq!(allowed, is_allowed(&t, number));
    // Holding more takes access away in one place only: a TPM quote names
    // the machine, so MkAttestDoc is withdrawn from any token that reaches
    // the network. Everywhere else a wider token keeps what it had.
    let wide = caps_to_bits(&bits_to_caps(caps | extra));
    if allowed && !is_allowed(&token(wide), number) {
        assert!(number == SyscallNumber::MkAttestDoc, "a wider token lost {number:?}");
        assert!(wide & network() != 0 && caps & network() == 0);
    }
    if number == SyscallNumber::MkAttestDoc && caps & network() != 0 {
        assert!(!allowed && !is_allowed_as_shell(&t, number), "a quote reached a networked token");
    }
    // The shell gets at least what any other caller with the same token gets.
    if allowed {
        assert!(is_allowed_as_shell(&t, number), "the shell lost {number:?}");
    }
    // An empty token reaches only the call that reports on its own streams
    // and random bytes, as gate_tests hold the cap table to.
    if is_allowed(&token(0), number) {
        let open = [SyscallNumber::MkTtyQuery, SyscallNumber::CryptoRandom];
        assert!(open.contains(&number), "{number:?} open to an empty token");
    }
}

fn bounds(addr: u64, len: u64) {
    let len = len as usize;
    if let Ok(range) = check(addr, len) {
        assert!(addr != 0 && len <= MAX_COPY_SIZE);
        if let Some((first, last)) = range {
            let end = addr + (len as u64 - 1);
            assert!(end <= USER_SPACE_END, "accepted a copy into the kernel half");
            assert!(first <= addr && last <= end && end - last < 4096);
            assert!(first % 4096 == 0 && last % 4096 == 0);
        } else {
            assert_eq!(len, 0);
        }
    }
}

fuzz_target!(|data: &[u8]| {
    for step in data.chunks_exact(32) {
        let id = word(&step[0..]);
        let bits = word(&step[8..]);
        let addr = word(&step[16..]);
        let len = word(&step[24..]);
        if let Some(number) = decode(id) {
            gate(number, bits, len);
        }
        // Small lengths reach the accepted path more often than a uniform u64.
        bounds(addr, len & 0x0fff_ffff);
        bounds(addr, len);
    }
});
