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

//! A checkpoint, `nsh-ckpt/1`: the session, the last entry it covers, the
//! chain value after that entry, and the host keys' hybrid signature over
//! `H("nsh/1 record-ckpt\0" || SHA-512(the fields before it))`.

use super::CkptRefusal;
use crate::ct::eq;
use crate::hybrid;
use nonos_hash::sha512;
use nsh_crypto::{labelled, Label};
use nsh_wire::keys::{before_sig, HYBRID_SIG_LEN};
use nsh_wire::{HybridSig, Reader, RecordHeader, WireError};

pub const CKPT_LEN: usize = 8 + 16 + 4 + 64 + HYBRID_SIG_LEN;

/// A checkpoint that verified: entry `seq` must leave the chain at `chain`.
#[derive(Debug, Clone, Copy)]
pub(crate) struct Mark {
    pub seq: u32,
    pub chain: [u8; 64],
}

pub(crate) fn check(bytes: &[u8], h: &RecordHeader<'_>) -> Result<Mark, CkptRefusal> {
    let mut r = Reader::new(bytes);
    let parsed = read(&mut r).and_then(|m| r.finish().map(|_| m));
    let (session, mark, sig) = parsed.map_err(CkptRefusal::Parse)?;
    if !eq(session, h.session_id) {
        return Err(CkptRefusal::Session);
    }
    let signed = before_sig(bytes).map_err(CkptRefusal::Parse)?;
    let digest = labelled(Label::RecordCkpt, &sha512(signed));
    hybrid::verify(h.host.ed, h.host.ml, &digest, &sig).map_err(CkptRefusal::Sig)?;
    Ok(mark)
}

type Read<'a> = (&'a [u8; 16], Mark, HybridSig<'a>);

fn read<'a>(r: &mut Reader<'a>) -> Result<Read<'a>, WireError> {
    r.preamble(b"NSHC", 1)?;
    let session = r.array::<16>()?;
    let seq = r.u32()?;
    let chain = *r.array::<64>()?;
    Ok((session, Mark { seq, chain }, HybridSig::read(r)?))
}
