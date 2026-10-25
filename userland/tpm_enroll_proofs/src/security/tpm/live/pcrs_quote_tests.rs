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

//! A quote over the document's selection covers exactly it, and its
//! digest is that of the values the kernel reads beside it.

use sha2::{Digest, Sha256};

use super::pcrs_tests::{hex, QUOTED};
use super::steps::load_test_ak;
use super::swtpm::Swtpm;
use super::tools::tool;
use crate::security::tpm::boot_reads::read_pcrs;
use crate::security::tpm::quote::{build_quote, check_attest, covered, parse_quote};
use crate::security::tpm::transact_resending;

#[test]
fn a_quote_covers_the_selection_and_the_values_read() {
    let Some(t) = Swtpm::start("a_quote_covers_the_selection_and_the_values_read") else {
        return;
    };
    tool(Some(&t), "tpm2_pcrextend", &[&format!("4:sha256={}", hex(&[4; 32]))]);
    let (ak, _) = load_test_ak();
    let values = read_pcrs(&QUOTED).expect("read");
    let mut out = [0u8; 4096];
    // SAFETY: the test's own swtpm; the signature is the kernel's.
    let n = unsafe { transact_resending(&build_quote(ak, &[0x5C; 32], &QUOTED), &mut out) };
    let quote = parse_quote(&out[..n.expect("sent")]).expect("a quote");
    check_attest(quote.attest, &[0x5C; 32]).expect("over the nonce");
    let c = covered(quote.attest).expect("covers one selection");
    assert_eq!((c.hash, c.select), (0x000B, &[0x97, 0x43, 0x00][..]));
    assert_eq!(c.digest, &Sha256::digest(values.concat())[..]);
}
