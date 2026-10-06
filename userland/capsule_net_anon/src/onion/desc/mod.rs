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


//! A v3 onion service descriptor: the outer layer and its signature, the
//! two encrypted layers inside it, and the introduction points at the core.

extern crate alloc;

use alloc::vec::Vec;

use crate::directory::consensus::object_after;
use crate::directory::lines::lines;

mod error;
pub mod intro;
mod layer;
mod outer;

pub use error::DescError;
pub use intro::IntroPoint;

/// HS_DESC_MAX_LEN: an HSDir stores nothing longer, so a client reads
/// nothing longer either.
pub const DESC_MAX: usize = 50_000;

/// What a client needs from a descriptor.
pub struct Descriptor {
    /// The revision counter; a cache keeps the highest it has seen.
    pub revision: u64,
    /// How long it may be used from when it was fetched, in seconds.
    pub lifetime: u64,
    pub intro_points: Vec<IntroPoint>,
    /// The puzzle the service asks for while it is under load, if any.
    pub pow: Option<super::pow::PowParams>,
}

/// Decode and check `body` for the service whose blinded key and
/// subcredential for this period are `blinded` and `subcredential`, at
/// `now`. `seed` gives X25519(this client's secret for the service, the
/// layer's ephemeral key) when the client holds one, `None` otherwise.
pub fn decode(
    body: &[u8],
    blinded: &[u8; 32],
    subcredential: &[u8; 32],
    now: u64,
    seed: impl FnOnce(&[u8; 32]) -> Option<[u8; 32]>,
) -> Result<Descriptor, DescError> {
    if body.len() > DESC_MAX {
        return Err(DescError::Malformed);
    }
    let outer = outer::outer(body, blinded, now)?;
    let middle = layer::open(&outer.superencrypted, blinded, subcredential, outer.revision, layer::SUPERENCRYPTED)
        .ok_or(DescError::Layer)?;
    let encrypted = lines(&middle)
        .find(|l| l.keyword == b"encrypted")
        .and_then(|l| object_after(&middle, l.at))
        .ok_or(DescError::Malformed)?;
    /*
     * With no client authorization the inner layer's secret is the blinded
     * key alone. A service that restricts its clients keys it with a cookie
     * as well, which only a client holding a key it listed can recover; for
     * any other client the inner layer's MAC fails here.
     */
    let mut secret = blinded.to_vec();
    if let Some(auth) = super::client_auth::auth_layer(&middle) {
        if let Some(mut s) = seed(&auth.ephemeral) {
            if let Some(cookie) = super::client_auth::cookie(subcredential, &s, &auth) {
                secret.extend_from_slice(&cookie);
            }
            super::client_auth::wipe(&mut s);
        }
    }
    let inner = layer::open(&encrypted, &secret, subcredential, outer.revision, layer::ENCRYPTED);
    super::client_auth::wipe(&mut secret);
    let inner = inner.ok_or(DescError::ClientAuth)?;
    if !lines(&inner).any(|l| l.keyword == b"create2-formats") {
        return Err(DescError::Malformed);
    }
    let pow = super::pow::params(&inner).map_err(|_| DescError::Malformed)?;
    let intro_points = intro::intro_points(&inner, &outer.signing_key, now);
    if intro_points.is_empty() {
        return Err(DescError::NoIntroPoints);
    }
    Ok(Descriptor { revision: outer.revision, lifetime: outer.lifetime, intro_points, pow })
}
