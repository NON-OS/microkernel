// NONOS Operating System (AGPL-3.0-or-later)
//! What a `proxy` command in the address bar tells the reader: a mistyped
//! address is said to be one, and a proxy set while an anonymous network is
//! chosen is said to carry nothing until Direct is.

use super::net::mixnet::Network;
use super::said::{said, Outcome};

#[test]
fn a_mistyped_proxy_is_said_to_be_one() {
    let line = said(Outcome::Bad, Network::Direct);
    assert!(line.starts_with("Not a proxy address"), "{line}");
    assert!(line.contains("socks5://host:port"), "and how to write one: {line}");
}

#[test]
fn a_proxy_names_the_network_requests_really_take() {
    let direct = said(Outcome::Set("10.0.0.2", 1080), Network::Direct);
    assert_eq!(direct, "Direct requests now go through the SOCKS5 proxy 10.0.0.2:1080.");
    let nym = said(Outcome::Set("10.0.0.2", 1080), Network::Nym);
    assert!(nym.contains("kept for the direct network"), "{nym}");
    assert!(nym.contains(Network::Nym.label()), "the network in use is named: {nym}");
    assert_eq!(said(Outcome::Off, Network::Anyone), "SOCKS5 proxy off.");
}
