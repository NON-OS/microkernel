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

//! What the client says when a session does not end well: one sentence for
//! each refusal, naming what failed and, where one can, what to change, then
//! the refusal's own name in brackets, as docs/nsh/nsh-v1-refusals.md lists
//! it, so a report can be matched to the table.

use super::explain_host::host;
use super::explain_reason::reason;
use crate::Refusal;
use alloc::format;
use alloc::string::String;
use core::time::Duration;
use nsh_wire::Reason;

/// Why `session::run` stopped before the host's CLOSE.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Ended {
    /// The client or the host refused, for this reason.
    Refused(Refusal),
    /// The connection closed, or failed, after the login and before the
    /// session ended.
    Lost,
    /// The connection closed, or failed, before the host admitted the login.
    Closed,
    /// The host did not finish the handshake in this time.
    Slow(Duration),
    /// The host did not end the session in this time once it was closed.
    Unended(Duration),
}

impl Ended {
    /// The connection went, before the login or after it: the first is most
    /// often a host that serves nothing there, so it is told apart.
    pub fn gone(admitted: bool) -> Ended {
        if admitted {
            Ended::Lost
        } else {
            Ended::Closed
        }
    }
}

impl From<Refusal> for Ended {
    fn from(r: Refusal) -> Self {
        Ended::Refused(r)
    }
}

/// The line `nsh` prints for `e`.
pub fn ended(e: &Ended) -> String {
    match e {
        Ended::Refused(r) => format!("{} [{r:?}]", refusal(r)),
        Ended::Lost => "the connection closed before the session ended".into(),
        Ended::Closed => "the host closed the connection before the login; check that it \
             serves nsh/1 there: a NONOS guest's nsh.front listens only while its network \
             route is Direct, and a QEMU port forward takes the connection even when nothing \
             in the guest listens"
            .into(),
        Ended::Slow(t) => format!(
            "the host did not finish the handshake in {} s; check that it is up and serving \
             nsh/1 there, or allow a slow host more with --handshake SECS",
            t.as_secs()
        ),
        Ended::Unended(t) => format!(
            "the host did not end the session in {} s after it was closed, so it sent no \
             record; the shell may still be running a command there; allow a slow host more \
             with --close-wait SECS",
            t.as_secs()
        ),
    }
}

/// One sentence for `r`.
pub fn refusal(r: &Refusal) -> String {
    match r {
        Refusal::Wire(_) => "the host sent a frame that breaks nsh/1".into(),
        Refusal::Open => "a sealed frame from the host did not open: the connection was \
                          changed on the way, or it is not this session's"
            .into(),
        Refusal::Limit => "the session sent or received the most nsh/1 allows one direction; \
                           connect again"
            .into(),
        Refusal::Host(h) => format!("refused the host before sending any credential: {}", host(h)),
        Refusal::AuthRefused(why) => format!("the host refused the login: {}", reason(*why)),
        Refusal::Aborted(why @ (Reason::Unattested | Reason::Unenrolled)) => {
            format!("the host turned the connection away: {}", reason(*why))
        }
        Refusal::Aborted(why) => format!("the host ended the session: {}", reason(*why)),
        Refusal::Record(_) => {
            "the host's session record is not this session's; it was not kept".into()
        }
        Refusal::RecordLimit => "the host's session record passed 64 MiB; it was not kept".into(),
        Refusal::Sign => "the user key did not sign; check the file given with --key".into(),
        Refusal::Unsendable(_) => "a grant or the window size could not be sent as it is; \
                                   check the files given with --grant and --escalate"
            .into(),
        Refusal::GrantSubject => {
            "the grant is for another user key than the one given with --key".into()
        }
        Refusal::NotNow => "the client asked for a step the session does not take now".into(),
        Refusal::Ended => "the session is over".into(),
    }
}
