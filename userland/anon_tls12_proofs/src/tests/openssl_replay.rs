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


//! Every recorded OpenSSL handshake, replayed: the client writes exactly the
//! bytes OpenSSL accepted when it was recorded, however the server's bytes
//! are split, and ends exactly as it did then.

use super::cases::{all, finished_end, get, handshake, outcome_line, records, server_hello, signature_scheme};
use super::replay::{run, Replay};

fn replay(case: &super::cases::Case, chunk: usize) -> (String, Replay) {
    let mut r = Replay::new(case.server, case.client, chunk);
    crate::trace::take();
    let start = nonos_libc::mk_uptime_ms();
    let outcome = run(&mut r, || nonos_libc::mk_uptime_ms() - start > 20_000);
    (outcome.line(), r)
}

use std::string::String;

#[test]
fn every_recording_replays_byte_for_byte_at_any_split() {
    let cases = all();
    assert_eq!(cases.len(), 9);
    for case in &cases {
        for chunk in [1, 7, 512, usize::MAX] {
            let (line, r) = replay(case, chunk);
            assert_eq!(line, outcome_line(case), "{} at chunk {chunk}", case.name);
            assert!(r.wrote_all(), "{} at chunk {chunk}: the client wrote less than when recorded", case.name);
        }
    }
}

#[test]
fn the_sessions_carry_data_both_ways() {
    for name in ["chacha_certreq", "aes_gcm_renegotiate", "pkcs1_sha384", "pkcs1_sha256", "fragmented", "no_ems"] {
        assert!(outcome_line(&get(name)).starts_with("session hello from openssl\\n"), "{name}");
        // The client's last record is its ping, protected: application data.
        let client = records(get(name).client);
        assert_eq!(client.last().unwrap().0, 23, "{name}");
    }
}

#[test]
fn both_suites_were_negotiated() {
    assert_eq!(server_hello(get("chacha_certreq").server).0, 0xCCA8);
    assert_eq!(server_hello(get("aes_gcm_renegotiate").server).0, 0xC030);
    assert_eq!(server_hello(get("fragmented").server).0, 0xC030);
}

#[test]
fn pss_and_pkcs1_over_both_hashes_were_verified() {
    assert_eq!(signature_scheme(get("chacha_certreq").server), 0x0804, "RSA-PSS SHA-256 by default");
    assert_eq!(signature_scheme(get("pkcs1_sha384").server), 0x0501);
    assert_eq!(signature_scheme(get("pkcs1_sha256").server), 0x0401);
}

#[test]
fn a_certificate_request_is_answered_with_an_empty_certificate() {
    let case = get("chacha_certreq");
    assert!(handshake(case.server).iter().any(|(k, _)| *k == 13), "OpenSSL asked, as a relay does");
    let sent = handshake(case.client);
    assert_eq!(sent[1], (11, std::vec![0, 0, 0]), "an empty certificate_list, before the key exchange");
    assert_eq!(sent[2].0, 16);
    // Without a request there is no Certificate at all.
    let plain = handshake(get("aes_gcm_renegotiate").client);
    assert!(!plain.iter().any(|(k, _)| *k == 11));
}

#[test]
fn the_extended_master_secret_is_used_when_offered_back_and_not_required() {
    assert!(server_hello(get("chacha_certreq").server).2.contains(&23));
    assert!(!server_hello(get("no_ems").server).2.contains(&23), "this server left it out");
    assert!(outcome_line(&get("no_ems")).starts_with("session"), "and the session still works");
}

#[test]
fn handshake_messages_split_across_records_are_reassembled() {
    let case = get("fragmented");
    let recs = records(case.server);
    assert!(recs.iter().all(|(_, body, _)| body.len() <= 512 + 40));
    assert!(recs.iter().filter(|(k, _, _)| *k == 22).count() >= 3, "the certificate spans records");
}

#[test]
fn a_tls13_server_negotiating_tls12_is_refused_by_its_mark() {
    let case = get("downgrade");
    assert_eq!(&server_hello(case.server).1[24..], b"DOWNGRD\x01", "OpenSSL marked its random");
    assert_eq!(outcome_line(&case), "refused Downgrade");
    assert_eq!(case.client.len(), records(case.client)[0].1.len() + 5, "nothing after the hello");
}

#[test]
fn no_shared_suite_ends_in_the_servers_alert() {
    assert_eq!(outcome_line(&get("no_shared_suite")), "refused PeerAlert(40)");
}

#[test]
fn renegotiation_ends_the_session_and_says_so() {
    let (line, _) = replay(&get("aes_gcm_renegotiate"), usize::MAX);
    assert!(line.ends_with("done=true"), "{line}");
    assert!(crate::trace::take().iter().any(|l| l == "tls12 relay asked to renegotiate, link ended"));
}

#[test]
fn close_notify_ends_the_session_cleanly() {
    let (line, _) = replay(&get("close_notify"), usize::MAX);
    assert_eq!(line, "session hello from python ssl\\n done=true");
    assert!(crate::trace::take().iter().any(|l| l == "tls12 relay closed the session"));
}

#[test]
fn every_single_byte_change_to_the_servers_handshake_is_refused() {
    // Each byte of the server's flight through its Finished, flipped in
    // turn. Every one must end in a refusal: the signature covers the
    // randoms and the key share, the Finished covers the rest, and the
    // record layer covers the framing. None may panic.
    for name in ["chacha_certreq", "aes_gcm_renegotiate"] {
        let case = get(name);
        let end = finished_end(case.server);
        for at in 0..end {
            let mut bytes = case.server.to_vec();
            bytes[at] ^= 0x01;
            let mut r = Replay::lenient(&bytes);
            let start = nonos_libc::mk_uptime_ms();
            let outcome = run(&mut r, || nonos_libc::mk_uptime_ms() - start > 20_000);
            assert!(outcome.line().starts_with("refused"), "{name}: flip at {at} of {end} gave {}", outcome.line());
        }
    }
}
