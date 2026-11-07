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

//! What nsh prints when a session does not end well: a sentence for every
//! refusal and reason, the flag to change where one is to blame, and the
//! refusal's table name after it; and --handshake held to its bounds.

use nsh_client::connect::args::{parse, CLOSE_WAIT_SECS, HANDSHAKE_SECS, MAX_HANDSHAKE_SECS};
use nsh_client::connect::explain::{ended, refusal, Ended};
use nsh_client::{AttestRefusal as A, EnrolRefusal as E, Half, HostRefusal as H, Refusal as R};
use nsh_wire::{Reason, WireError};
use std::collections::HashSet;
use std::time::Duration;

const REASONS: [Reason; 21] = [
    Reason::Ok,
    Reason::Protocol,
    Reason::Version,
    Reason::Policy,
    Reason::Limit,
    Reason::Busy,
    Reason::Unattested,
    Reason::Unenrolled,
    Reason::GrantSig,
    Reason::GrantTime,
    Reason::GrantScope,
    Reason::GrantRevoked,
    Reason::RevocationStale,
    Reason::RoleUnknown,
    Reason::CapsCeiling,
    Reason::CapUnknown,
    Reason::UserSig,
    Reason::EscParent,
    Reason::EscApproval,
    Reason::EscTooLong,
    Reason::EscActive,
];

fn hosts() -> Vec<H> {
    let attest = [A::Parse, A::Version, A::Quote, A::ReleasePcrs, A::Dma, A::Registry];
    let step5 = [A::Log, A::Replay, A::BootApplication, A::Loader, A::PathOnly, A::NotVendor];
    let attest = attest.into_iter().chain(step5).chain([A::DeviceProof, A::Unavailable]);
    let attest = attest.map(H::Attest);
    let enrol = [E::Authority, E::Sig(Half::MlDsa65), E::Fleet, E::Ak, E::HostKeys];
    let enrol = enrol.into_iter().chain([E::HostName, E::Expired, E::Revoked]).map(H::Enrol);
    let rest = [H::HostSig(Half::Ed25519), H::Challenge, H::FirmwarePcrs];
    attest.chain(enrol).chain(rest).collect()
}

#[test]
fn every_host_refusal_has_its_own_sentence() {
    let said: Vec<String> = hosts().into_iter().map(|h| refusal(&R::Host(h))).collect();
    for s in &said {
        assert!(s.starts_with("refused the host before sending any credential: "), "{s}");
        assert!(!s.contains("Host(") && s.is_ascii(), "{s}");
    }
    assert_eq!(said.iter().collect::<HashSet<_>>().len(), said.len(), "{said:#?}");
}

#[test]
fn every_reason_has_its_own_sentence() {
    let said: HashSet<String> = REASONS.iter().map(|r| refusal(&R::AuthRefused(*r))).collect();
    assert_eq!(said.len(), 21, "{said:#?}");
    for s in &said {
        assert!(s.starts_with("the host refused the login: ") && s.len() > 36, "{s}");
    }
    let ended_by = refusal(&R::Aborted(Reason::Busy));
    assert_eq!(
        ended_by,
        "the host ended the session: it is serving all the sessions it takes; try again later"
    );
}

#[test]
fn a_host_that_serves_no_one_says_why_and_what_to_do() {
    let no_tpm = refusal(&R::Aborted(Reason::Unattested));
    assert!(no_tpm.starts_with("the host turned the connection away: it has no TPM"), "{no_tpm}");
    assert!(no_tpm.contains("vTPM"), "{no_tpm}");
    let unenrolled = refusal(&R::Aborted(Reason::Unenrolled));
    assert!(unenrolled.starts_with("the host turned the connection away: "), "{unenrolled}");
    assert!(unenrolled.contains("nsh-enrol") && unenrolled.contains("/nonos/nsh"), "{unenrolled}");
}

