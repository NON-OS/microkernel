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

//! Sealed handshake frames that are changed, or replayed out of their
//! order: none opens, none is answered, and no credential follows.

use super::silent;
use crate::kit::pair::Pair;
use crate::kit::proof::ProofSpec;
use crate::kit::server::Server;
use nsh_client::Refusal;
use nsh_wire::Kind;

fn at_proof(frame: impl FnOnce(&mut Server) -> Vec<u8>) -> (Result<(), Refusal>, Pair) {
    let mut p = Pair::good();
    p.to_proof();
    let f = frame(&mut p.server);
    (p.feed(&f), p)
}

#[test]
fn a_changed_or_replayed_host_proof_does_not_open_and_is_not_answered() {
    let sealed = |s: &mut Server| s.seal(Kind::HostProof, &ProofSpec::good().bytes(s));
    for at in [4usize, 5, 100, 2000, 5000, usize::MAX] {
        let (r, mut p) = at_proof(|s| {
            let mut f = sealed(s);
            let i = at.min(f.len() - 1);
            f[i] ^= 0x01;
            f
        });
        assert_eq!(r, Err(Refusal::Open), "byte {at}");
        silent(&mut p);
        assert_eq!(p.client.attester().calls.get(), 0);
    }
    let (r, mut p) = at_proof(|s| {
        let mut f = sealed(s);
        let n = u32::from_le_bytes(f[..4].try_into().unwrap()) - 1;
        f[..4].copy_from_slice(&n.to_le_bytes());
        f
    });
    assert_eq!(r, Err(Refusal::Open), "the length is bound to the frame");
    silent(&mut p);
    let (r, mut p) = at_proof(|s| {
        let _first = sealed(s);
        sealed(s)
    });
    assert_eq!(r, Err(Refusal::Open), "a frame sealed second cannot be opened first");
    silent(&mut p);
}
