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

//! The message a failed fetch is shown with.

use alloc::string::String;

use crate::browser::fetch::types::{Fetch, TlsWhy};
use crate::browser::tls13;
use crate::browser::tls13::CertProblem;

/* The reason arrives as a byte on the wire; the sentence is a rendering of
 * it. Any other code is said by `words`, on the way the request took. */
pub fn reason(job: &Fetch) -> String {
    if let Some(why) = job.tls_why {
        return sentence(why, &job.url.host);
    }
    let code = job.error.unwrap_or("the page could not be fetched");
    let said = crate::browser::fetch::words::words(code, job.way, &job.url.host);
    match job.tls_alert {
        Some(description) => {
            alloc::format!("{said} The server's alert was {}.", tls13::alert_name(description))
        }
        None => said,
    }
}

/*
 * Each sentence says what was found and then what it means for the reader,
 * so a status line cut short still carries the finding. The certificate ones
 * all begin "certificate", which also keeps them out of the retry rule. A
 * clock that is wrong makes a good certificate look expired or early, so
 * those two show the clock the check was run at.
 */
/// The page's wording for a refused handshake reaching `host`.
pub fn sentence(why: TlsWhy, host: &str) -> String {
    match why {
        TlsWhy::Tls12Only => String::from(
            "This site only speaks TLS 1.2, which this browser does not support yet.",
        ),
        TlsWhy::Cert(Some(CertProblem::UnknownIssuer), _) => String::from(
            "certificate not trusted: it was issued by an authority this browser does not trust",
        ),
        TlsWhy::Cert(Some(CertProblem::NameMismatch), _) => alloc::format!(
            "certificate name mismatch: the certificate is for a different name, not {}",
            host
        ),
        TlsWhy::Cert(Some(CertProblem::Expired), now) => alloc::format!(
            "certificate expired: the site's certificate has expired, or this machine's clock is wrong ({})",
            clock(now)
        ),
        TlsWhy::Cert(Some(CertProblem::NotYetValid), now) => alloc::format!(
            "certificate not yet valid: this machine's clock may be wrong ({})",
            clock(now)
        ),
        TlsWhy::Cert(Some(CertProblem::Unreadable), _) => String::from(
            "certificate unreadable: the site sent a certificate that could not be read",
        ),
        TlsWhy::Cert(None, _) => String::from(
            "certificate not verified: the site's certificate or handshake signature did not check out",
        ),
    }
}

/// `YYYYMMDDhhmmss` as a person reads it, or that it could not be read.
pub fn clock(now: u64) -> String {
    if now == 0 {
        return String::from("the clock could not be read");
    }
    let part = |div: u64| (now / div) % 100;
    alloc::format!(
        "it reads {:04}-{:02}-{:02} {:02}:{:02} UTC",
        now / 10_000_000_000,
        part(100_000_000),
        part(1_000_000),
        part(10_000),
        part(100)
    )
}
