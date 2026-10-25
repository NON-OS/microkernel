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

//! Steps 2 to 4 of checking a record: the document given is the one the
//! header names, and it passes the client's checks for the challenge the
//! header's transcript and host keys make; the enrolment in the header is
//! the fleet authority's, for the document's AK and these host keys.

use super::RecordRefusal;
use crate::ct::eq;
use crate::host_check::{self, Claim};
use crate::{Attester, Policy};
use nonos_hash::sha512;
use nsh_wire::RecordHeader;

pub(crate) fn check<A: Attester + ?Sized>(
    h: &RecordHeader<'_>,
    doc: &[u8],
    policy: &Policy,
    attester: &A,
) -> Result<(), RecordRefusal> {
    if !eq(&sha512(doc), h.doc_hash) {
        return Err(RecordRefusal::DocHash);
    }
    let claim =
        Claim { host: h.host, enrol: &h.enrol, doc, entries: None, th_server_hello: h.transcript };
    host_check::check(&claim, policy, None, attester).map_err(RecordRefusal::Host)
}
