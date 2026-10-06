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

//! Why a refused certificate was refused, in terms a person can act on.

/// The first thing wrong with a certificate list, in the order the chain
/// walk checks it.
#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub enum CertProblem {
    /// The leaf names other hosts, not the one being reached.
    NameMismatch,
    /// A certificate's notAfter is before the clock.
    Expired,
    /// A certificate's notBefore is after the clock.
    NotYetValid,
    /// The chain ends at an authority that is not in the trust store.
    UnknownIssuer,
    /// The list or a certificate in it could not be read.
    Unreadable,
}

/*
 * Run only after the handshake has already refused, to put a reason on the
 * error page. It never runs before a decision and its answer changes none:
 * the chain walk in chain_walk.rs is what refuses.
 *
 * It follows the walk's order (the leaf's name, the leaf's window, each
 * issuer's window, then the anchor) and stops at the first finding. It
 * checks no signatures: a link whose signature fails is not something a
 * person can act on, so it falls through to `None`, which the caller words
 * as a certificate that could not be verified. The anchor test is the one
 * the walk makes (the top certificate pinned, or its issuer a known root),
 * without the signature over it.
 */
/// What is wrong with the TLS 1.3 Certificate message `body` for `host` at
/// `now`, or `None` if none of the reasons above applies.
pub fn cert_problem(body: &[u8], host: &[u8], now: u64) -> Option<CertProblem> {
    let n = super::cert_count::cert_count(body);
    if n < 1 {
        return Some(CertProblem::Unreadable);
    }
    let Some(leaf) = super::cert_at::cert_at(body, 0) else {
        return Some(CertProblem::Unreadable);
    };
    if !super::cert_dns_match::matches(leaf, host) {
        return Some(CertProblem::NameMismatch);
    }
    for i in 0..n {
        let Some(cert) = super::cert_at::cert_at(body, i) else {
            return Some(CertProblem::Unreadable);
        };
        if let Some(problem) = window_problem(cert, now) {
            return Some(problem);
        }
    }
    let Some(top) = super::cert_at::cert_at(body, n - 1) else {
        return Some(CertProblem::Unreadable);
    };
    if anchored(top) {
        None
    } else {
        Some(CertProblem::UnknownIssuer)
    }
}

fn window_problem(cert: &[u8], now: u64) -> Option<CertProblem> {
    let Some((from, until)) = super::cert_window::cert_window(cert) else {
        return Some(CertProblem::Unreadable);
    };
    if now < from {
        Some(CertProblem::NotYetValid)
    } else if now > until {
        Some(CertProblem::Expired)
    } else {
        None
    }
}

fn anchored(top: &[u8]) -> bool {
    let pinned = super::cert_spki::cert_spki(top)
        .and_then(super::hash_sha256::hash_sha256)
        .is_some_and(|h| super::roots::is_trusted_spki_hash(&h));
    pinned
        || super::cert_issuer::cert_issuer(top)
            .and_then(super::roots::find_spki_by_subject)
            .is_some()
}
