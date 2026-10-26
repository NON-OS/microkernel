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
//! Any bytes as an AgentProof receipt: read or refused, then verified or
//! refused, and never a panic. The trust names an all-zero AK, which no
//! quote's key is, so nothing may verify.

#![no_main]

use agentproof_receipt::napr::read::parse;
use agentproof_receipt::trust::{Release, Supervisor, Trust};
use agentproof_receipt::verify;
use libfuzzer_sys::fuzz_target;

fuzz_target!(|data: &[u8]| {
    let _ = parse(data);
    let trust = Trust {
        releases: vec![Release { version: "x".into(), pcr8: [1; 32], pcr9: [1; 32], pcr14: [1; 32] }],
        supervisors: vec![Supervisor { meas: [0; 32], caps: 0 }],
        ak: [0; 64],
    };
    assert!(verify(data, &trust).is_err());
});
