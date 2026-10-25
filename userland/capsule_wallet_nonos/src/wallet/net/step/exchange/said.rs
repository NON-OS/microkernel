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

//! What a failed exchange says, in words a person can act on.

use nonos_tls::CertProblem;

pub(super) const NO_ANSWER: &str = "the RPC host did not answer";
pub(super) const NO_HELLO: &str = "the TLS hello could not be built";
pub(super) const NOT_TRUSTED: &str = "the RPC host did not prove itself";
pub(super) const NO_CHAIN: &str = "the RPC host's TLS handshake did not finish";
pub(super) const HTTP_REFUSED: &str = "the RPC host answered with an HTTP error";
pub(super) const CUT_SHORT: &str = "the RPC host's answer was cut short";
pub(super) const NO_REPLY: &str = "the RPC host sent nothing back after the request";
pub(super) const CUT_RECORD: &str = "the RPC host's answer stopped partway through a record";
pub(super) const RECORD_FAILED: &str = "the RPC host's answer did not decrypt";
pub(super) const ALERTED: &str = "the RPC host ended the TLS handshake with an alert";
pub(super) const RETRY: &str = "the RPC host asked for a key exchange this wallet does not offer";
pub(super) const UNUSABLE: &str = "the RPC host's TLS hello could not be used";
pub(super) const BROKEN: &str = "the RPC host's TLS handshake did not decrypt";
pub(super) const NO_CLOCK: &str = "this machine's clock could not be read, so the RPC host's \
     certificate dates cannot be checked";
pub(super) const NAME_MISMATCH: &str = "the RPC host's certificate names another host";
pub(super) const EXPIRED: &str = "the RPC host's certificate has expired";
pub(super) const NOT_YET: &str =
    "the RPC host's certificate is not valid yet, or this machine's clock \
     is behind";
pub(super) const UNKNOWN_ISSUER: &str =
    "the RPC host's certificate is from an authority this wallet \
     does not trust";
pub(super) const UNREADABLE: &str = "the RPC host's certificate could not be read";
pub(super) const SLOW: &str = "the RPC host's TLS handshake took longer than the wallet waits";

/* What a person can act on, from the first thing wrong with the chain. */
pub(super) fn refused_why(problem: Option<CertProblem>) -> &'static str {
    match problem {
        Some(CertProblem::NameMismatch) => NAME_MISMATCH,
        Some(CertProblem::Expired) => EXPIRED,
        Some(CertProblem::NotYetValid) => NOT_YET,
        Some(CertProblem::UnknownIssuer) => UNKNOWN_ISSUER,
        Some(CertProblem::Unreadable) => UNREADABLE,
        None => NOT_TRUSTED,
    }
}
