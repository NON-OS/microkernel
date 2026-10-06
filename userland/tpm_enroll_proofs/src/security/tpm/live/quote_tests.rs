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

//! The first quote of a boot. `attest_doc::produce` quotes with the AK, so on
//! a TPM built from the reference code its first quote after each startup is
//! answered `TPM_RC_RETRY`. Sent once, as it used to be, the document fails;
//! through `transact_resending` it carries a quote over the caller's nonce.

use super::resend_tests::sent_once;
use super::steps::load_test_ak;
use super::swtpm::Swtpm;
use super::verify::verifies;
use crate::security::tpm::quote::{build_quote, check_attest, parse_quote};
use crate::security::tpm::transact_resending;

/// The PCRs `attest_doc::produce` quotes.
const QUOTED_PCRS: [u8; 4] = [0, 1, 2, 7];

#[test]
fn the_first_quote_of_a_startup_sent_once_is_retry() {
    let Some(_t) = Swtpm::start("the_first_quote_of_a_startup_sent_once_is_retry") else {
        return;
    };
    let (ak, _) = load_test_ak();
    let cmd = build_quote(ak, &[0x5A; 32], &QUOTED_PCRS);
    assert_eq!(sent_once(&cmd), 0x922, "the quote the document asked for did not run");
}

#[test]
fn the_first_quote_of_a_startup_resent_carries_the_nonce() {
    let Some(_t) = Swtpm::start("the_first_quote_of_a_startup_resent_carries_the_nonce") else {
        return;
    };
    let (ak, public) = load_test_ak();
    let nonce = [0x5B; 32];
    let mut out = [0u8; 4096];
    let cmd = build_quote(ak, &nonce, &QUOTED_PCRS);
    // SAFETY: the test's own swtpm; the signature is the kernel's.
    let n = unsafe { transact_resending(&cmd, &mut out) }.expect("sent");
    let quote = parse_quote(&out[..n]).expect("a quote, the first of this startup");
    check_attest(quote.attest, &nonce).expect("over the nonce it was asked for");
    /* TPMT_SIGNATURE: ECDSA, SHA-256, then r and s, and nothing after them */
    assert_eq!(quote.signature[..4], [0x00, 0x18, 0x00, 0x0B], "the AK's scheme");
    let r = u16::from_be_bytes([quote.signature[4], quote.signature[5]]) as usize;
    let s = u16::from_be_bytes([quote.signature[6 + r], quote.signature[7 + r]]) as usize;
    assert_eq!(quote.signature.len(), 8 + r + s, "the session's answer is not in it");
    let mut rs = [0u8; 64];
    rs[32 - r..32].copy_from_slice(&quote.signature[6..6 + r]);
    rs[64 - s..].copy_from_slice(&quote.signature[8 + r..]);
    assert!(verifies(&public.area, quote.attest, &rs), "a verifier accepts it with the AK");
    for cut in 0..n {
        assert!(parse_quote(&out[..cut]).is_err(), "a quote cut at {cut} is refused");
    }
}