#[test]
fn a_refusal_names_the_flag_to_change() {
    let says = |h: H, flag: &str| {
        let s = refusal(&R::Host(h));
        assert!(s.contains(flag), "{s} names {flag}");
    };
    says(H::Attest(A::ReleasePcrs), "--releases");
    says(H::Attest(A::Registry), "--releases");
    says(H::Enrol(E::Authority), "--authority");
    says(H::Enrol(E::Fleet), "--fleet");
    says(H::Enrol(E::HostName), "--host");
    says(H::Enrol(E::Revoked), "--revoked-enrol");
    assert!(refusal(&R::GrantSubject).contains("--key"));
    assert!(refusal(&R::Sign).contains("--key"));
    assert!(refusal(&R::Unsendable(WireError::Value)).contains("--grant"));
    assert!(refusal(&R::Host(H::Attest(A::PathOnly))).contains("development image"));
    assert!(refusal(&R::Host(H::Attest(A::Loader))).contains("--releases"));
}

#[test]
fn the_printed_line_keeps_the_table_name() {
    let line = ended(&Ended::Refused(R::Host(H::Attest(A::ReleasePcrs))));
    assert!(line.ends_with(" [Host(Attest(ReleasePcrs))]"), "{line}");
    assert_eq!(ended(&Ended::Lost), "the connection closed before the session ended");
    assert_eq!((Ended::gone(true), Ended::gone(false)), (Ended::Lost, Ended::Closed));
    let closed = ended(&Ended::Closed);
    assert!(closed.starts_with("the host closed the connection before the login;"), "{closed}");
    assert!(closed.contains("route is Direct"), "{closed}");
    let slow = ended(&Ended::Slow(Duration::from_secs(7)));
    assert!(slow.starts_with("the host did not finish the handshake in 7 s;"), "{slow}");
    assert!(slow.contains("--handshake SECS"), "{slow}");
    let unended = ended(&Ended::Unended(Duration::from_secs(3)));
    assert!(unended.starts_with("the host did not end the session in 3 s after it was closed,"));
    assert!(unended.contains("--close-wait SECS"), "{unended}");
}

fn with(extra: &[&str]) -> Vec<String> {
    let base = ["--connect", "h:1", "--host", "h", "--fleet", &"f1".repeat(16)];
    let base = base.into_iter().chain(["--authority", "a", "--releases", "r", "--key", "k"]);
    base.chain(["--grant", "g"]).chain(extra.iter().copied()).map(String::from).collect()
}

#[test]
fn handshake_is_held_to_its_bounds() {
    assert_eq!(parse(&with(&[])).map(|a| a.handshake), Some(HANDSHAKE_SECS));
    assert_eq!(parse(&with(&["--handshake", "1"])).map(|a| a.handshake), Some(1));
    let most = MAX_HANDSHAKE_SECS.to_string();
    assert_eq!(
        parse(&with(&["--handshake", &most])).map(|a| a.handshake),
        Some(MAX_HANDSHAKE_SECS)
    );
    let over = (MAX_HANDSHAKE_SECS + 1).to_string();
    for bad in ["0", "-1", "x", "", &over, "99999999999999999999999"] {
        assert!(parse(&with(&["--handshake", bad])).is_none(), "--handshake {bad:?}");
    }
    assert!(parse(&with(&["--handshake", "5", "--handshake", "5"])).is_none(), "given twice");
}

#[test]
fn close_wait_is_held_to_its_bounds() {
    assert_eq!(parse(&with(&[])).map(|a| a.close_wait), Some(CLOSE_WAIT_SECS));
    assert_eq!(parse(&with(&["--close-wait", "1"])).map(|a| a.close_wait), Some(1));
    let most = MAX_HANDSHAKE_SECS.to_string();
    assert_eq!(
        parse(&with(&["--close-wait", &most])).map(|a| a.close_wait),
        Some(MAX_HANDSHAKE_SECS)
    );
    let over = (MAX_HANDSHAKE_SECS + 1).to_string();
    for bad in ["0", "-1", "x", "", &over, "99999999999999999999999"] {
        assert!(parse(&with(&["--close-wait", bad])).is_none(), "--close-wait {bad:?}");
    }
    assert!(parse(&with(&["--close-wait", "5", "--close-wait", "5"])).is_none(), "given twice");
    let both = parse(&with(&["--handshake", "7", "--close-wait", "9"]));
    assert_eq!(both.map(|a| (a.handshake, a.close_wait)), Some((7, 9)));
}
