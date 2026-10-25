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

//! The user's key signs only for an accepted host; a failed signature
//! ends the session without a frame.

use crate::kit::{attest::TestAttester, keys, pair::config, proof::ProofSpec, server::Server};
use nsh_client::{Client, Event, Rand, Refusal, UserSig, UserSigner};
use nsh_crypto::Secret;
use std::cell::Cell;
use std::rc::Rc;

struct Counting {
    asked: Rc<Cell<u32>>,
    works: bool,
}

impl UserSigner for Counting {
    fn ed_public(&self) -> &[u8; 32] {
        &keys::user().ed.public
    }

    fn ml_public(&self) -> &[u8; 1952] {
        &keys::user().ml_public
    }

    fn sign(&self, digest: &[u8; 64], rand: Rand<'_>) -> Option<UserSig> {
        self.asked.set(self.asked.get() + 1);
        self.works.then(|| keys::user_key().sign(digest, rand)).flatten()
    }
}

fn run(works: bool, spec: &ProofSpec) -> (Result<(), Refusal>, u32, Vec<u8>, Vec<Event>) {
    let asked = Rc::new(Cell::new(0));
    let signer = Counting { asked: asked.clone(), works };
    let mut c =
        Client::new(config(), TestAttester::default(), signer, Secret::new([3; 32]), [4; 32])
            .unwrap();
    let mut s = Server::default();
    let sh = s.hello(&c.take_output(), |_| {});
    let mut d = crate::kit::rng::Det::new(7);
    let mut rand = |b: &mut [u8]| d.fill(b);
    c.feed(&sh, &mut rand).unwrap();
    let body = spec.bytes(&s);
    let r = c.feed(&s.seal(nsh_wire::Kind::HostProof, &body), &mut rand);
    (r, asked.get(), c.take_output(), c.events())
}

#[test]
fn the_signer_is_asked_only_after_the_host_is_accepted() {
    let bad = ProofSpec { sig: crate::kit::enrol::SigMode::BadEd, ..ProofSpec::good() };
    let (r, asked, out, _) = run(true, &bad);
    assert!(matches!(r, Err(Refusal::Host(_))));
    assert_eq!(asked, 0, "a refused host is never signed for");
    assert_eq!(out.len(), 4 + 3 + 16, "only ABORT(POLICY) goes out");
    let (r, asked, _, events) = run(true, &ProofSpec::good());
    assert_eq!((r, asked, events), (Ok(()), 1, vec![Event::HostAccepted]));
    let (r, asked, out, events) = run(false, &ProofSpec::good());
    assert_eq!((r, asked), (Err(Refusal::Sign), 1));
    assert!(out.is_empty() && events.is_empty(), "a failed signature sends and says nothing");
}
