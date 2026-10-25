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

//! The operator's tool, `tools/nsh-enrol`: a certificate it makes from a
//! machine's document passes the client's whole check, and every document
//! or value the client would refuse is refused before anything is signed.

use nsh_attest::Doc;
use nsh_enrol::{AuthorityKey, Request};
use std::sync::OnceLock;

use crate::build::ak::Key;
use crate::build::keys::{hybrid, HOST};
use crate::build::tcg_event::digest;
use crate::build::tcg_log::loader;
use crate::build::Built;

mod authority;
mod body;
mod expect;
mod inspect;
mod refusals;
mod sign;
mod whole;

pub use whole::check;

/// The boot applications the operator expects on these machines.
pub fn expected() -> &'static [[u8; 32]] {
    static APPS: OnceLock<Vec<[u8; 32]>> = OnceLock::new();
    APPS.get_or_init(|| [&b"boot manager"[..], b"first stage", b"vendor tool"].map(digest).to_vec())
}

pub fn key() -> &'static AuthorityKey {
    static KEY: OnceLock<AuthorityKey> = OnceLock::new();
    KEY.get_or_init(|| AuthorityKey::generate().expect("a key"))
}

/// Host keys and the one release loader.
pub fn inputs() -> ([u8; 1984], [[u8; 32]; 1]) {
    let h = hybrid(HOST);
    let keys = [&h.ed.public[..], &h.ml_public[..]].concat();
    (keys.try_into().expect("1984 bytes"), [loader()])
}

/// The request for a built host, as an operator makes it.
pub fn request<'a>(b: &'a Built, keys: &'a [u8; 1984], loaders: &'a [[u8; 32]]) -> Request<'a> {
    Request {
        doc: &b.doc,
        challenge: Doc::parse(&b.doc).map(|d| *d.challenge).unwrap_or([0; 32]),
        ak: Key::new(1).public(),
        host_keys: keys,
        host: b"db-1.eu.example",
        fleet: [0xF1; 16],
        loaders,
        boot_apps: expected(),
        not_after_ms: b.now_ms + 1_000,
        now_ms: b.now_ms,
        authority_id: key().id(),
    }
}
