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

//! Signature subpackets: who made it, and whether anything critical is
//! present that this verifier cannot honour.

use super::refusal::Refusal;

const CREATED: u8 = 2;
const EXPIRES: u8 = 3;
const ISSUER: u8 = 16;
const SIGNER_UID: u8 = 28;
const ISSUER_FPR: u8 = 33;

#[derive(Default)]
pub struct Found {
    pub fingerprint: Option<[u8; 20]>,
    pub key_id: Option<[u8; 8]>,
}

/// Walk one area. In the hashed area a critical subpacket of a type not
/// known here voids the signature, as RFC 9580 5.2.3.7 requires.
pub fn read(mut area: &[u8], hashed: bool, found: &mut Found) -> Result<(), Refusal> {
    while !area.is_empty() {
        let (len, head) = match usize::from(area[0]) {
            o @ 0..=191 => (o, 1),
            o @ 192..=254 => {
                (((o - 192) << 8) + usize::from(*area.get(1).ok_or(Refusal::Malformed)?) + 192, 2)
            }
            _ => (be(area.get(1..5).ok_or(Refusal::Malformed)?), 5),
        };
        let body = area.get(head..head + len).ok_or(Refusal::Malformed)?;
        let (&kind, value) = body.split_first().ok_or(Refusal::Malformed)?;
        match (kind & 0x7F, value) {
            (ISSUER_FPR, [4, fpr @ ..]) => found.fingerprint = fpr.try_into().ok(),
            (ISSUER, id) => found.key_id = found.key_id.or(id.try_into().ok()),
            (CREATED | EXPIRES | SIGNER_UID | ISSUER_FPR, _) => {}
            _ if hashed && kind & 0x80 != 0 => return Err(Refusal::Critical),
            _ => {}
        }
        area = &area[head + len..];
    }
    Ok(())
}

fn be(d: &[u8]) -> usize {
    d.iter().fold(0, |v, &b| v << 8 | usize::from(b))
}
