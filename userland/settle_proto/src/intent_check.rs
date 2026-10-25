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

//! INTENT's checks: the calldata's selector belongs to the kind, and an
//! approve's spender word is an address.

use crate::bytes::fixed;
use crate::kinds::{Kind, KINDS};

/// The kind numbered `id`, if `calldata` is one of its calls: a known
/// selector, then whole words.
pub fn kind_of(id: u8, calldata: &[u8]) -> Option<&'static Kind> {
    let kind = KINDS.iter().find(|k| k.id == id)?;
    let sel = u32::from_be_bytes(calldata.get(..4)?.try_into().ok()?);
    let whole = calldata.len().checked_sub(4)?.is_multiple_of(32);
    (whole && kind.calls.iter().any(|(_, s)| *s == sel)).then_some(kind)
}

/// A ShieldClaim (kind 20) settle quotes: the asset is ETH (0) or NOX (1),
/// and `to` is the sender, so a claim never pays anyone else.
pub fn claim_ok(calldata: &[u8], from: &[u8; 20]) -> bool {
    let (Some(asset), Some(to)) = (calldata.get(4..36), calldata.get(36..68)) else { return false };
    let zero = |w: Option<&[u8]>| w.is_some_and(|w| w.iter().all(|b| *b == 0));
    let to_ok = zero(to.get(..12)) && to.get(12..) == Some(&from[..]);
    zero(asset.get(..31)) && asset.get(31).is_some_and(|b| *b <= 1) && to_ok && calldata.len() == 68
}

/// An approve's spender, when its word is an address.
pub fn approve_spender(calldata: &[u8]) -> Option<[u8; 20]> {
    let w = calldata.get(4..36)?;
    w.get(..12)?.iter().all(|b| *b == 0).then(|| fixed(w, 12))?
}
