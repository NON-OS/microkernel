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

//! The hostname check the wallet's RPC connections run, nonos_tls's own,
//! held to forged certificates for the wallet's RPC host: a name in the key
//! or inside a long entry is not a name the certificate claims.

use super::der_build::{certificate, general_names, key_holding, tlv};
use crate::cert_dns_match::matches;

const DNS: u8 = 0x82;
const URI: u8 = 0x86;
const RPC: &[u8] = b"ethereum-rpc.publicnode.com";

#[test]
fn the_rpc_host_hidden_in_the_key_is_not_claimed() {
    let fake = tlv(0x04, &general_names(&[(DNS, RPC)]));
    let mut planted = vec![0x06, 0x03, 0x55, 0x1d, 0x11];
    planted.extend_from_slice(&fake);
    let cert = certificate(&key_holding(&planted), &general_names(&[(DNS, b"attacker.example")]));
    assert!(!matches(&cert, RPC), "the wallet believed a name in the key");
    assert!(matches(&cert, b"attacker.example"));
}

#[test]
fn a_bare_dns_shape_anywhere_in_the_certificate_is_not_a_name() {
    let mut shaped = vec![DNS, RPC.len() as u8];
    shaped.extend_from_slice(RPC);
    let cert = certificate(&key_holding(&shaped), &general_names(&[(DNS, b"attacker.example")]));
    assert!(!matches(&cert, RPC), "the old scan matched these bytes in the key");
}

#[test]
fn the_rpc_host_inside_a_long_entry_is_not_a_dns_name() {
    let mut uri = vec![DNS, RPC.len() as u8];
    uri.extend_from_slice(RPC);
    uri.resize(200, b'a');
    let cert =
        certificate(&key_holding(b"k"), &general_names(&[(URI, &uri), (DNS, b"attacker.example")]));
    assert!(!matches(&cert, RPC));
}

#[test]
fn a_certificate_for_the_rpc_host_still_matches() {
    let cert = certificate(&key_holding(b"k"), &general_names(&[(DNS, RPC)]));
    assert!(matches(&cert, RPC));
    let wild = certificate(&key_holding(b"k"), &general_names(&[(DNS, b"*.publicnode.com")]));
    assert!(matches(&wild, RPC));
}
