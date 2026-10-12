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

use super::commerce::Commerce;
use super::lp::{lp, skip};

fn word(body: &[u8], at: usize) -> Option<[u8; 32]> {
    body.get(at..at.checked_add(32)?)?.try_into().ok()
}

pub fn parse_commerce(body: &[u8]) -> Option<Commerce> {
    let verdicts = body.get(..2)?;
    let listing_id = word(body, 2)?;
    let release_id = word(body, 34)?;
    let release_hash = word(body, 66)?;
    let at = skip(body, 98, 8)?;
    let chain_id = u64::from_le_bytes(body.get(98..at)?.try_into().ok()?);
    let (token, at) = lp(body, at)?;
    let (block, end) = lp(body, at)?;
    if end != body.len() {
        return None;
    }
    Some(Commerce {
        commerce_verdict: *verdicts.first()?,
        binding_verdict: *verdicts.get(1)?,
        listing_id,
        release_id,
        release_hash,
        chain_id,
        token_contract: token.to_vec(),
        block: block.to_vec(),
    })
}
