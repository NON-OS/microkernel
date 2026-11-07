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

//! nsh.front's and nsh.attest's pure modules, mounted from the capsules'
//! sources.

extern crate alloc;

// nsh.attest's reply frames, the other end of the relay.
#[path = "../../capsule_nsh_attest/src/reply.rs"]
pub mod attest_reply;
// nsh.attest's serial lines, its enrolment challenge, and the host keys'
// sealed layout and label.
#[path = "../../capsule_nsh_front/src/accepted.rs"]
pub mod accepted;
#[path = "../../capsule_nsh_attest/src/anchor.rs"]
pub mod anchor;
#[path = "../../capsule_nsh_attest/src/challenge.rs"]
pub mod challenge;
#[path = "../../capsule_nsh_attest/src/keys/label.rs"]
pub mod key_label;
#[path = "../../capsule_nsh_attest/src/keys/layout.rs"]
pub mod key_layout;
#[path = "../../capsule_nsh_front/src/line.rs"]
pub mod line;
#[path = "../../capsule_nsh_front/src/marks.rs"]
pub mod marks;
#[path = "../../capsule_nsh_front/src/net_rule.rs"]
pub mod net_rule;
#[path = "../../capsule_nsh_front/src/read.rs"]
pub mod read;
#[path = "../../capsule_nsh_front/src/refused_said.rs"]
pub mod refused_said;
#[path = "../../capsule_nsh_front/src/relay_said.rs"]
pub mod relay_said;
#[path = "../../capsule_nsh_front/src/reply.rs"]
pub mod reply;
#[path = "../../capsule_nsh_attest/src/said.rs"]
pub mod said;
#[path = "../../capsule_nsh_front/src/turn_away.rs"]
pub mod turn_away;
#[path = "../../capsule_nsh_front/src/unready.rs"]
pub mod unready;

#[cfg(test)]
mod tests;
