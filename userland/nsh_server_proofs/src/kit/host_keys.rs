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

//! The host keys of the spec, which sign with both halves unless a proof
//! makes them fail.

use super::attest_build::keys::{hybrid, HOST};
use nsh_server::{HostSig, HostSigner, Rand};
use nsh_wire::keys::{ED_PUBLIC_LEN, ML_PUBLIC_LEN};
use std::cell::Cell;
use std::rc::Rc;

/// The host keys of the spec, which sign with both halves.
#[derive(Clone, Default)]
pub struct Keys {
    /// Refuse to sign while set: the host keys failing.
    pub refuse: Rc<Cell<bool>>,
}

impl HostSigner for Keys {
    fn ed_public(&self) -> &[u8; ED_PUBLIC_LEN] {
        &hybrid(HOST).ed.public
    }
    fn ml_public(&self) -> &[u8; ML_PUBLIC_LEN] {
        &hybrid(HOST).ml_public
    }
    fn sign(&self, digest: &[u8; 64], _: Rand<'_>) -> Option<HostSig> {
        if self.refuse.get() {
            return None;
        }
        let sig = hybrid(HOST).sign(digest);
        let (ed, ml) = sig.split_at(64);
        Some(HostSig { ed: ed.try_into().ok()?, ml: Box::new(ml.try_into().ok()?) })
    }
}
