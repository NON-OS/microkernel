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

//! What the client checks, and the session it checks it for. Evidence is
//! made only from a `HOST_PROOF` whose host signatures verify, so no check
//! runs over parts nobody has shown the host keys signed.

use nsh_wire::keys::{ED_PUBLIC_LEN, ML_PUBLIC_LEN};

/// The parts of a verified `HOST_PROOF` the checks read, as raw bytes.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Evidence<'a> {
    pub(crate) host_ed: &'a [u8; ED_PUBLIC_LEN],
    pub(crate) host_ml: &'a [u8; ML_PUBLIC_LEN],
    /// The enrolment certificate, whole: its digest is what a revocation
    /// names.
    pub(crate) enrol: &'a [u8],
    pub(crate) doc: &'a [u8],
    pub(crate) entries: &'a [u8],
}

/// This connection: `TH(SERVER_HELLO)`, the host name the user typed, and
/// the client's clock.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Session<'a> {
    pub th_server_hello: &'a [u8; 64],
    pub host: &'a [u8],
    pub now_ms: u64,
}

impl<'a> Evidence<'a> {
    pub fn host_keys(&self) -> (&'a [u8; ED_PUBLIC_LEN], &'a [u8; ML_PUBLIC_LEN]) {
        (self.host_ed, self.host_ml)
    }

    pub fn enrol(&self) -> &'a [u8] {
        self.enrol
    }

    pub fn doc(&self) -> &'a [u8] {
        self.doc
    }

    pub fn entries(&self) -> &'a [u8] {
        self.entries
    }
}
