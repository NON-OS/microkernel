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

//! The files, named as their readers name them: the machine's records as
//! nsh.attest reads them under /nonos/nsh (`roles`, `roles.anchor`,
//! `revocations`, `enrol`), the client's as an operator hands them to `nsh`
//! (`--key`, `--authority`, `--grant`). No host key is among them: the
//! machine makes its own at its first boot, and the enrolment names the
//! public keys it wrote to /nonos/nsh/host.public.

use super::machine::Machine;
use super::make::{enrol_for, grant, revocations, table};
use super::window::Window;
use crate::kit::attest_build::keys::{hybrid, AUTHORITY};
use crate::kit::roles_build::keys::{key, K};

/// The header of nsh-enrol's secret key file, version 1.
pub const KEY_FILE: &[u8; 8] = b"NSHA\x01\x00\x00\x00";

pub struct Fixture {
    pub name: &'static str,
    pub bytes: Vec<u8>,
    /// Written readable by its owner alone.
    pub secret: bool,
}

fn public(name: &'static str, bytes: Vec<u8>) -> Fixture {
    Fixture { name, bytes, secret: false }
}

fn secret(name: &'static str, bytes: Vec<u8>) -> Fixture {
    Fixture { name, bytes, secret: true }
}

/// The machine's role table, its anchor and its revocation list.
pub fn records(w: &Window) -> Vec<Fixture> {
    let t = table(w);
    vec![
        public("roles", t.bytes()),
        public("roles.anchor", t.anchor.to_vec()),
        public("revocations", revocations(w).bytes()),
    ]
}

/// The machine's enrolment: machine `m`, its host keys `public` (the
/// Ed25519 key, then the ML-DSA-65 key, as host.public holds them).
pub fn enrolment(w: &Window, m: &Machine, public: &[u8; HOST_PUBLIC_LEN]) -> Fixture {
    self::public("enrol", enrol_for(w, m, public))
}

/// host.public's length.
pub const HOST_PUBLIC_LEN: usize = 32 + 1952;

/// What the user's `nsh` reads.
pub fn client(w: &Window) -> Vec<Fixture> {
    let (a, u) = (hybrid(AUTHORITY), key(K::User));
    vec![
        secret(
            "user.secret",
            [&KEY_FILE[..], &u.ed.private, &u.ml_pk, u.ml_sk.as_bytes()].concat(),
        ),
        public("authority.public", [&a.ed.public[..], &a.ml_public[..]].concat()),
        public("grant", grant(w).bytes()),
    ]
}
