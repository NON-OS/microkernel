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

//! `attest` and `pcr`: what the kernel recorded of this boot's measurement,
//! in the terminal's words: the loader's verdicts, the proof that admitted
//! the kernel, and the kernel's and the calling program's digests.

use alloc::vec::Vec;

use super::hex::encode;
use crate::host::{Attest, Host, PROOF_PATH_ONLY, PROOF_STARK};
use crate::out::ShellOut;

pub const NO_RECORD: &[u8] = b"attest: the kernel recorded no measured-boot state";

pub fn run(out: &mut dyn ShellOut, host: &mut dyn Host) -> bool {
    let Some(att) = host.attest() else {
        out.writeln(NO_RECORD);
        return false;
    };
    verdict(out, b"kernel signature", att.kernel_sig_ok);
    verdict(out, b"secure boot", att.secure_boot);
    verdict(out, b"attestation", att.attested);
    out.writeln(proof_line(att.proof));
    digests(out, &att);
    true
}

pub fn pcr(out: &mut dyn ShellOut, host: &mut dyn Host) -> bool {
    let Some(att) = host.attest() else {
        out.writeln(NO_RECORD);
        return false;
    };
    digests(out, &att);
    true
}

fn verdict(out: &mut dyn ShellOut, label: &[u8], ok: bool) {
    let mut line = Vec::from(label);
    line.resize(18, b' ');
    line.extend_from_slice(if ok { b"yes" } else { b"no" });
    out.writeln(&line);
}

fn proof_line(proof: u8) -> &'static [u8] {
    match proof {
        PROOF_STARK => b"proof             stark",
        PROOF_PATH_ONLY => b"proof             merkle path (development kernel)",
        _ => b"proof             none recorded",
    }
}

fn digests(out: &mut dyn ShellOut, att: &Attest) {
    out.writeln(&[&b"kernel  "[..], &encode(&att.kernel_blake3)].concat());
    out.writeln(&[&b"program "[..], &encode(&att.program_hash)].concat());
}
