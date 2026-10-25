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

//! The PCRs the attestation document carries, read from a real TPM 2.0:
//! the values after extends, and a selection larger than one answer holds.

use sha2::{Digest, Sha256};

use super::swtpm::Swtpm;
use super::tools::tool;
use crate::security::tpm::boot_reads::read_pcrs;

/// What `attest_doc::produce` quotes, in this order.
pub(super) const QUOTED: [u8; 8] = [0, 1, 2, 4, 7, 8, 9, 14];

/// PCR `pcr` from zero, extended once by `event`.
fn once(event: &[u8; 32]) -> [u8; 32] {
    Sha256::new().chain_update([0u8; 32]).chain_update(event).finalize().into()
}

#[test]
fn eight_pcrs_read_as_extended() {
    let Some(t) = Swtpm::start("eight_pcrs_read_as_extended") else { return };
    assert_eq!(read_pcrs(&QUOTED).expect("read"), [[0; 32]; 8], "a new TPM");
    let mut want = [[0u8; 32]; 8];
    for (k, pcr) in QUOTED.iter().enumerate() {
        let event = [*pcr ^ 0xA5; 32];
        tool(Some(&t), "tpm2_pcrextend", &[&format!("{pcr}:sha256={}", hex(&event))]);
        want[k] = once(&event);
    }
    assert_eq!(read_pcrs(&QUOTED).expect("read"), want);
    let reversed: [u8; 8] = core::array::from_fn(|i| QUOTED[7 - i]);
    let back = read_pcrs(&reversed).expect("read");
    assert!((0..8).all(|i| back[i] == want[7 - i]), "in the order asked for");
}

#[test]
fn a_selection_larger_than_one_answer_is_read_in_rounds() {
    let Some(t) = Swtpm::start("a_selection_larger_than_one_answer_is_read_in_rounds") else {
        return;
    };
    tool(Some(&t), "tpm2_pcrextend", &[&format!("9:sha256={}", hex(&[9; 32]))]);
    tool(Some(&t), "tpm2_pcrextend", &[&format!("16:sha256={}", hex(&[16; 32]))]);
    let twelve = [0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 16, 23];
    let got = read_pcrs(&twelve).expect("twelve, over two rounds");
    assert_eq!((got[9], got[10]), (once(&[9; 32]), once(&[16; 32])));
    assert!(got[..9].iter().chain([&got[11]]).all(|v| *v == [0; 32]));
}

pub(super) fn hex(b: &[u8]) -> String {
    b.iter().map(|x| format!("{x:02x}")).collect()
}
