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

/*
 * Believing a catalogue: signed by the marketplace operator, the key the
 * market's index is signed with, and naming only files the pins name, with
 * the pins' tier, length and SHA-256, and a mirror to fetch each from.
 */

use nonos_ed25519::{verify, Signature};

use super::admit::admit;
use super::parse::parse;
use super::types::Catalogue;

/* The marketplace operator, v1, read from the key file as the market reads it. */
const OPERATOR: [u8; 32] = *include_bytes!("../../../../.keys/marketplace_operator_ed25519.pub");

pub fn check(blob: &[u8]) -> Result<Catalogue, &'static str> {
    let at = blob.len().checked_sub(64).ok_or("the model catalogue is cut short")?;
    let (body, sig) = blob.split_at(at);
    let sig: [u8; 64] = sig.try_into().map_err(|_| "the model catalogue is cut short")?;
    if !verify(&OPERATOR, body, &Signature::from_bytes(&sig)) {
        return Err("the model catalogue is not signed by the NONOS marketplace operator");
    }
    let cat = parse(body).ok_or("the model catalogue is malformed")?;
    admit(&cat)?;
    Ok(cat)
}
