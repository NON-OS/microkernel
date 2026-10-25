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

//! The verdict on a real TPM's quote: one over the values read is taken; a
//! PCR extended between the read and the quote is told as moved; a quote
//! over other qualifying data or another selection is refused.

use super::error::AttestDocError;
use super::produce_judge::judge;
use crate::security::tpm::boot_reads::read_pcrs;
use crate::security::tpm::live::steps::load_test_ak;
use crate::security::tpm::live::swtpm::Swtpm;
use crate::security::tpm::live::tools::tool;
use crate::security::tpm::quote::{build_quote, QuoteError};
use crate::security::tpm::transact_resending;

const QUOTED: [u8; 8] = [0, 1, 2, 4, 7, 8, 9, 14];

fn quote(ak: u32, nonce: &[u8], pcrs: &[u8]) -> Vec<u8> {
    let mut out = [0u8; 4096];
    // SAFETY: the test's own swtpm; the signature is the kernel's.
    let n = unsafe { transact_resending(&build_quote(ak, nonce, pcrs), &mut out) };
    out[..n.expect("sent")].to_vec()
}

#[test]
fn a_quote_is_judged_against_the_values_read_before_it() {
    let Some(t) = Swtpm::start("a_quote_is_judged_against_the_values_read_before_it") else {
        return;
    };
    let (ak, _) = load_test_ak();
    let before = read_pcrs(&QUOTED).expect("read");
    let honest = judge(&[0x71; 32], before, &quote(ak, &[0x71; 32], &QUOTED)).expect("judged");
    assert_eq!(honest.expect("signed what was read").pcrs, before);
    let other_nonce = judge(&[0x72; 32], before, &quote(ak, &[0x71; 32], &QUOTED));
    assert_eq!(other_nonce.map(|_| ()), Err(AttestDocError::Quote(QuoteError::NonceMismatch)));
    let other_pcrs = judge(&[0x71; 32], before, &quote(ak, &[0x71; 32], &[0, 1, 2, 7]));
    assert_eq!(other_pcrs.map(|_| ()), Err(AttestDocError::Quote(QuoteError::Selection)));
    let event: String = "a5".repeat(32);
    tool(Some(&t), "tpm2_pcrextend", &[&format!("4:sha256={event}")]);
    let moved = judge(&[0x71; 32], before, &quote(ak, &[0x71; 32], &QUOTED)).expect("judged");
    assert!(moved.is_none(), "PCR 4 moved after it was read");
}
