// NONOS Operating System (AGPL-3.0-or-later)
//! `ping`, `nslookup` and `pull`, which can only leave directly: each runs
//! only when Direct is the default network, and otherwise prints one line
//! saying why nothing was sent. The rule is nonos_route_link's, so an
//! unreadable or unknown default refuses, as the mixnet it stands for.

use nonos_policy_proto::route::{ANYONE, DIRECT, NYM};

use crate::direct_gate::{refusal_line, NSLOOKUP, PING, PULL, PUSH};
use crate::direct_only::direct_refused;

fn every_default() -> impl Iterator<Item = Option<u8>> {
    core::iter::once(None).chain((0..=u8::MAX).map(Some))
}

#[test]
fn only_direct_lets_them_run() {
    for default in every_default() {
        for (command, what) in [(&b"ping"[..], PING), (b"nslookup", NSLOOKUP), (b"pull", PULL), (b"push", PUSH)] {
            let line = refusal_line(command, what, direct_refused(default));
            assert_eq!(line.is_none(), default == Some(DIRECT), "{default:?}");
        }
    }
}

#[test]
fn a_refusal_names_the_command_the_network_and_what_was_not_sent() {
    let line = refusal_line(b"ping", PING, direct_refused(Some(NYM))).unwrap_or_default();
    assert_eq!(
        line,
        b"ping: the chosen network is the Nym mixnet, which is anonymous; \
          ICMP cannot cross it and would leave directly, so nothing was sent"
    );
    let line = refusal_line(b"nslookup", NSLOOKUP, direct_refused(Some(ANYONE)));
    let line = line.unwrap_or_default();
    assert!(line.starts_with(b"nslookup: the chosen network is the Anyone onion network"));
    assert!(line.ends_with(b"a lookup would leave in the clear, so nothing was sent"));
    let line = refusal_line(b"pull", PULL, direct_refused(None)).unwrap_or_default();
    assert!(line.starts_with(b"pull: the default network could not be read"));
    assert!(line.windows(4).any(|w| w == b"curl"), "points at what goes the chosen way");
    let line = refusal_line(b"push", PUSH, direct_refused(Some(NYM))).unwrap_or_default();
    assert!(line.starts_with(b"push: the chosen network is the Nym mixnet"));
    assert!(line.ends_with(b", so nothing was sent"));
}

#[test]
fn every_refusal_is_one_line() {
    for default in every_default() {
        for what in [PING, NSLOOKUP, PULL, PUSH] {
            if let Some(line) = refusal_line(b"cmd", what, direct_refused(default)) {
                assert!(!line.contains(&b'\n'));
                assert!(line.ends_with(b", so nothing was sent"));
            }
        }
    }
}
