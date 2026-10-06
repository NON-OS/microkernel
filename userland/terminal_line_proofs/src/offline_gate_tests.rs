// NONOS Operating System (AGPL-3.0-or-later)
//! `ping`, `nslookup`, `pull` and `push` on a machine with no address (no
//! cable, no Wi-Fi joined): one line at once saying what is missing and what
//! to do, instead of a three-second wait and "servfail". A DHCP client that
//! does not answer is not taken as offline.

use crate::offline_gate::{offline_line, LeaseSeen};

fn said(command: &[u8], seen: LeaseSeen) -> String {
    String::from_utf8(offline_line(command, seen).expect("a line")).unwrap()
}

#[test]
fn a_machine_with_an_address_goes_ahead() {
    assert_eq!(offline_line(b"ping", LeaseSeen::Bound), None);
}

#[test]
fn a_dhcp_client_that_does_not_answer_is_not_taken_as_offline() {
    assert_eq!(offline_line(b"ping", LeaseSeen::Silent), None);
}

#[test]
fn no_address_says_what_is_missing_and_what_to_do() {
    let line = said(b"ping", LeaseSeen::Unbound);
    assert!(line.starts_with("ping: not connected to a network"), "{line}");
    assert!(line.contains("Plug in a cable or join a Wi-Fi network in Settings"), "{line}");
    assert!(line.ends_with("nothing was sent"), "{line}");
}

#[test]
fn a_boot_with_no_network_says_so() {
    let line = said(b"nslookup", LeaseSeen::NoClient);
    assert_eq!(line, "nslookup: no network is running on this boot, so nothing was sent");
}

#[test]
fn every_command_is_named_and_no_line_shows_a_code() {
    for cmd in [&b"ping"[..], b"nslookup", b"pull", b"push"] {
        for seen in [LeaseSeen::Unbound, LeaseSeen::NoClient] {
            let line = said(cmd, seen);
            assert!(line.as_bytes().starts_with(cmd));
            assert!(!line.contains("servfail") && !line.contains("errno"), "{line}");
            assert!(!line.chars().any(|c| c.is_ascii_digit()), "{line}");
        }
    }
}
