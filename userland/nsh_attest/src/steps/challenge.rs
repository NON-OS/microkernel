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

//! Step 2: the document answers this session's challenge,
//! `SHA-256("nsh/1 attest\0" || TH(SERVER_HELLO) || H(host keys))`. The
//! challenge depends on `client_random` and names the host keys, so a
//! document made for another session or another key is refused.

use nsh_crypto::attest_challenge;

use crate::{Doc, Evidence, Refusal, Session};

pub fn challenge(doc: &Doc<'_>, ev: &Evidence<'_>, s: &Session<'_>) -> Result<(), Refusal> {
    let want = attest_challenge(s.th_server_hello, ev.host_ed, ev.host_ml);
    match *doc.challenge == want {
        true => Ok(()),
        false => Err(Refusal::Challenge),
    }
}
