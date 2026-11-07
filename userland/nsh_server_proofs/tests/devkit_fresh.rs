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

//! The devkit's keys are its own. In a process whose key space comes from
//! the OS generator, as nsh-devkit's does, no authority, role-table or user
//! key is the proofs' fixed one, the client files name exactly the keys the
//! machine's records were made with, and a host built from the records and
//! keys of its own admits the real client holding them. A second space
//! cannot be set. This runs in its own process, so the proofs' fixed keys
//! are untouched.

use nonos_ed25519::KeyPair;
use nonos_hash::sha256;
use nsh_server::{Config, RoleStore};
use nsh_server_proofs::kit::attest_build::keys::{hybrid, reseed, AUTHORITY, HOST};
use nsh_server_proofs::kit::devkit::fresh::fresh;
use nsh_server_proofs::kit::devkit::{client, enrolment, records, Fixture, Machine, Window};
use nsh_server_proofs::kit::host::{config, Kernel, Keys};
use nsh_server_proofs::kit::roles::Table;
use nsh_server_proofs::kit::roles_build::keys::{key, K};
use nsh_server_proofs::kit::roles_build::NOW;
use nsh_server_proofs::kit::run::Run;
use std::cell::Cell;
use std::rc::Rc;

fn get<'a>(files: &'a [Fixture], name: &str) -> &'a [u8] {
    &files.iter().find(|f| f.name == name).expect(name).bytes
}

/// The proofs' fixed Ed25519 public half for a builder seed.
fn fixed_ed(seed: &[u8]) -> [u8; 32] {
    KeyPair::from_seed(sha256(seed)).public
}

#[test]
fn the_devkits_keys_are_fresh_and_pair_with_each_other() {
    assert!(fresh(), "a space from the OS generator");
    assert!(!reseed([0; 32]), "no second space");
    assert_ne!(hybrid(HOST).ed.public, fixed_ed(&[b'e', b'd', HOST]));
    assert_ne!(hybrid(AUTHORITY).ed.public, fixed_ed(&[b'e', b'd', AUTHORITY]));
    for k in [K::AuthA, K::AuthB, K::User] {
        assert_ne!(
            key(k).ed.public,
            fixed_ed(&[b'n', b's', b'h', b'-', b'r', b'o', b'l', b'e', k as u8])
        );
    }

    let w = Window::at(NOW).expect("a window");
    let h = hybrid(HOST);
    let public: [u8; 1984] =
        [&h.ed.public[..], &h.ml_public[..]].concat().try_into().expect("keys");
    let (mut hosts, clients) = (records(&w), client(&w));
    hosts.push(enrolment(&w, &Machine::fixture(), &public));
    assert!(hosts.iter().all(|f| !f.secret), "no host secret among the machine's records");
    let a = hybrid(AUTHORITY);
    assert_eq!(
        get(&clients, "authority.public"),
        &[&a.ed.public[..], &a.ml_public[..]].concat()[..]
    );
    assert_eq!(&get(&clients, "user.secret")[8..40], &key(K::User).ed.private);

    let anchor = get(&hosts, "roles.anchor").try_into().expect("an anchor");
    let store = RoleStore::new(
        get(&hosts, "roles").to_vec(),
        anchor,
        0,
        get(&hosts, "revocations").to_vec(),
    );
    let table = Table { store: Rc::new(store.expect("the store")), fail: Rc::new(Cell::new(None)) };
    let cfg = Config { enrol: get(&hosts, "enrol").to_vec(), ..config() };
    let grant = nsh_server_proofs::kit::devkit::make::grant(&w);
    let mut run = Run::with(&grant, cfg, Kernel::honest(), Keys::default(), table);
    run.pump();
    assert!(
        run.c_events.iter().any(|e| matches!(e, nsh_client::Event::LoggedIn(_))),
        "{:?}",
        run.c_err
    );
}
