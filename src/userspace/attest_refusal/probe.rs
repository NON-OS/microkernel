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

use super::embed::*;
use crate::capabilities::Capability;
use crate::kernel_core::process_spawn::capsule_spawn::{self, CapsuleSpecVerified, SpawnError};
use crate::security::nonos_trust_anchor::{decode as decode_trust_anchor, BAKED_TRUST_ANCHOR_POLICY};
use crate::userspace::capsule_proof_io::embed as honest;

/// Spawn each broken variant, which the gate must refuse, then the honest
/// capsule, which it must admit. One line per outcome, for the harness.
pub fn run() {
    let Ok(anchor) = decode_trust_anchor(BAKED_TRUST_ANCHOR_POLICY) else {
        return say(b"trust anchor", false);
    };
    let base = Capability::CoreExec.bit() | Capability::IPC.bit() | Capability::Memory.bit();
    let with_io = base | Capability::IO.bit();
    let cases: [(&[u8], &[u8], &[u8], &[u8], u64); 4] = [
        (b"flip", honest::PROOF_IO_NONOS_ID_CERT_BYTES, honest::PROOF_IO_MANIFEST_BYTES, FLIP_TRAILER, base),
        (b"extra_cap", EXTRA_CAP_CERT, EXTRA_CAP_MANIFEST, honest::PROOF_IO_ATTESTATION_BYTES, with_io),
        (b"kernel_kind", honest::PROOF_IO_NONOS_ID_CERT_BYTES, honest::PROOF_IO_MANIFEST_BYTES, KERNEL_KIND_TRAILER, base),
        (b"stale_epoch", honest::PROOF_IO_NONOS_ID_CERT_BYTES, honest::PROOF_IO_MANIFEST_BYTES, STALE_EPOCH_TRAILER, base),
    ];
    for (name, cert, manifest, trailer, caps) in cases {
        let r = capsule_spawn::spawn_verified(&spec(cert, manifest, trailer, caps), &anchor, None);
        say(name, matches!(r, Err(SpawnError::AttestationRejected)));
        super::attack_line::attack_line(name, &r, trailer, honest::PROOF_IO_ELF, caps);
    }
    let honest_spec = spec(
        honest::PROOF_IO_NONOS_ID_CERT_BYTES,
        honest::PROOF_IO_MANIFEST_BYTES,
        honest::PROOF_IO_ATTESTATION_BYTES,
        base,
    );
    say(b"honest", capsule_spawn::spawn_verified(&honest_spec, &anchor, None).is_ok());
}

fn spec<'a>(cert: &'a [u8], manifest: &'a [u8], trailer: &'a [u8], caps: u64) -> CapsuleSpecVerified<'a> {
    CapsuleSpecVerified {
        name: "proof_io",
        service_port: 4500,
        reply_inbox: "endpoint.proof_io.reply",
        reply_port: 4501,
        elf: honest::PROOF_IO_ELF,
        nonos_id_cert_bytes: cert,
        manifest_bytes: manifest,
        attestation_trailer: trailer,
        target_triple: env!("NONOS_USER_TARGET"),
        requested_caps: caps,
        debug_tag: b"",
    }
}

/* The harness reads these lines: "as expected" for each case, or the case is a failure. */
fn say(case: &[u8], as_expected: bool) {
    crate::sys::serial::print(b"[ATTEST-PROBE] ");
    crate::sys::serial::print(case);
    crate::sys::serial::print(if as_expected { b" as expected\n" } else { b" UNEXPECTED\n" });
}
