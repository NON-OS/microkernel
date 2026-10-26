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

//! The only place the throwaway root (P5) anchors a chain, in a `devnet`
//! build. Its key is public, so the root alone proves nothing about who
//! signed: what keeps it from vouching for a real chain's RPC, a package
//! mirror or a gateway is that it is taken for the local RPC's own leaf only.

use super::dev::{DEV_ROOT_SPKI, DEV_ROOT_SPKI_SHA256, DEV_ROOT_SUBJECT, DEV_RPC_HOST};
use crate::crypto_port::Checked;

/// Whether the served chain of `len` certificates, from `leaf` up to `top`,
/// ends at the throwaway root for `host`. Each condition refuses alone: the
/// name is the local RPC's, the leaf is for a TLS server and is no CA, and
/// the root signed the leaf itself, served alone or with the root over it.
pub fn anchors(top: &[u8], leaf: &[u8], host: &[u8], len: u8) -> Checked {
    if host != DEV_RPC_HOST
        || !super::dev_eku::serves_tls(leaf)
        || crate::cert_is_ca::cert_is_ca(leaf)
        || !direct(top, leaf, len)
    {
        return Ok(false);
    }
    crate::verify_link::verify_link(leaf, DEV_ROOT_SPKI)
}

// The root's path length is zero, so nothing may stand between it and the
// leaf: the chain is the leaf alone, or the leaf and the root itself.
fn direct(top: &[u8], leaf: &[u8], len: u8) -> bool {
    let named = crate::cert_issuer::cert_issuer(leaf) == Some(DEV_ROOT_SUBJECT);
    match len {
        1 => named,
        2 => named && is_dev_root(top),
        _ => false,
    }
}

fn is_dev_root(cert: &[u8]) -> bool {
    crate::cert_spki::cert_spki(cert)
        .and_then(crate::hash_sha256::hash_sha256)
        .is_some_and(|h| h == DEV_ROOT_SPKI_SHA256)
}
