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

//! The host's side of a test: the user's `USER_AUTH` for a grant, and a
//! login checked by `nsh_roles` against the table and list in a directory.

use nsh_crypto::{labelled, Label};
use nsh_roles::{login, open_policy, open_revocations, Host, Login, Refusal};
use nsh_wire::UserAuth;
use std::path::Path;

const TH: [u8; 64] = [0x5A; 64];

pub fn read(d: &Path, f: &str) -> Vec<u8> {
    std::fs::read(d.join(f)).expect(f)
}

/// The user's `USER_AUTH` for `grant`, signed over `TH`.
pub fn user_auth(d: &Path, grant: &[u8]) -> Vec<u8> {
    let key = nsh_enrol::AuthorityKey::from_file(&read(d, "user.secret")).expect("user key");
    let mut body = key.public();
    body.extend_from_slice(&(grant.len() as u16).to_le_bytes());
    body.extend_from_slice(grant);
    body.extend_from_slice(&key.sign(&labelled(Label::UserSig, &TH)).expect("signed"));
    body
}

pub fn host_login(d: &Path, anchor: &[u8; 32], list: &str) -> Result<Login, Refusal> {
    let (tb, lb) = (read(d, "table"), read(d, list));
    let p = open_policy(&tb, anchor, 0).expect("the table opens");
    let r = open_revocations(&lb, &p, 0).expect("the list opens");
    let now = nsh_enrol::files::now_ms().expect("clock");
    let ua = user_auth(d, &read(d, "base"));
    login(&Host::new(&p, &r, now).expect("host"), &UserAuth::parse(&ua).expect("parses"), &TH)
}
