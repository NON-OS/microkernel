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

//! The record as the host keeps it: handed over as it grows, checkpointed,
//! held to its cap with room kept for its end, and signed by the host keys
//! or, if they fail, left to its checkpoints.

use super::good::{check, kinds, received};
use super::record::*;
use crate::kit::host::{config, Keys};
use nsh_client::Verdict;
use nsh_server::{HostKey, HostSigner};
use nsh_wire::EntryBody;

#[test]
fn host_keys_in_memory_sign_with_both_halves() {
    let mut rng = crate::kit::rng::Det::new(9);
    let (ml_public, ml) = nonos_mldsa65::keypair(&mut |b| rng.fill(b)).expect("a key pair");
    let key = HostKey::from_parts(nsh_crypto::Secret::new([7; 32]), ml, &ml_public);
    let digest = [0x42; 64];
    let sig = key.sign(&digest, &mut |b| rng.fill(b)).expect("signed");
    let ed = nonos_ed25519::Signature::from_bytes(&sig.ed);
    assert!(nonos_ed25519::verify(key.ed_public(), &digest, &ed));
    assert!(nonos_mldsa65::verify(key.ml_public(), &digest, &sig.ml));
    assert!(key.sign(&digest, &mut crate::kit::rng::dry).is_none(), "no randomness, no signature");
}

#[test]
fn a_full_record_still_counts_every_byte_and_ends_with_its_exit() {
    let mut run = filled();
    run.host(|s, now, r| s.output(b"late", now, r)).expect("output");
    run.host(|s, now, r| s.command(b"y", now, r)).expect("no room is not an error to the host");
    run.host(|s, now, r| s.exit(3, now, r)).expect("exit");
    let rec = received(&run);
    let k = kinds(&rec);
    assert_eq!(k.last(), Some(&EntryBody::Exit(3)));
    let want = EntryBody::Output { bytes: 4, digest: &nonos_hash::sha256(b"late") };
    assert_eq!(k[k.len() - 2], want, "the output sent at the cap is counted");
    assert!(matches!(
        check(&run, &rec, &[]).expect("checks").verdict,
        Verdict::Complete { end_reason: 6, .. }
    ));
}

#[test]
fn a_clock_that_goes_back_never_takes_the_record_back() {
    let mut run = run_with(config(), Keys::default());
    run.now += 5_000;
    run.host(|s, now, r| s.command(b"later", now, r)).expect("command");
    run.now -= 4_000;
    run.host(|s, now, r| s.command(b"earlier", now, r)).expect("command");
    run.now -= 2_000;
    run.host(|s, now, r| s.exit(0, now, r)).expect("exit");
    let rec = received(&run);
    let v = check(&run, &rec, &[]).expect("the record checks").verdict;
    assert!(matches!(v, Verdict::Complete { end_ms, .. } if end_ms == run.now + 6_000), "{v:?}");
}
