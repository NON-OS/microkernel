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

//! `version` and `commands` in an nsh session.

use crate::kit::{Lines, Script};
use nonos_shell::builtin::attest::NO_RECORD;
use nonos_shell::host::{Attest, PROOF_PATH_ONLY, PROOF_STARK};
use nonos_shell::session::Session;

fn run(host: &mut Script, line: &str) -> Vec<String> {
    let mut out = Lines::default();
    Session::default().run_line(line.as_bytes(), &mut out, host);
    out.text()
}

fn booted(proof: u8) -> Script {
    let att = Attest {
        kernel_sig_ok: true,
        secure_boot: false,
        attested: true,
        proof,
        kernel_blake3: [0xab; 32],
        program_hash: [0x01; 32],
    };
    Script { attest: Some(att), ..Script::default() }
}

#[test]
fn attest_prints_the_verdicts_the_proof_and_the_digests() {
    let kernel = format!("kernel  {}", "ab".repeat(32));
    let program = format!("program {}", "01".repeat(32));
    assert_eq!(
        run(&mut booted(PROOF_PATH_ONLY), "attest; echo $?"),
        [
            "kernel signature  yes",
            "secure boot       no",
            "attestation       yes",
            "proof             merkle path (development kernel)",
            kernel.as_str(),
            program.as_str(),
            "0",
        ]
    );
    assert_eq!(run(&mut booted(PROOF_STARK), "attest")[3], "proof             stark");
    assert_eq!(run(&mut booted(0), "attest")[3], "proof             none recorded");
    assert_eq!(run(&mut booted(9), "attest")[3], "proof             none recorded");
}

#[test]
fn pcr_prints_the_digests_alone() {
    let out = run(&mut booted(PROOF_STARK), "pcr");
    assert_eq!(
        out,
        [format!("kernel  {}", "ab".repeat(32)), format!("program {}", "01".repeat(32))]
    );
}

#[test]
fn a_host_with_no_record_says_so_and_fails() {
    let none = String::from_utf8_lossy(NO_RECORD).into_owned();
    assert_eq!(run(&mut Script::default(), "attest; echo $?"), [none.as_str(), "1"]);
    assert_eq!(run(&mut Script::default(), "pcr; echo $?"), [none.as_str(), "1"]);
}
