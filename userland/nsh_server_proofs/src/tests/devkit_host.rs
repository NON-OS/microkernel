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

//! A development machine as nsh.front and nsh.attest open it: the host keys
//! the machine made (the spec host's, here), the enrolment the devkit signs
//! for those keys, and the machine's role table vouched for by the server's
//! own `RoleStore`, read from the records the devkit wrote.

use crate::kit::attest_build::keys::{hybrid, HOST};
use crate::kit::devkit::{enrolment, records, Fixture, Machine, Window, HOST_PUBLIC_LEN};
use nonos_mldsa65::SecretKey;
use nsh_crypto::Secret;
use nsh_server::{Config, HostKey, RoleStore};

/// November 2026: a clock the spec's windows open at.
pub const AT_SPEC: u64 = 1_793_000_000_000;

pub fn file<'a>(files: &'a [Fixture], name: &str) -> &'a [u8] {
    &files.iter().find(|f| f.name == name).expect(name).bytes
}

/// The host keys the machine made, as nsh.attest holds them.
pub fn keys() -> HostKey {
    let h = hybrid(HOST);
    let mut seed = h.ed.private;
    HostKey::from_parts(
        Secret::take(&mut seed),
        SecretKey::from_bytes(h.ml_secret.as_bytes()),
        &h.ml_public,
    )
}

/// The machine's host.public.
pub fn public() -> [u8; HOST_PUBLIC_LEN] {
    let h = hybrid(HOST);
    [&h.ed.public[..], &h.ml_public[..]].concat().try_into().expect("host.public")
}

pub fn config(enrol: &[u8]) -> Config {
    Config {
        enrol: enrol.to_vec(),
        handshake_ms: 30_000,
        idle_s: 600,
        ckpt_every: 0,
        max_record: 1 << 20,
    }
}

pub fn roles(files: &[Fixture]) -> RoleStore {
    let anchor = file(files, "roles.anchor").try_into().expect("an anchor");
    let table = file(files, "roles").to_vec();
    RoleStore::new(table, anchor, 0, file(files, "revocations").to_vec()).expect("the store")
}

/// The machine's records made at `now_ms`, its enrolment last.
pub fn made_at(now_ms: u64) -> (Window, Vec<Fixture>) {
    let w = Window::at(now_ms).expect("a window");
    let mut files = records(&w);
    files.push(enrolment(&w, &Machine::fixture(), &public()));
    (w, files)
}
