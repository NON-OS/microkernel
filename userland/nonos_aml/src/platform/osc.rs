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

//! _OSC (ACPI 6.5, 6.2.11): the operating system says what it supports and
//! asks for control; firmware answers with what it grants. Asked first with
//! the query flag, then committed with no more than was granted.

use super::eval::Eval;
use super::osc_status::{check, OscError};
pub use super::osc_uuid::{PCI_HOST_UUID, PLATFORM_UUID};
use crate::load::Aml;
use crate::namespace::Id;
use crate::object::Object;
use alloc::vec::Vec;

pub(super) fn call(
    aml: &mut Aml,
    eval: &mut Eval<'_>,
    osc: Id,
    (uuid, rev): (&[u8; 16], u64),
    words: &[u32],
) -> Result<Vec<u32>, OscError> {
    let bytes: Vec<u8> = words.iter().flat_map(|w| w.to_le_bytes()).collect();
    let args = alloc::vec![
        Object::buffer(uuid.to_vec()),
        Object::Integer(rev),
        Object::Integer(words.len() as u64),
        Object::buffer(bytes),
    ];
    let Object::Buffer(b) = eval(aml, osc, args).map_err(OscError::Fault)? else {
        return Err(OscError::Shape);
    };
    let b = b.borrow();
    let out: Vec<u32> =
        b.chunks_exact(4).map(|c| u32::from_le_bytes([c[0], c[1], c[2], c[3]])).collect();
    if out.len() < words.len() {
        return Err(OscError::Shape);
    }
    check(out[0])?;
    Ok(out)
}

/// Query, then commit what was granted. `words[0]` is the status word and
/// is set here; the rest are the UUID's capability words. Returns them as
/// firmware granted them.
pub fn negotiate(
    aml: &mut Aml,
    eval: &mut Eval<'_>,
    osc: Id,
    uuid: (&[u8; 16], u64),
    words: &[u32],
) -> Result<Vec<u32>, OscError> {
    let all = alloc::vec![u32::MAX; words.len()];
    super::osc_claim::offer_then_claim(aml, eval, osc, uuid, words, &all).map(|(_, g)| g)
}
