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

//! What the kernel records for a family when it accepts a proof is the plain
//! BLAKE3 of the bytes it was given: no domain, no trailer, no capabilities.
//! The anonymity fronts grant a Linux family the network by finding the hash
//! of what was vouched for the policy, its domain and its bytes, and the
//! policy's program hash among those records (userland/linux_net_grant
//! by_policy.rs), so the two must be the same hash of the same bytes or every
//! grant fails closed. Each record keeps the call that verified it beside it,
//! and a policy is looked for under a local trailer only.

use super::digest::in_chunks;

const LOCAL_VERIFY: &str = include_str!("../../../../src/syscall/microkernel/local_verify.rs");
const SIGN: &str = include_str!("../../../../src/security/local_build/sign.rs");
const TRAILER: &str = include_str!("../../../../src/security/local_build/trailer.rs");
const PATH: &str = include_str!("../../../../src/security/capsule_attest/path.rs");
const GATE: &str = include_str!("../../../../src/syscall/microkernel/capsule_verify/gate.rs");
const FACT: &str = include_str!("../../../../src/syscall/microkernel/family_fact/mod.rs");
const BY_POLICY: &str = include_str!("../../../linux_net_grant/src/by_policy.rs");

#[test]
fn the_kernels_image_digest_is_the_grants_hash_of_the_same_bytes() {
    for bytes in [vec![0x5a; 256], b"NLXPOL01 a policy".to_vec(), vec![7; (1 << 20) + 3]] {
        let read = |at: u64, buf: &mut [u8]| -> Result<(), ()> {
            buf.copy_from_slice(&bytes[at as usize..at as usize + buf.len()]);
            Ok(())
        };
        let kernel = in_chunks(bytes.len() as u64, 64 << 10, read, || {}).unwrap();
        assert_eq!(kernel, *blake3::hash(&bytes).as_bytes(), "{} bytes", bytes.len());
    }
    assert!(BY_POLICY.contains("facts.holds(pid, &vouched_hash(policy), LOCAL)"));
    assert!(BY_POLICY.contains("facts.holds(pid, &p.program_blake3, PROVED)"));
}

#[test]
fn the_measurement_recorded_is_that_digest_unchanged() {
    // MkLocalVerify hashes the image (digest_in, the file above) and records
    // what the verify gives back.
    assert!(LOCAL_VERIFY.contains("let digest = match digest_in(elf_ptr, elf_len) {"));
    assert!(LOCAL_VERIFY.contains("crate::process::foreign::seal::measured(&p.measurement, LOCAL);"));
    // A local trailer gives back the first 32 bytes of its context, which are
    // the digest as given.
    assert!(SIGN.contains("measurement.copy_from_slice(&ctx[..32]);"));
    assert!(TRAILER.contains("ctx[..32].copy_from_slice(digest);"));
    // A vendor or developer path gives back the digest itself.
    assert_eq!(PATH.matches("Ok(*digest)").count(), 3);
}

#[test]
fn each_measurement_is_kept_under_the_call_that_verified_it() {
    assert!(LOCAL_VERIFY.contains("use crate::process::foreign::seal::kind::LOCAL;"));
    assert!(GATE.contains("use crate::process::foreign::seal::kind::PUBLISHER;"));
    assert!(GATE.contains("crate::process::foreign::seal::measured(&p.measurement, PUBLISHER);"));
    assert!(FACT.contains("let Some(kinds) = asked_kinds(len) else { return ERRNO_INVAL };"));
    assert!(FACT.contains("Some(m) => i64::from(seal::holds(pid, &m, kinds)),"));
    assert!(FACT.contains("u8::try_from(len).ok().filter(|k| *k != 0 && *k & !seal::kind::ALL == 0)"));
}
