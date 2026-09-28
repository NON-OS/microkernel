// NONOS Operating System (AGPL-3.0-or-later)
//! The link handshake, cell by cell. NETINFO is a fixed-length cell, and the
//! handshake used to wait for a variable-length one, so every link failed at
//! the moment the relay said NETINFO, just after its identity was proved.

use crate::cell::{CELL_AUTH_CHALLENGE, CELL_CERTS, CELL_NETINFO, CELL_VPADDING};
use crate::link_step::{classify, Next};

#[test]
fn a_fixed_netinfo_after_proved_certs_finishes_the_handshake() {
    assert_eq!(classify(false, CELL_NETINFO, true), Next::Finish);
}

#[test]
fn netinfo_before_certs_is_refused() {
    assert_eq!(classify(false, CELL_NETINFO, false), Next::Refuse);
}

#[test]
fn certs_is_variable_length_only() {
    assert_eq!(classify(true, CELL_CERTS, false), Next::Certs);
    assert_eq!(classify(false, CELL_CERTS, false), Next::Refuse);
}

#[test]
fn padding_and_an_auth_challenge_are_passed_over() {
    assert_eq!(classify(true, CELL_AUTH_CHALLENGE, true), Next::Ignore);
    assert_eq!(classify(true, CELL_VPADDING, false), Next::Ignore);
    assert_eq!(classify(false, 0, true), Next::Ignore);
}

#[test]
fn a_variable_netinfo_is_not_a_netinfo() {
    assert_eq!(classify(true, CELL_NETINFO, true), Next::Refuse);
}

#[test]
fn circuit_cells_during_the_handshake_are_refused() {
    for command in [3u8, 4, 9, 10, 11] {
        assert_eq!(classify(false, command, true), Next::Refuse, "command {command}");
    }
}
