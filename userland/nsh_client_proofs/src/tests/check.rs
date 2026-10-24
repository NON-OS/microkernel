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

//! The checks many proofs share: what the client sent after a refusal, and
//! a run that differs from the good one only in its `HOST_PROOF`.

use crate::kit::attest::TestAttester;
use crate::kit::pair::{config, Pair};
use crate::kit::proof::ProofSpec;
use nsh_client::{Config, Refusal};
use nsh_wire::{Kind, Reason};

/// The client sent exactly one frame, `ABORT(reason)`, and then nothing.
pub fn aborted_with(p: &mut Pair, reason: Reason) {
    let sent = p.sent();
    assert_eq!(sent, vec![(Kind::Abort.byte(), reason.code().to_le_bytes().to_vec())]);
    assert!(p.client.is_over());
}

/// The client sent nothing at all, and the session is over.
pub fn silent(p: &mut Pair) {
    assert!(p.client.take_output().is_empty(), "nothing is sent");
    assert!(p.client.is_over());
}

/// A good run but for `spec`, with `cfg` and `attester`: the client's
/// refusal of the host. Whatever it is, the only frame sent after the
/// hello is `ABORT(POLICY)`: never a credential.
pub fn host_run(spec: &ProofSpec, cfg: Config, attester: TestAttester) -> (Refusal, Pair) {
    let mut p = Pair::with(cfg, attester);
    p.to_proof();
    let hp = p.proof(spec);
    let r = p.feed(&hp).expect_err("the host is refused");
    if let Refusal::Host(_) = r {
        aborted_with(&mut p, Reason::Policy);
    }
    (r, p)
}

pub fn host_refusal(spec: &ProofSpec) -> Refusal {
    host_run(spec, config(), TestAttester::default()).0
}

pub fn logged_in() -> Pair {
    let mut p = Pair::good();
    p.login();
    p
}

/// The server seals one frame and the client takes it.
pub fn gets(p: &mut Pair, kind: Kind, body: &[u8]) -> Result<(), Refusal> {
    let f = p.server.seal(kind, body);
    p.feed(&f)
}
