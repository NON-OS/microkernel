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

//! `attest_doc`'s PCR read and quote, on a real TPM 2.0: the values the
//! document carries are the ones the quote signed, whatever was extended
//! before, and the quote answers the qualifying data it was given. A PCR
//! that moves between the read and the quote is read again, once.

use sha2::{Digest, Sha256};

use super::error::AttestDocError;
use super::produce_quote::{quote_pcrs, quote_with};
use crate::security::tpm::boot_reads::read_pcrs;
use crate::security::tpm::live::steps::load_test_ak;
use crate::security::tpm::live::swtpm::Swtpm;
use crate::security::tpm::live::tools::tool;
use crate::security::tpm::quote::{build_quote, check_attest, covered};
use crate::security::tpm::transact_resending;

const QUOTED: [u8; 8] = [0, 1, 2, 4, 7, 8, 9, 14];

#[test]
fn the_document_s_values_are_the_ones_the_quote_signed() {
    let Some(t) = Swtpm::start("the_document_s_values_are_the_ones_the_quote_signed") else {
        return;
    };
    for pcr in [0, 4, 9, 14] {
        let event: String = core::iter::repeat_n(format!("{pcr:02x}"), 32).collect();
        tool(Some(&t), "tpm2_pcrextend", &[&format!("{pcr}:sha256={event}")]);
    }
    let (ak, _) = load_test_ak();
    let q = quote_pcrs(ak, &[0x6A; 32]).expect("read and quoted");
    assert_eq!(q.pcrs, read_pcrs(&QUOTED).expect("read"));
    assert!(q.pcrs[1] == [0; 32] && q.pcrs[3] != [0; 32], "PCR 1 untouched, PCR 4 extended");
    check_attest(&q.attest, &[0x6A; 32]).expect("over the qualifying data");
    let c = covered(&q.attest).expect("one selection");
    assert_eq!(c.digest, &Sha256::digest(q.pcrs.concat())[..]);
    assert!(!q.signature.is_empty());
}

#[test]
fn a_pcr_moved_once_is_read_again_and_one_that_keeps_moving_is_refused() {
    let Some(t) = Swtpm::start("a_pcr_moved_once_is_read_again") else { return };
    let (ak, _) = load_test_ak();
    let quotes = core::cell::Cell::new(0);
    let read = || read_pcrs(&QUOTED).map_err(|_| AttestDocError::PcrRead);
    let quote = |moves: bool, buf: &mut [u8]| {
        quotes.set(quotes.get() + 1);
        if moves {
            tool(Some(&t), "tpm2_pcrextend", &[&format!("4:sha256={}", "4c".repeat(32))]);
        }
        // SAFETY: the test's own swtpm; the signature is the kernel's.
        unsafe { transact_resending(&build_quote(ak, &[0x6B; 32], &QUOTED), buf) }
            .map_err(AttestDocError::Tpm)
    };
    let once = quote_with(&[0x6B; 32], read, |b| quote(quotes.get() == 0, b));
    assert!(once.is_ok() && quotes.get() == 2, "read again, then signed");
    quotes.set(0);
    let always = quote_with(&[0x6B; 32], read, |b| quote(true, b));
    assert_eq!(always.map(|_| ()), Err(AttestDocError::PcrsMoved));
    assert_eq!(quotes.get(), 2, "twice, then refused");
}
