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

//! The kernel's `TPM2_PCR_Extend` against a real TPM 2.0: it moves both of
//! a PC's banks exactly as the reference tools' extend of the same digests
//! does, and a bank the TPM does not implement leaves the PCR as it was.

use super::swtpm::Swtpm;
use super::tools::{hex, tool};
use crate::security::tpm::boot_reads::read_pcrs;
use crate::security::tpm::pcr_extend::pcr_extend;

/// PCRs 14 and 15 in `bank`, as the reference tools read them.
fn pair(t: &Swtpm, bank: &str) -> String {
    String::from_utf8(tool(Some(t), "tpm2_pcrread", &[&format!("{bank}:14,15")])).expect("text")
}

#[test]
fn an_extend_moves_each_bank_as_the_reference_does() {
    let Some(t) = Swtpm::start("an_extend_moves_each_bank_as_the_reference_does") else {
        return;
    };
    let (sha1, sha256) = ([0x5A; 20], [0xC3; 32]);
    for _ in 0..2 {
        pcr_extend(14, &[(0x0004, &sha1), (0x000B, &sha256)]).expect("extended");
        let both = format!("15:sha1={},sha256={}", hex(&sha1), hex(&sha256));
        tool(Some(&t), "tpm2_pcrextend", &[&both]);
    }
    for bank in ["sha1", "sha256"] {
        let text = pair(&t, bank);
        let values: Vec<&str> = text.lines().filter_map(|l| l.split(": 0x").nth(1)).collect();
        assert_eq!(values.len(), 2, "{text}");
        assert_eq!(values[0], values[1], "{bank}: PCR 14 against the reference's PCR 15");
    }
    let read = read_pcrs(&[14, 15]).expect("read");
    assert_eq!(read[0], read[1]);
    assert_ne!(read[0], [0; 32]);
}

#[test]
fn a_bank_the_tpm_does_not_implement_extends_nothing() {
    let Some(t) = Swtpm::start("a_bank_the_tpm_does_not_implement_extends_nothing") else {
        return;
    };
    let before = read_pcrs(&[14]).expect("read");
    let refused = pcr_extend(14, &[(0x000B, &[0xC3; 32]), (0x00FE, &[0; 32])]);
    assert!(refused.is_err(), "an unknown bank is refused");
    assert_eq!(read_pcrs(&[14]).expect("read"), before, "and the SHA-256 bank is unmoved");
    drop(t);
}
