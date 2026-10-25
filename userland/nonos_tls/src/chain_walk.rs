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

// Each link costs a signature check in the pool and the message can list 255
// certificates. Served chains are a leaf and one to three issuers; ten leaves
// room for cross-signed paths and refuses a list that only asks for work.
const MAX_CHAIN: u8 = 10;

// Validate the server's TLS Certificate(11) message body for `host` at time
// `now`: the leaf must match the hostname and be in its validity window and each
// certificate must be signed by the next one up. The top served certificate is
// then anchored to a trusted root. Servers commonly omit the root, so if the top
// certificate's own SPKI is not itself pinned we look up a root by the top
// certificate's issuer and verify the top certificate's signature against that
// root's public key. Fail closed everywhere.
pub fn verify_chain(body: &[u8], host: &[u8], now: u64) -> super::crypto_port::Checked {
    let n = super::cert_count::cert_count(body);
    if !(1..=MAX_CHAIN).contains(&n) {
        return Ok(false);
    }
    let Some(leaf) = super::cert_at::cert_at(body, 0) else {
        return Ok(false);
    };
    if !super::cert_dns_match::matches(leaf, host)
        || !super::cert_valid_now::cert_valid_now(leaf, now)
    {
        return Ok(false);
    }
    let mut i = 0u8;
    while i + 1 < n {
        let Some(child) = super::cert_at::cert_at(body, i) else {
            return Ok(false);
        };
        let Some(parent) = super::cert_at::cert_at(body, i + 1) else {
            return Ok(false);
        };
        let Some(parent_spki) = super::cert_spki::cert_spki(parent) else {
            return Ok(false);
        };
        // Every issuer must be a CA. Signature checks alone accept a chain in
        // which an attacker's own leaf certificate signs a forgery.
        if !super::verify_link::verify_link(child, parent_spki)?
            || !super::cert_valid_now::cert_valid_now(parent, now)
            || !super::cert_is_ca::cert_is_ca(parent)
        {
            return Ok(false);
        }
        i += 1;
    }
    let Some(top) = super::cert_at::cert_at(body, n - 1) else {
        return Ok(false);
    };
    super::chain_anchor::anchor(top, leaf, host, n)
}
