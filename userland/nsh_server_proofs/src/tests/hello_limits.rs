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

//! `CLIENT_HELLO` and the host's proof: only `VERSION` and `BUSY` can be
//! told before keys exist; everything else broken there is closed without
//! a word. A host that cannot attest or sign says `BUSY` or nothing, and
//! never sends a proof the client would refuse.

use crate::kit::host::{config, Kernel, Keys};
use crate::kit::roles::Table;
use crate::kit::roles_build::grant::GrantB;
use crate::kit::run::Run;
use nsh_client::Refusal;
use nsh_server::{AttestFault, Why};
use nsh_wire::Reason;

#[test]
fn a_document_no_client_could_read_is_never_sent() {
    for doc in [vec![], vec![0u8; nsh_wire::MAX_DOC + 1]] {
        let kernel = Kernel { doc: Some(doc), ..Kernel::honest() };
        let mut run =
            Run::with(&GrantB::default(), config(), kernel, Keys::default(), Table::shipped());
        run.pump();
        assert_eq!(run.refused(), [Why::Attest(AttestFault::Malformed)]);
        assert_eq!(run.c_err, Some(Refusal::Aborted(Reason::Busy)));
    }
}
