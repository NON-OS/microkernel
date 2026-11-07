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

//! `attest` and `pcr`: the measured-boot state this machine recorded.
//!
//! `mk_attest_status` returns what the loader checked and the two hashes it
//! measured: the kernel's BLAKE3 and this program's. It is read only and needs
//! the AttestRead capability, which this terminal holds; it changes nothing and
//! reveals only what a verifier is entitled to recompute. `attest` prints the
//! verdicts and the hashes; `pcr` prints the two hashes alone, the measured
//! values a remote verifier pins.

use alloc::vec::Vec;

use nonos_libc::{mk_attest_status, AttestStatus, ZK_PATH_ONLY, ZK_STARK};

use crate::command::output::Output;

/// `attest`: the full measured-boot picture. Returns false when the kernel will
/// not answer, so the status says the state is unknown rather than clean.
pub fn run(out: &mut Output<'_>, _argv: &[&[u8]]) -> bool {
    let Some(att) = read() else {
        return refused(out);
    };
    verdict(out, b"kernel signature", att.kernel_sig_ok == 1);
    verdict(out, b"secure boot", att.secure_boot == 1);
    verdict(out, b"attestation", att.zk_attestation_ok == 1);
    out.writeln(proof_line(att.zk_verified));
    labelled(out, b"kernel  ", &att.kernel_blake3);
    labelled(out, b"program ", &att.program_hash);
    true
}

/// `pcr`: the two measured hashes alone, one per line, each named.
pub fn pcr(out: &mut Output<'_>, _argv: &[&[u8]]) -> bool {
    let Some(att) = read() else {
        return refused(out);
    };
    labelled(out, b"kernel  ", &att.kernel_blake3);
    labelled(out, b"program ", &att.program_hash);
    true
}

fn read() -> Option<AttestStatus> {
    let mut att = AttestStatus::default();
    if mk_attest_status(&mut att) != 0 {
        return None;
    }
    Some(att)
}

fn refused(out: &mut Output<'_>) -> bool {
    out.writeln(b"attest: the kernel recorded no measured-boot state");
    false
}

/// A `yes`/`no` verdict row, the accent carried by the words rather than colour
/// so it reads the same in the proofs and on a plain console.
fn verdict(out: &mut Output<'_>, label: &[u8], ok: bool) {
    let mut line = Vec::from(label);
    line.resize(18, b' ');
    line.extend_from_slice(if ok { b"yes" } else { b"no" });
    out.writeln(&line);
}

/// What checked the kernel's proof: a full STARK, a development Merkle path, or
/// nothing recorded.
fn proof_line(zk: u8) -> &'static [u8] {
    match zk {
        ZK_STARK => b"proof             stark",
        ZK_PATH_ONLY => b"proof             merkle path (development kernel)",
        _ => b"proof             none recorded",
    }
}

fn labelled(out: &mut Output<'_>, label: &[u8], hash: &[u8]) {
    let mut line = Vec::from(label);
    line.extend_from_slice(&super::hex::encode(hash));
    out.writeln(&line);
}
