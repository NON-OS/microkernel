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

//! init's hold on the release it was given a pin for (I-C5, op 8 v2).

use nonos_market_proto::{linux_policy_body, LinuxPolicy};

use super::kernel_linux_policy::{held_to, parse, LinuxPolicyReply};

const HASH: [u8; 32] = [5; 32];

/* init holds the answer to the pin get_release gave it: an answer made from
 * another release, as when the index was replaced between asks, is none. */
#[test]
fn an_answer_for_another_release_is_never_taken() {
    let body = linux_policy_body(&LinuxPolicy::NoPolicy, &HASH);
    let answer = parse(&body).expect("reads");
    assert_eq!(held_to(answer.clone(), &HASH), Some(LinuxPolicyReply::NoPolicy));
    assert_eq!(held_to(answer, &[6; 32]), None);
}
