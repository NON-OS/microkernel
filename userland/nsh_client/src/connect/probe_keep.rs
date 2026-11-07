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

//! The probe's attester: it keeps the document and the registry entries it
//! is handed and refuses them, so the client aborts before USER_AUTH and no
//! credential leaves. It checks nothing; what it keeps is for an operator
//! to read, never to trust.

use crate::{AttestRefusal, Attester, Quoted};
use alloc::vec::Vec;
use core::cell::RefCell;
use nsh_wire::Enrol;

#[derive(Default)]
pub struct Keep {
    pub seen: RefCell<Option<(Vec<u8>, Vec<u8>)>>,
}

impl Attester for Keep {
    fn check(
        &self,
        doc: &[u8],
        _: &[u8; 32],
        entries: Option<&[u8]>,
        _: &Enrol<'_>,
    ) -> Result<Quoted, AttestRefusal> {
        *self.seen.borrow_mut() = Some((doc.to_vec(), entries.unwrap_or(&[]).to_vec()));
        Err(AttestRefusal::Unavailable)
    }
}
