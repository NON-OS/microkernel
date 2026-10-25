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

//! A HOST_PROOF body changed after it was signed, wherever the change
//! is: refused, and answered with ABORT alone, never a credential.

use crate::kit::pair::Pair;
use crate::kit::proof::ProofSpec;
use nsh_client::Refusal;
use nsh_wire::Kind;

#[test]
fn a_body_changed_after_signing_is_refused_wherever_it_changed() {
    let len = ProofSpec::good().bytes(&Pair::good().server).len();
    for at in (0..len).step_by(997).chain([31, 32, 1983, 1984, 1986, len - 3373, len - 1]) {
        let mut p = Pair::good();
        p.to_proof();
        let mut body = ProofSpec::good().bytes(&p.server);
        body[at] ^= 0x04;
        let f = p.server.seal(Kind::HostProof, &body);
        let r = p.feed(&f).expect_err("a changed HOST_PROOF is refused");
        let sent = p.sent();
        assert_eq!(sent.len(), 1, "byte {at}: {r:?}");
        assert_eq!(sent[0].0, Kind::Abort.byte(), "byte {at}: no credential");
        assert!(matches!(r, Refusal::Host(_) | Refusal::Wire(_)), "byte {at}: {r:?}");
    }
}
