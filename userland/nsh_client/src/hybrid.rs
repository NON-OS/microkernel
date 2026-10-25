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

//! A hybrid signature check: Ed25519 and ML-DSA-65 over the same message,
//! and both must verify. Both halves are always checked, and a failure
//! names the half that failed.

use nonos_ed25519::Signature;
use nsh_wire::keys::{ED_PUBLIC_LEN, ML_PUBLIC_LEN};
use nsh_wire::HybridSig;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Half {
    Ed25519,
    MlDsa65,
}

pub(crate) fn verify(
    ed: &[u8; ED_PUBLIC_LEN],
    ml: &[u8; ML_PUBLIC_LEN],
    msg: &[u8],
    sig: &HybridSig<'_>,
) -> Result<(), Half> {
    let ed_ok = nonos_ed25519::verify(ed, msg, &Signature::from_bytes(sig.ed));
    let ml_ok = nonos_mldsa65::verify(ml, msg, sig.ml);
    match (ed_ok, ml_ok) {
        (true, true) => Ok(()),
        (false, _) => Err(Half::Ed25519),
        (true, false) => Err(Half::MlDsa65),
    }
}
