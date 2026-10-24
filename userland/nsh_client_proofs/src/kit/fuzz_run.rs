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

//! One handshake whose `HOST_PROOF` is built from fuzzed fields: what the
//! oracle says should pass, and whether the client sent `USER_AUTH`.

use super::fuzz_oracle::Oracle;
use super::fuzz_proof::fuzzed;
use super::pair::Pair;
use nsh_wire::Kind;

pub struct HostRun {
    pub oracle: Oracle,
    /// The client sent `USER_AUTH`.
    pub credential: bool,
    pub pair: Pair,
}

pub fn host_run(b: &[u8]) -> HostRun {
    let mut pair = Pair::good();
    pair.to_proof();
    let (spec, oracle) = fuzzed(b, &pair.server);
    let wire = pair.proof(&spec);
    let _ = pair.feed(&wire);
    let out = pair.client.take_output();
    let frames = pair.server.open(&out).unwrap_or_default();
    let credential = frames.iter().any(|f| f.0 == Kind::UserAuth.byte());
    HostRun { oracle, credential, pair }
}
